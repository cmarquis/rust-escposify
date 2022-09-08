extern crate tempfile;

extern crate escposify;
extern crate image;

use escposify::device::{self};
use escposify::img::Image;
use escposify::printer::Printer;
use image::{ImageBuffer, DynamicImage};

#[test]
fn simple() {
    let pn = device::Network::new("10.10.10.65", 9100).expect("oh no");
    let mut printer = Printer::new(pn, None, None);

    let img = ImageBuffer::from_fn(256, 256, |x, _| {
        if x % 2 == 0 {
            image::Rgb([0, 0, 0])
        } else {
            image::Rgb([0xFF, 0xFF, 0xFF])
        }
    });
    let image = Image::from(DynamicImage::ImageRgb8(img));

    let _ = printer
        .chain_text("hi")
        .unwrap()
        .chain_bit_image(&image, None)
        .unwrap()
        .chain_feed(1)
        .unwrap()
        .chain_text("ho")
        .unwrap()
        .chain_cut(false)
        .unwrap()
        .flush();
}
