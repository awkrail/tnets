use crate::net;

const DUMMY_MTU: u16 = u16::MAX;

pub fn dummy_init() -> net::NetDevice
{
    let name = String::from("net0");
    let dev_type = net::NET_DEVICE_TYPE_DUMMY;
    let mtu = DUMMY_MTU;
    let flags = 0;
    let hlen = 0;
    let alen = 0;
    net::NetDevice::new(name, dev_type, mtu, flags, hlen, alen)
}
