use image;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use image::ImageBuffer;
use image::RgbaImage;
use image::DynamicImage;
use image::Rgba;

fn resize(img: &RgbaImage, new_dims: (u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (old_height, old_width) = img.dimensions();
    let (new_height, new_width) = new_dims;

    let mut resized = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = Rgba([old_pixel[0], old_pixel[1], old_pixel[2], old_pixel[3]]);
        }
    }

    resized
}

fn pixelate(img: &DynamicImage) -> RgbaImage {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(&img, (old_dims.0 / 10, old_dims.1 / 10)); // You can adjust the divisor for desired pixelation level

    let pixelated = resize(&small, old_dims);

    // Save the small and pixelated images as debugs
    small.save("small.png").unwrap();
    pixelated.save("pixelated.png").unwrap();

    pixelated
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("C:/Users/dabro/Desktop/pixelizer/photo.png")?
        .with_guessed_format()?
        .decode()?;

    let _img_ = pixelate(&img);

    // Save the original image as a debug
    img.save("original.png").unwrap();

    Ok(())
}
