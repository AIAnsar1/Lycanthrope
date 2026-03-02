pub mod bindings;

use crate::errors::*;
use bindings::*;
use std::ffi::CStr;
use std::net::Ipv4Addr;

pub fn build_tcp_packet(
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
    src_port: u16,
    dst_port: u16,
    seq: u32,
    ack: u32,
    flags: u8,
    payload: &[u8],
) -> Result<Vec<u8>> {
    let params = TcpPacketParams {
        src_ip: u32::from(src_ip).to_be(),
        dst_ip: u32::from(dst_ip).to_be(),
        src_port,
        dst_port,
        seq,
        ack,
        flags,
        window: if payload.is_empty() { 4 } else { 65535 },
        ttl: 64,
    };
    let mut buffer = vec![0u8; 65535];

    let result = unsafe {
        lyc_build_tcp_packet(
            &params,
            if payload.is_empty() {
                std::ptr::null()
            } else {
                payload.as_ptr()
            },
            payload.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
        )
    };

    if result < 0 {
        bail!(LycError::PacketBuild(result));
    }

    buffer.truncate(result as usize);
    Ok(buffer)
}

pub fn tcp_checksum(src_ip: Ipv4Addr, dst_ip: Ipv4Addr, segment: &[u8]) -> u16 {
    unsafe {
        lyc_tcp_checksum(
            u32::from(src_ip).to_be(),
            u32::from(dst_ip).to_be(),
            segment.as_ptr(),
            segment.len(),
        )
    }
}

pub fn ip_checksum(header: &[u8]) -> u16 {
    unsafe { lyc_ip_checksum(header.as_ptr(), header.len()) }
}

fn last_c_error() -> String {
    unsafe {
        let ptr = lyc_last_error();
        if ptr.is_null() {
            return "unknown error".to_string();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn network_init() -> Result<()> {
    let ret = unsafe { lyc_network_init() };
    if ret < 0 {
        bail!(LycError::NetworkInit(last_c_error()));
    }
    Ok(())
}

pub fn network_cleanup() {
    unsafe { lyc_network_cleanup() }
}

pub struct RawSocket {
    handle: i64,
}

unsafe impl Send for RawSocket {}
unsafe impl Sync for RawSocket {}

impl RawSocket {
    pub fn new() -> Result<Self> {
        network_init()?;

        let handle = unsafe { lyc_raw_socket_create() };

        if handle < 0 {
            bail!(LycError::SocketCreate(last_c_error()));
        }
        info!("Raw socket created: handle={}", handle);
        Ok(Self { handle })
    }

    pub fn send_packet(&self, packet: &[u8], dst_ip: Ipv4Addr, dst_port: u16) -> Result<usize> {
        let ret = unsafe {
            lyc_raw_socket_send(
                self.handle,
                packet.as_ptr(),
                packet.len(),
                u32::from(dst_ip).to_be(),
                dst_port,
            )
        };

        if ret < 0 {
            bail!(LycError::SendFailed(last_c_error()));
        }
        trace!("Sent {} bytes to {}:{}", ret, dst_ip, dst_port);
        Ok(ret as usize)
    }

    pub fn handle(&self) -> i64 {
        self.handle
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        debug!("Closing raw socket: handle={}", self.handle);
        unsafe { lyc_raw_socket_close(self.handle) }
    }
}
