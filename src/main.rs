use image::{DynamicImage, GenericImageView, ImageError};
use image::imageops::FilterType;

fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    image::open(path)
}

fn resize_image(img: &DynamicImage, new_width: u32) -> DynamicImage {
    let (original_width, original_height) = img.dimensions();
    let aspect_ratio = original_height as f32 / original_width as f32;

    let adjusted_aspect_ratio = aspect_ratio / 2.5;
    let new_height = ((new_width as f32 * adjusted_aspect_ratio)).round() as u32;
    println!("Resizing from {}x{} to {}x{}", original_width, original_height, new_width, new_height);

    img.resize_exact(new_width, new_height, FilterType::Lanczos3)
}

fn map_to_ascii(grayscale_value: u8) -> char {
    let ascii_chars = " .:;!>/lwO0&@";
    let index = (grayscale_value as f32 / 255.0 * (ascii_chars.len() - 1) as f32) as usize;
    ascii_chars.chars().nth(index).unwrap()
}

fn image_to_ascci(img: &DynamicImage) -> Vec<String> {
    let grayscaled_image = img.to_luma8();
    grayscaled_image
        .rows()
        .map(|row| {
            row.map(|pixel| map_to_ascii(pixel[0]))
                .collect::<String>()
        })
        .collect()
}

fn save_to_file(ascii_art_text: &[String], file_path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file_saved = File::create(file_path)?;
    for line in ascii_art_text {
        writeln!(file_saved, "{}", line)?;
    }
    Ok(())
}

fn main() {
    let input_path = "art.jpg";
    let output_path = "output.txt";
    let ascii_width = 170;

    match load_image(input_path) {
        Ok(img) => {
            let resized_img = resize_image(&img, ascii_width);
            let ascii_art = image_to_ascci(&resized_img);

            for line in &ascii_art {
                println!("{}", line);
            }

            match save_to_file(&ascii_art, output_path) {
                Ok(_) => {
                    println!("ASCII art saved successfully at {}", output_path);
                }
                Err(e) => {
                    eprintln!("Failed to save ASCII art to '{}': {}", output_path, e);
                }
            }
        }
        Err(e) => eprintln!("Failed to load image: {}", e),
    }
}