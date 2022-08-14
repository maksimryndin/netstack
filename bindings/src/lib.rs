#![allow(nonstandard_style)]
#![allow(clippy::missing_safety_doc)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::mem;
use std::os::raw;

// /usr/include/asm-generic/ioctl.h
const IOC_SIZEBITS: u8 = 14;
const IOC_NRBITS: u8 = 8;
const IOC_TYPEBITS: u8 = 8;
const IOC_NRSHIFT: u8 = 0;
const IOC_TYPESHIFT: u8 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u8 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u8 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;

//#define TUNSETIFF     _IOW('T', 202, int)
const INT_SIZE: usize = mem::size_of::<raw::c_int>();
pub const TUNSETIFF: u32 = IOC_WRITE << IOC_DIRSHIFT
    | (b'T' as u32) << IOC_TYPESHIFT
    | 202 << IOC_NRSHIFT
    | (INT_SIZE as u32) << IOC_SIZESHIFT;
