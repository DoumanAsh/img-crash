use arg::Args;

#[derive(Args, Debug)]
///img-crash
///Image optimizer
pub struct Cli {
    ///Image or folder to compress.
    pub file: Vec<String>,
    #[arg(short, long, default_value = "1")]
    ///Recursion depth in case of directory. Default 1
    pub depth: usize
}
