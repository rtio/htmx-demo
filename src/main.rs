#[macro_use]
extern crate rocket;

mod image;
mod image_id;

use image::resize_from_format;
use image_id::ImageId;
use rocket::{
    form::Form,
    fs::{NamedFile, TempFile},
    http::ContentType,
    response::Redirect,
    tokio::fs::File,
    Request,
};
use rocket_dyn_templates::{context, Template};
use std::{fmt, fs, io, path::Path};

// In a real application, these would be retrieved dynamically from a config.
const ID_LENGTH: usize = 5;
const SIZES: [i32; 8] = [1920, 1280, 1024, 768, 640, 480, 320, 240];

#[derive(Debug, FromFormField)]
enum OutputFormat {
    Png,
    Jpeg,
    Webp,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Jpeg => write!(f, "jpeg"),
            OutputFormat::Webp => write!(f, "webp"),
        }
    }
}

#[derive(FromForm)]
struct Upload<'r> {
    output_format: OutputFormat,
    image: TempFile<'r>,
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Image Resizer"
        },
    )
}

#[post("/upload", data = "<upload>")]
async fn upload(mut upload: Form<Upload<'_>>) -> io::Result<Redirect> {
    let id = ImageId::new(ID_LENGTH);
    fs::create_dir_all(id.file_path())?;

    let permanent_location = id.file_path().join("original");
    upload.image.persist_to(permanent_location).await?;

    println!("Image uploaded: {:?}", id.file_path());

    Ok(Redirect::to(uri!(resize_image(
        id,
        upload.output_format.to_string()
    ))))
}

#[get("/resize/<id>/<ext>")]
fn resize_image(id: ImageId<'_>, ext: &str) -> Template {
    let mut results = vec![];
    for size in SIZES.iter() {
        results.push(context! {
            maxwidth: size,
            src: uri!(retrieve(&id, size, ext)),
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

#[get("/resized/<id>")]
async fn retrieve_original(id: ImageId<'_>) -> (ContentType, Option<File>) {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir).join(id.file_path()).join("original");
    return (ContentType::PNG, File::open(&filename).await.ok());
}

#[get("/resized/<id>/<maxwidth>/<ext>")]
async fn retrieve(id: ImageId<'_>, maxwidth: i32, ext: &str) -> Option<NamedFile> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir)
        .join(id.file_path())
        .join(maxwidth.to_string())
        .with_extension(ext);
    match NamedFile::open(filename).await {
        Ok(f) => Some(f),
        Err(_) => {
            let new_file_name = resize_from_format(id.file_path(), maxwidth, ext);
            NamedFile::open(&new_file_name).await.ok()
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
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
