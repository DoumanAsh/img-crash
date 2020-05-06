#![no_main]

mod cli;
mod image;
mod optimize;

use arg::Args;
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

#[no_mangle]
unsafe extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let args = c_ffi::Args::new(argc, argv).expect("To get function arguments");

    let args = match cli::Cli::from_args(args.into_iter().skip(1)) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return !err.is_help() as isize
        }
    };

    let optimizer = optimize::Optimizer::new();

    for image in args.file {
        let meta = match fs::metadata(&image) {
            Ok(meta) => meta,
            Err(error) => {
                eprintln!("Unable to access path '{}'. Error: {}", image, error);
                continue;
            }
        };

        if meta.is_file() {
            match optimizer.optimize(&image) {
                Ok(_) => (),
                Err(error) => eprintln!("Unable to optimize '{}'. {}", image, error)
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
                    None => eprintln!("{}: Not a valid unicode path", entry.path().display())
                }
            }

        } else {
            eprintln!("Not a file or directory. Ignore");
        }

    }

    0
}
