mod options;
// mod config;
// mod oshelper;

extern crate rusb;
extern crate hidapi;


fn main() 
{
    // Parse CLI options and arguments
    let matches = options::get_matches();

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
    devices.sort_by(|dev1, dev2| dev2.vendor_id().cmp(&dev1.vendor_id()));

    for device in devices
    {
        let manufacturer_string = device.manufacturer_string().unwrap_or("Unknown MFG.");
        let product_string = device.product_string().unwrap_or("None");
        let device_description_string = format!("{} {}", manufacturer_string, product_string);
        let vendor_id = device.vendor_id();
        let product_id = device.product_id();
        let path = device.path();
        if matches.is_present("path")
        {
            println!("{0: <length$}{1:04x}:{2:04x} {3:?}", device_description_string, vendor_id, product_id, path, length = device_desc_max_length + 2);
        }
        else
        {
            println!("{0: <length$}{1:04x}:{2:04x}", device_description_string, vendor_id, product_id, length = device_desc_max_length + 2);
        }
    }
}