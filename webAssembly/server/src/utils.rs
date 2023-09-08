use std::{fs::File, path::Path};

use image::codecs::jpeg::JpegEncoder;

pub fn run(quantity: u8, file_name: &str) -> Vec<u8> {
    let file_path = format!("src/static/{}", file_name);
    println!("文件名: {}", file_path);

    // 打开图片
    let img = image::open(&file_path).unwrap();

    let new_file_path = format!("src/static/{}-new.jpg", file_name);
    // 创建一个写入目标的文件
    let img_write = File::create(Path::new(&new_file_path)).unwrap();

    // 使用指定的质量设置对图片进行编码
    let mut encoder = JpegEncoder::new_with_quality(img_write, quantity);

    encoder
        .encode(img.as_bytes(), img.width(), img.height(), img.color())
        .unwrap();

    println!("图片压缩完成!");

    // 读取压缩后的图片
    std::fs::read(new_file_path).unwrap()
}
