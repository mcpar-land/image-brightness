use image::{io::Reader as ImageReader, GenericImageView, Pixel};
use std::io::Cursor;

pub fn image_brightness(
	input: &[u8],
	sample_rate: usize,
) -> Result<f32, anyhow::Error> {
	let img = ImageReader::new(Cursor::new(input))
		.with_guessed_format()?
		.decode()?;

	let mut average: f32 = 0.0;
	let mut n_values: usize = 0;

	for (i, (_, _, pixel)) in img.pixels().enumerate().step_by(sample_rate) {
		n_values += 1;
		let pixel_brightness = pixel.to_luma()[0] as f32 / 255.0;
		if i != 0 {
			average += pixel_brightness;
		} else {
			average = pixel_brightness as f32;
		}
	}

	average /= n_values as f32;

	Ok(average)
}
