use MacAddress;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod internal;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "linux.rs"]
mod internal;

pub struct Interface {
    name: String,
    addr: MacAddress
}

impl Interface {
    pub fn new(name: String, addr: MacAddress) -> Interface {
        Interface { name, addr }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn addr(&self) -> &MacAddress {
        &self.addr
    }
}

pub use self::internal::InterfaceIterator;
