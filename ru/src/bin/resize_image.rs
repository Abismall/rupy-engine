use std::env;

fn resize_image(
    input_path: &str,
    output_path: &str,
    new_width: u32,
    new_height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    let resized_img =
        img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
    let format = image::guess_format(&std::fs::read(input_path)?)?;
    resized_img.save_with_format(output_path, format)?;
    println!("Image resized successfully.");
    println!("File saved to {}", output_path);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <input_path> <output_path> <new_width> <new_height>",
            args[0]
        );
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let new_width: u32 = args[3].parse().expect("Invalid width");
    let new_height: u32 = args[4].parse().expect("Invalid height");

    if let Err(e) = resize_image(input_path, output_path, new_width, new_height) {
        eprintln!("Error resizing image: {}", e);
    }
}
