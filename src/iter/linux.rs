use nix::{ifaddrs, sys::socket::SockAddr};
use {MacAddress, MacAddressError};
use iter::Interface;

/// An iterator over all available MAC addresses on the system.
pub struct InterfaceIterator {
    iter: std::iter::FilterMap<
        ifaddrs::InterfaceAddressIterator,
        fn(ifaddrs::InterfaceAddress) -> Option<Interface>,
    >,
}

impl InterfaceIterator {
    /// Creates a new `InterfaceIterator`.
    pub fn new() -> Result<InterfaceIterator, MacAddressError> {
        Ok(Self {
            iter: ifaddrs::getifaddrs()?.filter_map(filter_macs),
        })
    }
}

fn filter_macs(intf: ifaddrs::InterfaceAddress) -> Option<Interface> {
    if let SockAddr::Link(link) = intf.address? {
        Some(Interface::new(intf.interface_name, MacAddress::new(link.addr())))
    } else {
        None
    }
}

impl Iterator for InterfaceIterator {
    type Item = Interface;

    fn next(&mut self) -> Option<Interface> {
        self.iter.next()
    }
}
