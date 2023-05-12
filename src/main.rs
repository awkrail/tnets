#[macro_use] extern crate log;
extern crate env_logger as logger;

mod util;
mod net;
mod driver {
    pub mod dummy;
}

const LOOPBACK_IP_ADDR: &str = "127.0.0.1";
const LOOPBACK_NETMASK: &str = "255.0.0.0";

const ETHER_TAP_NAME: &str = "tap0";
const ETHER_TAP_HW_ADDR: &str = "00:00:5e:00:53:01";
const ETHER_TAP_IP_ADDR: &str = "192.0.2.2";
const ETHER_TAP_NETMASK: &str = "255.255.255.0";

const DEFAULT_GATEWAY: &str = "192.0.2.1";

const TEST_DATA: [u8; 48] = [
    0x45, 0x00, 0x00, 0x30,
    0x00, 0x80, 0x00, 0x00,
    0xff, 0x01, 0xbd, 0x4a,
    0x7f, 0x00, 0x00, 0x01,
    0x7f, 0x00, 0x00, 0x01,
    0x08, 0x00, 0x35, 0x64,
    0x00, 0x80, 0x00, 0x01,
    0x31, 0x32, 0x33, 0x34,
    0x35, 0x36, 0x37, 0x38,
    0x39, 0x30, 0x21, 0x40,
    0x23, 0x24, 0x25, 0x5e,
    0x26, 0x2a, 0x28, 0x29
];

fn main() {
    logger::init();
    
    let mut devices = Vec::new();

    if net::net_init() == -1 {
        error!("net_init() failure");
        return;
    }

    // TODO: implement dummy
    let mut dummy = driver::dummy::dummy_init();
    devices.push(dummy);

    if net::net_run(&mut devices) == -1 {
        error!("net_run() failure");    
        return;
    }

    let dev_type = 0x800;
    let len = TEST_DATA.len();
    net::net_output(&mut devices, dev_type, TEST_DATA, len);

    net::net_shutdown(&mut devices);
}
