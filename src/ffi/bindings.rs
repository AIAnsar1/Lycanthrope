#![allow(non_camel_case_types)]

use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TcpPacketParams {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u8,
    pub window: u16,
    pub ttl: u8,
}

pub const LYC_TH_FIN: u8 = 0x01;
pub const LYC_TH_SYN: u8 = 0x02;
pub const LYC_TH_RST: u8 = 0x04;
pub const LYC_TH_PSH: u8 = 0x08;
pub const LYC_TH_ACK: u8 = 0x10;
pub const LYC_TH_URG: u8 = 0x20;

unsafe extern "C" {                   
    pub fn lyc_build_tcp_packet(params: *const TcpPacketParams,payload: *const u8,payload_len: usize,out_buffer: *mut u8,buffer_size: usize) -> c_int;
    pub fn lyc_tcp_checksum(src_ip: u32,dst_ip: u32,tcp_segment: *const u8,tcp_len: usize) -> u16;
    pub fn lyc_ip_checksum(header: *const u8, header_len: usize) -> u16;
    pub fn lyc_network_init() -> c_int;
    pub fn lyc_network_cleanup();
    pub fn lyc_raw_socket_create() -> i64;
    pub fn lyc_raw_socket_send(sock: i64,packet: *const u8,len: usize,dst_ip: u32,dst_port: u16) -> c_int;
    pub fn lyc_raw_socket_close(sock: i64);
    pub fn lyc_last_error() -> *const std::os::raw::c_char;
}