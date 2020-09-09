extern crate hidapi;

use rusb::{
    ConfigDescriptor, DeviceDescriptor, DeviceHandle, DeviceList, EndpointDescriptor,
    InterfaceDescriptor, Language, Result, Speed, UsbContext,
};

use std::time::Duration;

struct UsbDevice<T: UsbContext> {
    handle: DeviceHandle<T>,
    language: Language,
    timeout: Duration,
}

pub fn print_hid(print_path: bool)
{
    let api = hidapi::HidApi::new().unwrap();
    
    let mut device_desc_max_length = 0;
    for device in api.device_list()
    {
        let manufacturer_string = device.manufacturer_string().unwrap_or("Unknown MFG.");
        let product_string = device.product_string().unwrap_or("None");
        let device_description_string = format!("{} {}", manufacturer_string, product_string);
        
        let length = device_description_string.len();
        if length > device_desc_max_length
        {
            device_desc_max_length = length;
        }
    }
    
    // convert iterator to vector
    let mut devices: Vec<&hidapi::DeviceInfo> = Vec::new();
    for device in api.device_list()
    {
        devices.push(device);
    }
    //sort vector
    devices.sort_by(|dev1, dev2| dev1.vendor_id().cmp(&dev2.vendor_id())
                            .then(dev1.product_id().cmp(&dev2.product_id()))
                            .then(dev1.path().cmp(&dev2.path())));
    
    for device in devices
    {
        let manufacturer_string = device.manufacturer_string().unwrap_or("Unknown MFG.");
        let product_string = device.product_string().unwrap_or("None");
        let device_description_string = format!("{} {}", manufacturer_string, product_string);
        let vendor_id = device.vendor_id();
        let product_id = device.product_id();
        let path = device.path();
        if print_path
        {
            println!("{0: <length$}{1:04x}:{2:04x} {3:?}", device_description_string, vendor_id, product_id, path, length = device_desc_max_length + 2);
        }
        else
        {
            println!("{0: <length$}{1:04x}:{2:04x}", device_description_string, vendor_id, product_id, length = device_desc_max_length + 2);
        }
    }
}


pub fn print_all(print_path: bool)
{
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
    }
}

pub fn print_devices() -> Result<()>
{
    let timeout = Duration::from_secs(1);
    
    for device in DeviceList::new()?.iter() 
    {
        let device_desc = match device.device_descriptor() 
        {
            Ok(d) => d,
            Err(_) => continue,
        };

        let mut usb_device = 
        {
            match device.open() 
            {
                Ok(h) => match h.read_languages(timeout) 
                {
                    Ok(l) => 
                    {
                        if l.len() > 0 
                        {
                            Some(UsbDevice 
                            {
                                handle: h,
                                language: l[0],
                                timeout,
                            })
                        }
                        else 
                        {
                            None
                        }
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            }
        };
        print_device(&device_desc, &mut usb_device);
    }
    Ok(())
}

fn print_device<T: UsbContext>(device_desc: &DeviceDescriptor, handle: &mut Option<UsbDevice<T>>)
{
    println!("{0:<20} {1:?}", 
            "iManufacturer",
            handle.as_mut().map_or(String::new(), |h| h
            .handle
            .read_manufacturer_string(h.language, device_desc, h.timeout)
            .unwrap())
    );
    println!("{0:<20} {1:?}",
            "iManufacturer index",
             device_desc.manufacturer_string_index().unwrap());
    println!(
            "{0:<20} {1:?}",
            "iProduct",
            handle.as_mut().map_or(String::new(), |h| h
            .handle
            .read_product_string_ascii(device_desc)
            .unwrap())
    );
    println!("{0:<20} {1:?}",
            "iProduct index", 
            device_desc.product_string_index().unwrap());
    
}
