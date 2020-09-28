extern crate photon_rs;
use std::io;

use photon_rs::native::{open_image, save_image};
use glob::{glob, MatchOptions, glob_with};

fn main() {
    println!("Hello, world!");
//     let mut img = open_image("dog.jpg");
//     photon_rs::filters::filter(&mut img, "twenties");
//     save_image(img, "new_image.jpg");
    image_files();

}

fn image_files() -> io::Result<()> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };
for entry in glob_with("./**/*.jpg", options).expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
Ok(())
}