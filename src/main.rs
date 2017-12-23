extern crate oxipng;
extern crate num_cpus;

mod cli;
mod image;

use std::cmp;

fn run() -> Result<i32, String> {
    let args = cli::Args::new()?;

    //Use at most 1/2 of physical CPU
    let cpu_num = cmp::max(num_cpus::get_physical() / 2, 1);
    let mut oxipng_options = oxipng::Options::from_preset(4);
    oxipng_options.verbosity = None;
    oxipng_options.threads = cpu_num;
    oxipng_options.strip = oxipng::headers::Headers::Safe;

    for image in args.images {
        println!(">>>Optimize {}", &image);
        let image = match image::Image::open(&image) {
            Ok(result) => result,
            Err(error) => {
                println!("Unable to open file. Error: {}", error);
                continue
            }
        };

        println!("Size={}b", image.len);
        let new_data = if image.is_png() {
            match oxipng::optimize_from_memory(image.slice(), &oxipng_options) {
                Ok(result) => result,
                Err(error) => {
                    println!("PNG Error: {}", error);
                    continue;
                }
            }
        }
        else {
            println!("Not supported...");
            continue;
        };

        println!("Optimized={}b", new_data.len());
        if (new_data.len() as u64) < image.len {
            match image.update(&new_data) {
                Ok(_) => (),
                Err(error) => println!("Couldn't write file. Error {}", error)
            }
        }
    }

    Ok(0)
}

fn main() {
    use std::process::exit;

    let code: i32 = match run() {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            1
        }
    };

    exit(code);
}

