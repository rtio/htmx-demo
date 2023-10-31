use std::path::{Path, PathBuf};

use libvips::{ops, VipsImage};

fn get_dirs(id: PathBuf, max_width_size: i32, ext: &str) -> (PathBuf, PathBuf) {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let original_file_name = Path::new(upload_dir).join(id.clone()).join("original");
    let new_file_name = Path::new(upload_dir)
        .join(id)
        .join(max_width_size.to_string())
        .with_extension(ext);
    (original_file_name, new_file_name)
}

pub fn resize_from_format(id: PathBuf, max_width_size: i32, ext: &str) -> String {
    println!("Resizing image: {:?} - {}", id, max_width_size);
    match ext {
        "jpeg" => resize_jpeg(id, max_width_size),
        "webp" => resize_webp(id, max_width_size),
        "png" => resize_png(id, max_width_size),
        "gif" => resize_gif(id, max_width_size),
        &_ => todo!(),
    }
}

fn resize_jpeg(id: PathBuf, max_width_size: i32) -> String {
    let (original_file_name, new_file_name) = get_dirs(id.clone(), max_width_size, "jpeg");
    let new_image = VipsImage::new_from_file(original_file_name.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::jpegsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}

fn resize_webp(id: PathBuf, max_width_size: i32) -> String {
    let (original_file_name, new_file_name) = get_dirs(id.clone(), max_width_size, "webp");

    let f_name = original_file_name.to_str().unwrap();
    let sufix = "[n=-1]";
    let file_with_sufix = Path::new(f_name).with_file_name(format!("{}{}", f_name, sufix));
    let new_image = VipsImage::new_from_file(file_with_sufix.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::webpsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}

fn resize_png(id: PathBuf, max_width_size: i32) -> String {
    let (original_file_name, new_file_name) = get_dirs(id.clone(), max_width_size, "png");
    let new_image = VipsImage::new_from_file(original_file_name.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::pngsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}

fn resize_gif(id: PathBuf, max_width_size: i32) -> String {
    let (original_file_name, new_file_name) = get_dirs(id.clone(), max_width_size, "gif");

    let f_name = original_file_name.to_str().unwrap();
    let sufix = "[n=-1]";
    let file_with_sufix = Path::new(f_name).with_file_name(format!("{}{}", f_name, sufix));
    let new_image = VipsImage::new_from_file(file_with_sufix.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::gifsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}
