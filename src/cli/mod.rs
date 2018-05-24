mod clap;
use self::clap::{parser, parse_int};

#[derive(Debug)]
pub struct Args {
    pub images: Vec<String>,
    pub depth: usize
}

impl Args {
    pub fn new() -> Result<Self, String> {
        let matches = parser().get_matches();
        let images = matches.values_of("file").unwrap().map(|value| value.to_string()).collect();
        let depth = match matches.value_of("depth") {
            Some(depth) => parse_int(depth)?,
            None => 1
        };

        Ok(Args {
            images,
            depth
        })
    }
}
