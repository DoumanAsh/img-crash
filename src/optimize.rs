use std::io;

use crate::image;

pub struct Optimizer {
    oxipng_options: oxipng::Options
}

impl Optimizer {
    pub fn new() -> Self {
        //Use at most 1/2 of physical CPU
        let mut oxipng_options = oxipng::Options::from_preset(4);
        oxipng_options.strip = oxipng::Headers::Safe;

        Self {
            oxipng_options
        }
    }

    pub fn optimize(&self, image_path: &str) -> io::Result<()> {
        let image = image::Image::open(&image_path)?;
        println!(">>>Optimize {}", &image_path);

        println!("Size={}b", image.len);
        let new_data = if image.is_png() {
            match oxipng::optimize_from_memory(image.slice(), &self.oxipng_options) {
                Ok(result) => result,
                Err(error) => {
                    println!("PNG Error: {}", error);
                    return Ok(());
                }
            }
        }
        else {
            println!("Not supported...");
            return Ok(());
        };

        println!("Optimized={}b", new_data.len());
        if (new_data.len() as u64) < image.len {
            match image.update(&new_data) {
                Ok(_) => (),
                Err(error) => println!("Couldn't write file. Error {}", error)
            }
        }

        Ok(())
    }
}
