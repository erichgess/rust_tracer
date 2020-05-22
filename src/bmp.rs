/// Save a buffer to a bitmap file
extern crate image;

use super::RenderBuffer;

pub fn save_to_bmp(filename: &str, buffer: &RenderBuffer) {
   let mut imgbuf = image::ImageBuffer::new(buffer.w as u32, buffer.h as u32);

   for (u,v,pixel) in imgbuf.enumerate_pixels_mut() {
       let (r, g, b) = buffer.buf[u as usize][v as usize].as_u8();
       *pixel = image::Rgb([r,g,b]);
   }

   imgbuf.save(filename).unwrap();
}
