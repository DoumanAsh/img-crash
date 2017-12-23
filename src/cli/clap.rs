extern crate clap;

pub use self::clap::{App, Arg, SubCommand, AppSettings};

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
}
