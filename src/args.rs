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

    Args {text, path, size, dark_color, light_color, border}
}