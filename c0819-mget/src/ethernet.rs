//! MAC 地址格式
//! 3个字节，4位一组，第一组最后两个位是标志位，用于区分 mac 地址类型
//! 按类型分为 本地地址，通用地址
//! 按模式分为 单播地址，组播地址

use rand;
use rand::RngCore;
use smoltcp::wire;
use std::fmt::Display;
use std::fmt::{self, write};

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        // 随机生成串
        rand::thread_rng().fill_bytes(&mut octets);
        // 本地标识位设置为1
        octets[0] |= 0b_0000_0010;
        // 广播标识位设置为0
        octets[0] &= 0b_1111_1110;
        MacAddress { 0: octets }
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress { 0: self.0 }
    }
}
