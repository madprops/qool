pub const VERSION: &str = "1.2.0";

mod macros;
mod args;
mod utils;

use crate::
{
    args::*,
    utils::*,
};

use std::
{
    fs, process,
};

use qrcode::QrCode;
use image::Rgb;

// Program starts here
fn main() 
{ 
    let args = start_args();
    if args.print {print_code(args)}
    else {make_image(args)}
}

// Exit the program
fn exit(s: &str) -> !
{
    p!(s);
    process::exit(0)
}

// Make the QR Code image
// using the specified settings
fn make_image(args: Args)
{
    // Render the bits into an image.
    let image = QrCode::new(args.text.as_bytes()).unwrap()
        .render::<Rgb<u8>>()
        .dark_color(color(&args.dark_color))
        .light_color(color(&args.light_color))
        .min_dimensions(args.size, args.size)
        .max_dimensions(args.size, args.size)
        .quiet_zone(args.border)
        .build();

    // Get the file's parent directory
    let parent = &args.path.parent().unwrap_or_else(||
    { 
        exit("Can't get file's parent.");
    });

    // Create directories if they don't exist
    fs::create_dir_all(parent).unwrap_or_else(|_|
    {
        exit("Couldn't create directories.");
    });

    // Save the image.
    image.save(&args.path).unwrap_or_else(|_|
    {
        exit("Couldn't save image.");
    });

    let s = format!("\n{}Saved{} {}\n", 
        termion::color::Fg(termion::color::Green),
        termion::color::Fg(termion::color::Reset),
        args.path.to_str().unwrap());
    
    p!(s);
}

fn print_code(args: Args)
{
    let code = QrCode::new(args.text.as_bytes()).unwrap();

    let string = code.render::<char>()
        .quiet_zone(args.border)
        .module_dimensions(2, 1)
        .build();

    println!("{}", string);
}