use std::
{
    path::{PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use image::Rgb;
use colorskill::color_name_to_rgb;

// Get the home directory
pub fn home() -> PathBuf
{
    dirs::home_dir().unwrap()
}

// Get the unix time in seconds
pub fn now() -> u64
{
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap().as_secs()
}

// Get a color Rgb by name
pub fn color(s: &str) -> Rgb<u8>
{
    let t = color_name_to_rgb(s, (0, 0, 0));
    Rgb([t.0, t.1, t.2])
}