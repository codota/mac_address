use os::win::{GetAdaptersAddresses, PIP_ADAPTER_ADDRESSES};
use std::ptr::null_mut;
use winapi::shared::{winerror::ERROR_SUCCESS, ws2def::AF_UNSPEC};
use {MacAddress, MacAddressError};
use iter::Interface;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

const GAA_FLAG_NONE: ::os::win::ULONG = 0x0000;

/// An iterator over all available MAC addresses on the system.
pub struct InterfaceIterator {
    #[allow(dead_code)]
    buffer: Vec<u8>,
    ptr: PIP_ADAPTER_ADDRESSES,
}

impl InterfaceIterator {
    /// Creates a new `InterfaceIterator`.
    pub fn new() -> Result<InterfaceIterator, MacAddressError> {
        let mut buf_len = 0;

        unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_NONE,
                null_mut(),
                null_mut(),
                &mut buf_len,
            );
        }

        // Allocate `buf_len` bytes, and create a raw pointer to it
        let mut adapters_list = vec![0u8; buf_len as usize];
        let ptr: PIP_ADAPTER_ADDRESSES = adapters_list.as_mut_ptr() as *mut _;

        // Get our list of adapters
        let result = unsafe {
            GetAdaptersAddresses(
                // [IN] Family
                AF_UNSPEC as u32,
                // [IN] Flags
                GAA_FLAG_NONE,
                // [IN] Reserved
                null_mut(),
                // [INOUT] AdapterAddresses
                ptr,
                // [INOUT] SizePointer
                &mut buf_len,
            )
        };

        if result != ERROR_SUCCESS {
            return Err(MacAddressError::InternalError);
        }

        Ok(Self {
            buffer: adapters_list,
            ptr,
        })
    }
}

impl Iterator for InterfaceIterator {
    type Item = Interface;

    fn next(&mut self) -> Option<Interface> {
        if self.ptr.is_null() {
            None
        } else {
            // PhysicalAddress is a `[u8; 8]`, until `TryFrom` stabilizes, this
            // is the easiest way to turn it into a `[u8; 6]`.
            let bytes = unsafe { *((&(*self.ptr).PhysicalAddress).as_ptr() as *const [u8; 6]) };
            self.ptr = unsafe { (*self.ptr).Next };

            let adapter_name = unsafe { u16_ptr_to_string((*self.ptr).FriendlyName).into_string() };
            if adapter_name.is_err() { return None; }

            Some(Interface::new(adapter_name.unwrap(), MacAddress::new(bytes)))
        }
    }
}

unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
}