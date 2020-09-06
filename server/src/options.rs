extern crate clap;
use self::clap::{Arg, App};

pub fn get_matches() -> clap::ArgMatches<'static>
{
    return App::new("usb-server")
                    .version("0.0.1")
                    .author("Kyle Petryszak <projectinitiativedev@gmail.com>")
                    .about("USB network server")
                    .arg(Arg::with_name("vendor_id")
                        .value_name("VID")
                        .help("Sets the vendor ID for the program")
                        .required(true)
                        .takes_value(true))
                    .arg(Arg::with_name("product_id")
                        .value_name("PID")
                        .help("Sets the product ID for the program")
                        .required(true)
                        .takes_value(true))
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Sets the level of verbosity"))
                    .arg(Arg::with_name("deletecache")
                        .short("x")
                        .long("deletecache")
                        .help("Deletes the configuration file"))
                    .get_matches();
}