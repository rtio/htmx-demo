use std::path::{Path, PathBuf};

use libvips::{ops, VipsImage};

pub fn resize_from_format(id: PathBuf, max_width_size: i32, ext: &str) -> String {
    match ext {
        "jpeg" => resize_jpeg(id, max_width_size),
        "webp" => resize_webp(id, max_width_size),
        "png" => resize_png(id, max_width_size),
        &_ => todo!(),
    }
}

pub fn resize_jpeg(id: PathBuf, max_width_size: i32) -> String {
    println!("Resizing image: {:?} - {}", id, max_width_size);
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let original_file_name = Path::new(upload_dir).join(id.clone()).join("original");
    let new_file_name = Path::new(upload_dir)
        .join(id)
        .join(max_width_size.to_string())
        .with_extension("jpeg");
    let new_image = VipsImage::new_from_file(original_file_name.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::jpegsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}

pub fn resize_webp(id: PathBuf, max_width_size: i32) -> String {
    println!("Resizing image: {:?} - {}", id, max_width_size);
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let original_file_name = Path::new(upload_dir).join(id.clone()).join("original");
    let new_file_name = Path::new(upload_dir)
        .join(id)
        .join(max_width_size.to_string())
        .with_extension("webp");
    let new_image = VipsImage::new_from_file(original_file_name.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::webpsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}

pub fn resize_png(id: PathBuf, max_width_size: i32) -> String {
    println!("Resizing image: {:?} - {}", id, max_width_size);
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let original_file_name = Path::new(upload_dir).join(id.clone()).join("original");
    let new_file_name = Path::new(upload_dir)
        .join(id)
        .join(max_width_size.to_string())
        .with_extension("png");
    let new_image = VipsImage::new_from_file(original_file_name.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::pngsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}
