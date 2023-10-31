use std::path::{Path, PathBuf};

use libvips::{ops, VipsImage};
use libvips::Result;

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

pub fn process_image_from_buffer(buffer: &[u8], max_width_size: i32, ext: &str) -> Result<Vec<u8>> {
    match ext {
        "jpeg" => resize_jpeg_from_buffer(buffer, max_width_size),
        "webp" => resize_webp_from_buffer(buffer, max_width_size),
        "png" => resize_png_from_buffer(buffer, max_width_size),
        "gif" => resize_gif_from_buffer(buffer, max_width_size),
        &_ => todo!(),
    }
}

fn resize_jpeg_from_buffer(buffer: &[u8], max_width_size: i32) -> Result<Vec<u8>> {
    let open_options = "[access=VIPS_ACCESS_SEQUENTIAL]";
    let source_image = VipsImage::new_from_buffer(buffer, open_options)?;
    let result_image = ops::thumbnail_image(&source_image, max_width_size)?;
    let save_options = ops::JpegsaveBufferOptions {
        q: 80,
        background: vec![255.0],
        strip: true,
        optimize_coding: true,
        optimize_scans: true,
        interlace: true,
        ..ops::JpegsaveBufferOptions::default()
    };

    ops::jpegsave_buffer_with_opts(&result_image, &save_options)
}

fn resize_webp_from_buffer(buffer: &[u8], max_width_size: i32) -> Result<Vec<u8>> {
    let open_options = "[access=VIPS_ACCESS_SEQUENTIAL,n=-1]";
    let source_image = VipsImage::new_from_buffer(buffer, open_options)?;
    let result_image = ops::thumbnail_image(&source_image, max_width_size)?;
    let save_options = ops::WebpsaveBufferOptions {
        q: 70,
        strip: true,
        lossless: false,
        effort: 2,
        ..ops::WebpsaveBufferOptions::default()
    };

    ops::webpsave_buffer_with_opts(&result_image, &save_options)
}

fn resize_png_from_buffer(buffer: &[u8], max_width_size: i32) -> Result<Vec<u8>> {
    let open_options = "[access=VIPS_ACCESS_SEQUENTIAL]";
    let source_image = VipsImage::new_from_buffer(buffer, open_options)?;
    let result_image = ops::thumbnail_image(&source_image, max_width_size)?;
    let save_options = ops::PngsaveBufferOptions {
        q: 80,
        strip: true,
        bitdepth: 8,
        ..ops::PngsaveBufferOptions::default()
    };

    ops::pngsave_buffer_with_opts(&result_image, &save_options)
}

fn resize_gif_from_buffer(buffer: &[u8], max_width_size: i32) -> Result<Vec<u8>> {
    let open_options = "[access=VIPS_ACCESS_SEQUENTIAL,n=-1]";
    let source_image = VipsImage::new_from_buffer(buffer, open_options)?;
    let result_image = ops::thumbnail_image(&source_image, max_width_size)?;
    let save_options = ops::GifsaveBufferOptions {
        strip: true,
        ..ops::GifsaveBufferOptions::default()
    };

    ops::gifsave_buffer_with_opts(&result_image, &save_options)
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

fn get_dirs(id: PathBuf, max_width_size: i32, ext: &str) -> (PathBuf, PathBuf) {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let original_file_name = Path::new(upload_dir).join(id.clone()).join("original");
    let new_file_name = Path::new(upload_dir)
        .join(id)
        .join(max_width_size.to_string())
        .with_extension(ext);
    (original_file_name, new_file_name)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn test_process_jpeg_image_from_buffer() {
        let mut file = File::open("samples/240.jpeg").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let result = process_image_from_buffer(&buffer, 100, "jpeg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_webp_image_from_buffer() {
        let mut file = File::open("samples/240.webp").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let result = process_image_from_buffer(&buffer, 100, "webp");
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_png_image_from_buffer() {
        let mut file = File::open("samples/320.png").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let result = process_image_from_buffer(&buffer, 100, "png");
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_gif_image_from_buffer() {
        let mut file = File::open("samples/320.gif").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let result = process_image_from_buffer(&buffer, 100, "gif");
        assert!(result.is_ok());
    }
}