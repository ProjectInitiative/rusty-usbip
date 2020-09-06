extern crate clap;
use self::clap::{Arg, App};

pub fn get_matches() -> clap::ArgMatches<'static>
{
    return App::new("usbinfo")
                    .version("0.0.1")
                    .author("Kyle Petryszak <projectinitiativedev@gmail.com>")
                    .about("USB device information")
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Sets the level of verbosity"))
                    .arg(Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .help("print out USB info with path data"))
                    .arg(Arg::with_name("hid")
                        .short("j")
                        .long("hid")
                        .help("print out HID USB info"))
                    .get_matches();
}