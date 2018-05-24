extern crate oxipng;
extern crate num_cpus;
extern crate walkdir;

mod cli;
mod image;
mod optimize;

use walkdir::WalkDir;

use std::fs;

#[inline]
///Filters errors out and prints them, if needed.
fn walk_filter_map_error(value: walkdir::Result<walkdir::DirEntry>) -> Option<walkdir::DirEntry> {
    match value {
        Ok(entry) => Some(entry),
        Err(error) => {
            eprintln!("Unexpected error while walking directory: {}", error);
            None
        }
    }
}

#[inline]
///Filter by type of entry.
fn walk_filter_type(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
}

fn run() -> Result<i32, String> {
    let args = cli::Args::new()?;

    let optimizer = optimize::Optimizer::new();

    for image in args.images {
        let meta = match fs::metadata(&image) {
            Ok(meta) => meta,
            Err(error) => {
                println!("Unable to access path '{}'. Error: {}", image, error);
                continue;
            }
        };
        if meta.is_file() {
            match optimizer.optimize(&image) {
                Ok(_) => (),
                Err(error) => println!("Unable to optimize '{}'. {}", image, error)
            }
        } else if meta.is_dir() {
            let walker = WalkDir::new(&image).min_depth(1).max_depth(args.depth)
                                             .into_iter()
                                             .filter_map(walk_filter_map_error)
                                             .filter(walk_filter_type);

            for entry in walker {
                match entry.path().to_str() {
                    Some(entry) => {
                        let _ = optimizer.optimize(entry);
                    },
                    None => println!("{}: Not a valid unicode path", entry.path().display())
                }
            }

        } else {
            println!("Not a file or directory. Ignore");
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

