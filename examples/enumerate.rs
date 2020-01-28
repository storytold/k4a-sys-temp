extern crate k4a_sys;
use std::ptr;
use k4a_sys::*;

fn main() {
    unsafe {
        let mut device: k4a_sys::k4a_device_t = ptr::null_mut();

        let device_count = k4a_device_get_installed_count();
        println!("Found {} connected devices:", device_count);
        for device_idx in 0..device_count {
            if k4a_device_open(device_idx, &mut device) != k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
                println!("Failed to open device #{}", device_idx);
                continue;
            }
            let mut serial_number_length: usize = 0;

            if k4a_device_get_serialnum(device, ptr::null_mut(), &mut serial_number_length) != 
                k4a_buffer_result_t_K4A_BUFFER_RESULT_TOO_SMALL {
                println!("{}: Failed to get serial number length", device_idx);
                k4a_device_close(device);
                device = ptr::null_mut();
                continue;
            }

            let mut serial_number = vec![0i8; serial_number_length];
            let serial_number_ptr = (&mut serial_number).as_mut_ptr();

            if k4a_device_get_serialnum(device, serial_number_ptr, &mut serial_number_length) != 
                k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
                println!("{}: Failed to get serial number", device_idx);
                k4a_device_close(device);
                device = ptr::null_mut();
                continue;
            }
        println!("{}: Device \"{:?}\"\n", device_idx, serial_number);
        k4a_device_close(device);
    }
    }
}
