const VERSION: &'static str = "1.0.1";

mod macros;

use std::
{
    fs, process, 
    path::{PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use qrcode::QrCode;
use clap::{App, Arg};
use image::Rgb;
use colorskill::color_name_to_rgb;

// Program starts here
fn main() 
{ 
    let args = start_args();
    make_image(args);
}

// Exit the program
fn exit(s: &str) -> !
{
    p!(s);
    process::exit(0)
}

// Make the QR Code image
// using the specified settings
fn make_image(args: Settings)
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

    p!("Image saved as: {}", args.path.to_str().unwrap())
}

// Struct for settings
struct Settings
{
    text: String,
    path: PathBuf,
    size: u32,
    dark_color: String,
    light_color: String,
    border: bool,
}

// Start the argument system
fn start_args() -> Settings
{
    let matches = App::new("qool")
    .version(VERSION)
    .about("Encrypted CLI Notepad")
    .arg(Arg::with_name("Text")
        .help("Text to be encoded")
        .required(true)
        .index(1))
    .arg(Arg::with_name("Path")
        .help("File path to save the image")
        .required(false)
        .index(2))
    .arg(Arg::with_name("size")
        .long("size")
        .value_name("size")
        .help("Approximate size of the image")
        .takes_value(true)) 
    .arg(Arg::with_name("dark-color")
        .long("dark-color")
        .value_name("dark-color")
        .help("The dark color. This could be a name like red, green, blue, etc")
        .takes_value(true)) 
    .arg(Arg::with_name("light-color")
        .long("light-color")
        .value_name("light-color")
        .help("The light color. This could be a name like red, green, blue, etc")
        .takes_value(true))
    .arg(Arg::with_name("no-border")
        .long("no-border")
        .multiple(false)
        .help("Don't show a border around the QR code"))
    .get_matches();

    let text = s!(matches.value_of("Text").unwrap());

    let path = if let Some(pth) = matches.value_of("Path")
    {
        PathBuf::from(pth)
    }

    else
    {
        home().join(PathBuf::from("qool-codes"))
            .join(format!("{}.png", now()))
    };

    let size = if let Some(sze) = matches.value_of("size")
    {
        let n = sze.parse::<u32>().unwrap_or_else(|_|
        {
            exit("Wrong size format.");
        });

        n
    }

    else
    {
        800
    };

    let dark_color = if let Some(clr) = matches.value_of("dark-color")
    {
        s!(clr)
    }

    else
    {
        s!("black")
    };

    let light_color = if let Some(clr) = matches.value_of("light-color")
    {
        s!(clr)
    }

    else
    {
        s!("white")
    };

    let border = !(matches.occurrences_of("no-border") > 0);

    Settings {text, path, size, dark_color, light_color, border}
}

// Get the home directory
fn home() -> PathBuf
{
    dirs::home_dir().unwrap()
}

// Get the unix time in seconds
fn now() -> u64
{
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap().as_secs()
}

// Get a color Rgb by name
fn color(s: &str) -> Rgb<u8>
{
    let t = color_name_to_rgb(s, (0, 0, 0));
    Rgb([t.0, t.1, t.2])
}