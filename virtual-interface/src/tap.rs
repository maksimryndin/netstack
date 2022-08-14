use std::ffi;
use std::fs::{File, OpenOptions};
use std::io;
use std::mem;
use std::os::unix::io::{FromRawFd, IntoRawFd};

use bindings;

const VIRTUAL_DEVICE: &str = "/dev/net/tun";

#[derive(Debug)]
pub enum VirtualInterfaceError {
    IoError(io::Error),
    IoctlError,
    DeviceNameTooLong,
    DeviceNameContainsNulByte(ffi::NulError),
}

impl From<io::Error> for VirtualInterfaceError {
    fn from(error: io::Error) -> Self {
        VirtualInterfaceError::IoError(error)
    }
}

impl From<ffi::NulError> for VirtualInterfaceError {
    fn from(error: ffi::NulError) -> Self {
        VirtualInterfaceError::DeviceNameContainsNulByte(error)
    }
}

#[derive(Debug)]
pub struct VirtualInterface {
    device: File,
}

impl VirtualInterface {
    pub fn create(name: &str) -> Result<Self, VirtualInterfaceError> {
        // reserve 1 byte for '\0'
        if name.len() >= bindings::IFNAMSIZ as usize {
            return Err(VirtualInterfaceError::DeviceNameTooLong);
        }
        // We have to check that the device name has no zero bytes in the middle
        let device_name = ffi::CString::new(name)?.into_bytes_with_nul();
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .open(VIRTUAL_DEVICE)?;
        // ifreq is a structure to control network device (see man 7 netdevice)
        let mut ifr: bindings::ifreq = unsafe { mem::zeroed() };

        // create stack allocated array to hold the device name
        let mut name_buffer = [0_u8; bindings::IFNAMSIZ as usize];
        // and copy name bytes to it
        for (i, b) in device_name.into_iter().enumerate() {
            name_buffer[i] = b;
        }
        ifr.ifr_ifrn.ifrn_name = name_buffer;
        // IFF_TAP - tap device
        // IFF_NO_PI - no additional info for Ethernet package
        // IFF_TUN_EXCL - prevent creation of duplicates
        ifr.ifr_ifru.ifru_flags = (bindings::IFF_TAP | bindings::IFF_NO_PI | bindings::IFF_TUN_EXCL)
            as std::os::raw::c_short;

        let raw_fd = device.into_raw_fd();
        // man ioctl: on error, -1 is returned, and errno is set appropriately.
        if unsafe { bindings::ioctl(raw_fd, bindings::TUNSETIFF as u64, &mut ifr as *mut _) } == -1
        {
            return Err(VirtualInterfaceError::IoctlError);
        }
        let device = unsafe { File::from_raw_fd(raw_fd) };
        Ok(Self { device })
    }

    pub fn device(&mut self) -> &mut File {
        &mut self.device
    }
}


#[cfg(test)]
mod tests {
    use std::process::Command;
    use super::*;

    #[test]
    fn interface_long_name() {
        match VirtualInterface::create("abcdefghijklomnpqrstuv") {
            Err(VirtualInterfaceError::DeviceNameTooLong) => return,
            _ => panic!("device name shouldn't be more than 20 chars"),
        };
    }

    #[test]
    fn interface_name_contains_zero_byte() {
        match VirtualInterface::create("dev\0ds0") {
            Err(VirtualInterfaceError::DeviceNameContainsNulByte(_)) => return,
            _ => panic!("device name shouldn't contain zero byte"),
        };
    }

    // #[test]
    // fn can_create_interface() {
    //     VirtualInterface::create("dev0").unwrap();
    //     Command::new("ip")
    //         .arg("link")
    //         .arg("show")
    //         .arg("dev0")
    //         .output()
    //         .expect("failed to get interface dev0");
    // }
}