const DUMMY_MTU: u16 = u16::MAX;

pub fn dummy_init()
{
    let dummy_device = NetDevice {
        dev_type: NET_DEVICE_TYPE_DUMMY,
        mtu: DUMMY_MTU,
        hlen: 0,
        alen: 0,
    };
    dummy_device
}
