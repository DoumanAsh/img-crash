mod clap;
use self::clap::{parser};

#[derive(Debug)]
pub struct Args {
    pub images: Vec<String>
}

impl Args {
    pub fn new() -> Result<Self, String> {
        let matches = parser().get_matches();
        let images = matches.values_of("file").unwrap().map(|value| value.to_string()).collect();

        Ok(Args {
            images
        })
    }
}
