mod options;
// mod config;
// mod oshelper;

extern crate rusb;
extern crate hidapi;

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::{str::FromStr};

extern crate libc;

use libc::c_char;
use std::ffi::CString;

fn main() 
{
    let matches = options::get_matches();

    let vid_hex: String = FromStr::from_str(matches.value_of("vendor_id").unwrap()).unwrap();
    let pid_hex: String = FromStr::from_str(matches.value_of("product_id").unwrap()).unwrap();

    let vid_base10 = convert_hex_to_u16(&vid_hex);
    let pid_base10 = convert_hex_to_u16(&pid_hex);

    let api = hidapi::HidApi::new().unwrap();
    for device in api.device_list() 
    {
        println!("{:04x}:{:04x}:{:?}", device.vendor_id(), device.product_id(), device.path());
    }
   
    let try_open_dev = api.open_path(&CString::new("\\\\?\\hid#ven_8086&dev_9d3e&subsys_00000000&rev_21&col07#4&f87ce30&0&0006#{4d1e55b2-f16f-11cf-88cb-001111000030}").unwrap());
    // let try_open_dev = api.open(vid_base10, pid_base10);
    // let try_open_dev = api.open(0x062a, 0x5918);
    
    if try_open_dev.is_ok()
    {
        let device = try_open_dev.unwrap();
        device.set_blocking_mode(true).unwrap();
        for _ in 0..1000
        {
            // Read data from device
            let mut buf = [0u8; 8];
            let res = device.read(&mut buf[..]).unwrap();
            println!("Read: {:?}", &buf[..res]);
        }       
    }

}

fn convert_hex_to_u16(hex: &str) -> u16
{
    match u16::from_str_radix(without_hex_prefix(&hex), 16)
    {
        // Err(e) => panic!("Input must be in hex format"),
        Err(_e) =>
        {
            println!("Input must be in hex form!");
            std::process::exit(1);
        },
        Ok(int) => int as u16,
    }
}

fn without_hex_prefix(hex: &str) -> &str
{
    hex.trim_start_matches("0x")
}

