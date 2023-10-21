use std::path::{Path, PathBuf};

use libvips::{VipsImage, ops};

pub fn resize_png(id: PathBuf, max_width_size: i32) -> String {
    println!("Resizing image: {:?} - {}", id, max_width_size);
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let original_file_name = Path::new(upload_dir)
        .join(id.clone())
        .join("original");
    let new_file_name = Path::new(upload_dir)
        .join(id)
        .join(max_width_size.to_string());
    let new_image = VipsImage::new_from_file(original_file_name.to_str().unwrap()).unwrap();
    let resized = ops::thumbnail_image(&new_image, max_width_size).unwrap();
    ops::pngsave(&resized, new_file_name.to_str().unwrap()).unwrap();
    return String::from(new_file_name.to_str().unwrap());
}