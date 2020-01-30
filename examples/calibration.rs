#![allow(non_upper_case_globals)]
extern crate k4a_sys;
use std::ptr;
use std::default;
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

    pub fn installed_count() -> u32 {
        unsafe {
            k4a_device_get_installed_count()
        }
    }
}
impl Drop for Device {
    fn drop (&mut self) {
        unsafe {
            k4a_device_close(self.k4a_device)
        }

    }
}
pub struct DeviceConfiguration (k4a_device_configuration_t);
impl Default for DeviceConfiguration {
    fn default() -> Self {
     Self (k4a_sys::k4a_device_configuration_t {
        color_format: k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
        color_resolution: k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
        depth_mode: k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_2X2BINNED,
        camera_fps: 0,
        synchronized_images_only: false,
        depth_delay_off_color_usec: 0,
        wired_sync_mode: 0,
        subordinate_delay_off_master_usec: 0,
        disable_streaming_indicator: false,
        }
     )
    }
}
impl DeviceConfiguration {
    pub fn init_disable_all() -> Self {
        Self (k4a_sys::k4a_device_configuration_t {
            color_format: k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            color_resolution: k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
            depth_mode: k4a_depth_mode_t_K4A_DEPTH_MODE_OFF,
            camera_fps: k4a_fps_t_K4A_FRAMES_PER_SECOND_30,
            synchronized_images_only: false,
            depth_delay_off_color_usec: 0,
            wired_sync_mode: k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE,
            subordinate_delay_off_master_usec: 0,
            disable_streaming_indicator: false,
        })
    }
}

fn print_calibration() {
    let num_devices = Device::installed_count();
    println!("Found {} connected devices", num_devices);
    
    unsafe {
            let mut config : DeviceConfiguration= DeviceConfiguration::init_disable_all();
            println!("my config exists");
            for i in 0..num_devices {
                let device = Device::open(i);

            }
    }

}

fn main () {
    let device = Device::open(0).expect("Couldn't open device");
    let serial_number = device.get_serial().expect("Couldn't get serial number");
    // println!("Found device with serial number {}", serial_number);
    print_calibration();
}