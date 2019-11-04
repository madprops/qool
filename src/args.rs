use crate::
{
    s, exit,
    VERSION,
    utils::*,
};

use std::
{
    path::{PathBuf},
};

use clap::{App, Arg};

// Struct for settings
pub struct Args
{
    pub text: String,
    pub path: PathBuf,
    pub size: u32,
    pub dark_color: String,
    pub light_color: String,
    pub border: bool,
    pub print: bool,
}

// Start the argument system
pub fn start_args() -> Args
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
    .arg(Arg::with_name("print")
        .long("print")
        .multiple(false)
        .help("Only print output to the terminal"))        
    .get_matches();

    let text = s!(matches.value_of("Text").unwrap());

    let path = match matches.value_of("Path")
    {
        Some(pth) => PathBuf::from(pth),
        None =>
        {
            home().join(PathBuf::from("qool-codes"))
            .join(format!("{}.png", name(&text)))
        }
    };

    let size = match matches.value_of("size")
    {
        Some(sze) =>
        {
            sze.parse::<u32>().unwrap_or_else(|_|
            {
                exit("Wrong size format.");
            })
        },
        None => 800
    };

    let dark_color = match matches.value_of("dark-color")
    {
        Some(color) => s!(color),
        None => s!("black")
    };

    let light_color = match matches.value_of("light-color")
    {
        Some(color) => s!(color),
        None => s!("white")
    };

    let border = matches.occurrences_of("no-border") == 0;
    let print = matches.occurrences_of("print") > 0;

    Args 
    {
        text, path, size, dark_color, 
        light_color, border, print,
    }
}