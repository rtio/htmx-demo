#[macro_use]
extern crate rocket;

mod image;
mod image_id;

use image::resize_png;
use image_id::ImageId;
use rocket::http::uri::Absolute;
use rocket::{
    form::Form,
    fs::{relative, FileServer, NamedFile, TempFile},
    http::ContentType,
    response::Redirect,
    tokio::fs::File,
    Request,
};
use rocket_dyn_templates::{context, Template};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

// In a real application, these would be retrieved dynamically from a config.
const ID_LENGTH: usize = 5;
const HOST: Absolute<'static> = uri!("http://localhost:8000");
const SIZES: [i32; 8] = [1920, 1280, 1024, 768, 640, 480, 320, 240];

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Image Resizer"
        },
    )
}

#[post("/upload", data = "<image>")]
async fn upload(mut image: Form<TempFile<'_>>) -> io::Result<Redirect> {
    let id = ImageId::new(ID_LENGTH);
    let permanent_location = id.file_path().join("original");
    fs::create_dir_all(id.file_path())?;
    println!("Permanent location: {:?}", permanent_location);
    image.persist_to(permanent_location).await?;
    println!("Image uploaded: {:?}", id.file_path());

    Ok(Redirect::to(uri!(resize_image(id))))
}

#[get("/resize/<id>")]
fn resize_image(id: ImageId<'_>) -> Template {
    let mut results = vec![];
    for size in SIZES.iter() {
        results.push(context! {
            maxwidth: size,
            src: uri!(retrieve(&id, size, "png")),
        });
    }
    Template::render(
        "result",
        context! {
            title: "Resized Image",
            images: results
        },
    )
}

#[get("/resize/<id>/<ext>")]
async fn retrieve_original(id: ImageId<'_>, ext: &str) -> (ContentType, Option<File>) {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir).join(id.file_path()).join("original");
    return (ContentType::PNG, File::open(&filename).await.ok());
}

#[get("/resize/<id>/<maxwidth>/<ext>")]
async fn retrieve(id: ImageId<'_>, maxwidth: i32, ext: &str) -> (ContentType, Option<File>) {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir)
        .join(id.file_path())
        .join(maxwidth.to_string());
    match File::open(&filename).await {
        Ok(f) => (ContentType::PNG, Some(f)),
        Err(_) => {
            let new_file_name = resize_png(id.file_path(), maxwidth);
            return (ContentType::PNG, File::open(&new_file_name).await.ok());
        }
    }
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[launch]
fn rocket() -> _ {
    // let app = VipsApp::new("IMAGE_RESIZER", false).expect("Cannot initialize libvips");
    rocket::build()
        .mount(
            "/",
            routes![index, upload, retrieve, resize_image, retrieve_original],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
