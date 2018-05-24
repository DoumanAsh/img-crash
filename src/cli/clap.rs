extern crate clap;

pub use self::clap::{App, Arg, SubCommand, AppSettings};

use ::std::fmt::Display;
use ::std::str::FromStr;

#[inline(always)]
///Shortcut to create CLI argument
pub fn arg(name: &str) -> Arg {
    Arg::with_name(name)
}

#[inline(always)]
///Shortcut to create CLI option/flag
pub fn flag(name: &str) -> Arg {
    arg(name).long(name)
}

#[inline(always)]
///Shortcut to parse integer
pub fn parse_int<T: FromStr>(name: &str) -> Result<T, String>
    where <T as FromStr>::Err: Display
{
    name.parse::<T>().map_err(|error| format!("Invalid number '{}' is supplied. {}", name, error))
}

const NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = "
Simple image optimizer.";

pub fn parser() -> App<'static, 'static> {
    App::new(NAME).about(ABOUT)
                  .author(AUTHOR)
                  .version(VERSION)
                  .setting(AppSettings::ArgRequiredElseHelp)
                  .setting(AppSettings::VersionlessSubcommands)
                  .arg(arg("file").required(true)
                                  .multiple(true)
                                  .takes_value(true)
                                  .help("Image to compress"))
                  .arg(flag("depth").required(false)
                                    .takes_value(true)
                                    .short("d")
                                    .help("Recursion depth in case of directory. Default 1"))
}
