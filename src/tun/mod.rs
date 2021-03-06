use std::net::IpAddr;

use tokio::io::Result;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub struct TunDevice<H, A: Tx, B: Rx>
{
    _handle: H,
    tx: A,
    rx: B,
}

impl<H, A: Tx, B: Rx> TunDevice<H, A, B>
{
    pub fn new(handle: H, tx: A, rx: B) -> Self {
        TunDevice { _handle: handle, tx, rx }
    }

    pub fn split(&self) -> (A, B) {
        (self.tx.clone(), self.rx.clone())
    }
    //
    // pub fn send_packet(&mut self, buff: &[u8]) -> Result<()> {
    //     self.tx.send_packet(buff)
    // }
    //
    // pub fn recv_packet(&mut self, buff: &mut [u8]) -> Result<usize> {
    //     self.rx.recv_packet(buff)
    // }
}

pub trait Tx: Clone {
    fn send_packet(&mut self, buff: &[u8]) -> Result<()>;
}

pub trait Rx: Clone {
    fn recv_packet(&mut self, buff: &mut [u8]) -> Result<usize>;
}

#[cfg(target_os = "linux")]
pub fn create_device(address: IpAddr, netmask: IpAddr) -> Result<TunDevice<tun::platform::Device, tun::platform::posix::Writer, tun::platform::posix::Reader>> {
    linux::create_device(address, netmask)
}

#[cfg(target_os = "windows")]
pub fn create_device(address: IpAddr, netmask: IpAddr) -> Result<TunDevice<std::sync::Arc<simple_wintun::adapter::WintunStream>, crate::tun::windows::Writer, crate::tun::windows::Reader>> {
    windows::create_device(address, netmask)
}