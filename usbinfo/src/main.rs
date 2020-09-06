mod options;
mod info;

extern crate rusb;
extern crate hidapi;


fn main() 
{
    // Parse CLI options and arguments
    let matches = options::get_matches();
    
    if matches.is_present("hid")
    {
        if matches.is_present("path")
        {
            info::print_hid(true)
        }
        else
        {
            info::print_hid(false)
        }
    }
    else
    {
        if matches.is_present("path")
        {
            info::print_all(true)
        }
        else
        {
            // info::print_all(false)
            info::print_devices();
        }
    }
}


