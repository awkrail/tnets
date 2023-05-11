const IFNAMSIZ: u8 = 16;
const NET_DEVICE_TYPE_DUMMY: u8     = 0x0000;
const NET_DEVICE_TYPE_LOOPBACK: u8  = 0x0001;
const NET_DEVICE_TYPE_ETHERNET: u8  = 0x0002;
const NET_DEVICE_FLAG_UP: u8        = 0x0001;
const NET_DEVICE_FLAG_LOOPBACK: u8  = 0x0010;
const NET_DEVICE_FLAG_BROADCAST: u8 = 0x0020;
const NET_DEVICE_FLAG_P2P: u8       = 0x0040;
const NET_DEVICE_FLAG_NEED_ARP: u8  = 0x0100;
const NET_DEVCIE_ADDR_LEN: u8       =     16;


struct net_device {
    name: &str,
    dev_type: u16,
    mtu: u16,
    flags: u16,
    hlen: u16,
    alen: u16,
    addr: [u8; NET_DEVICE_ADDR_LEN]
}

impl net_device {
    fn new(name: &str, dev_type: u16, mtu: u16, flags: u16,
           hlen: u16, alen: u16, addr: [u8; NET_DEVICE_ADDR_LEN]) -> Self 
    {
        net_device {
            name: name,
            dev_type: dev_type,
            mtu: mtu,
            flags: flags,
            hlen: hlen,
            alen: alen,
            addr: addr
        }
    }

    fn open(&self) {}
    fn close(&self) {}
    fn transmit(&self) {}

    fn is_up_net_device(flags: u16) -> bool 
    {
        flags & NET_DEVICE_FLAG_UP
    }

    fn get_state(flags: u16) -> &str 
    {
        if is_up_net_device(flags) {
            "up"
        } else {
            "down"
        }
    }

    fn open_device(&self) -> i8 
    {
        if is_up_net_device(self.flags) {
            error!("already opened, dev={}", self.name);
            return -1;
        }
        // TODO: impling open

        dev.flags |= NET_DEVICE_FLAG_UP;
        info!("dev={}, state={}", self.name, get_state(self.flags));
        0
    }

    fn close_device(&self) -> i8
    {
        if !is_up_net_device(self.flags) {
            error!("not opened, dev={}", self.name);
            return -1;
        }
        // TODO: impling close
        self.flags &= ~NET_DEVICE_FLAG_UP;
        info!("dev={}, state={}", self.name, get_state(self.flags));
        0
    }

    fn output(&self, dev_type: u16, data: u8, len: usize)
    {
        if !is_up_net_device(self.flags)  {
            error!("not opened, dev={}", dev.name);
            return -1;
        }
        if len > self.mtu {
            error!("too long, dev={}, mtu={}, len={}", dev.name, dev.mtu, len);
            return -1;
        }
        
        debug!("dev={}, type=0x{:04X}, len={}", dev.name, dev_type, len);
        // TODO: transmit
        0
    }

    fn input(&self, dev_type: u16, data: u8)
    {
        debug!("dev={}, type={}, len={}", self.name, dev_type, len);
        0
    }
}

pub fn net_run(devices: Vec<net_device>) -> i8 
{
    for device in devices.iter() {
        device.open_device();
    }
    debug!("running...");
    0
}

pub fn net_shutdown(devices: Vec<net_device>) -> i8
{
    for device in devices.iter() {
        device.close_device();
    }
    debug!("shutting down...");
    0
}

pub fn net_init() -> i8
{
    info!("initialized");
    0
}
