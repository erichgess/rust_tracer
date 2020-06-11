/// Save a buffer to a bitmap file
extern crate image;

use std::fs::create_dir_all;

use super::RenderBuffer;

pub fn save_to_bmp(dir: &str, filename: &str, buffer: &RenderBuffer) -> std::io::Result<()> {
    let mut imgbuf = image::ImageBuffer::new(buffer.w as u32, buffer.h as u32);

    for (u, v, pixel) in imgbuf.enumerate_pixels_mut() {
        let (r, g, b) = buffer.buf[u as usize][v as usize].as_u8();
        *pixel = image::Rgb([r, g, b]);
    }

    create_dir_all(dir)?;
    imgbuf.save(format!("{}/{}", dir, filename)).unwrap();
    Ok(())
}
