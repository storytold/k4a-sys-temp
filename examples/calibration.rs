#![allow(non_upper_case_globals)]
extern crate k4a_sys;
use std::ptr;
use std::ffi::{CString};
use k4a_sys::*;

struct Device {
    idx : u32,
    k4a_device: k4a_device_t,
}
//The beginnings of a wrapper struct
impl Device {
    pub fn get_idx(&self) -> u32 {
        self.idx
    }

    pub fn get_k4a_device(&self) -> k4a_device_t {
        self.k4a_device
    }

    fn new(k4a_device: k4a_device_t, idx: u32) -> Self {
        Self {
            idx,
            k4a_device,
        }

    }

    pub fn open(device_idx: u32) -> Option<Self> {
        unsafe {
                let device_count = k4a_sys::k4a_device_get_installed_count();
                let mut device: k4a_device_t = ptr::null_mut();
                if device_count == 0 {
                    println!("No K4A devices found");
                    return None;
                }

                if k4a_device_open(device_idx, &mut device) != k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
                    println!("Failed to open device");
                    return None;
                }
        Some(Self::new(device, device_idx))
        }
    }

    pub fn get_serial(&self) -> Option<String> {
        unsafe {
            let mut serial_number_length: usize = 0;

            if k4a_device_get_serialnum(self.get_k4a_device(), ptr::null_mut(), &mut serial_number_length) != 
                k4a_buffer_result_t_K4A_BUFFER_RESULT_TOO_SMALL {
                println!("{}: Failed to get serial number length", self.get_idx());
                return None
            }

            let mut serial_number = CString::new(vec![1u8; serial_number_length]).expect("Building a cstring failed");
            let serial_number_ptr = serial_number.into_raw();

            if k4a_device_get_serialnum(self.get_k4a_device(), serial_number_ptr, &mut serial_number_length) != 
                k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
                    println!("{}: Failed to get serial number", self.get_idx());
                    return None;
            }
            serial_number = CString::from_raw(serial_number_ptr);
            serial_number.into_string().ok() 
        }
    }
}
fn main () {
    let device = Device::open(0).expect("Couldn't open device");
    let serial_number = device.get_serial().expect("Couldn't get serial number");
    println!("Found device with serial number {}", serial_number)
}