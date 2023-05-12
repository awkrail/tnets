pub const IFNAMSIZ: u16 = 16;
pub const NET_DEVICE_TYPE_DUMMY: u16     = 0x0000;
pub const NET_DEVICE_TYPE_LOOPBACK: u16  = 0x0001;
pub const NET_DEVICE_TYPE_ETHERNET: u16  = 0x0002;
pub const NET_DEVICE_FLAG_UP: u16        = 0x0001;
pub const NET_DEVICE_FLAG_LOOPBACK: u16  = 0x0010;
pub const NET_DEVICE_FLAG_BROADCAST: u16 = 0x0020;
pub const NET_DEVICE_FLAG_P2P: u16       = 0x0040;
pub const NET_DEVICE_FLAG_NEED_ARP: u16  = 0x0100;
pub const NET_DEVCIE_ADDR_LEN: u16       =     16;

pub struct NetDevice { 
    name: String,
    dev_type: u16,
    mtu: u16,
    flags: u16,
    hlen: u16,
    alen: u16,
    //addr: [u8; NET_DEVICE_ADDR_LEN]
}

impl NetDevice {
    pub fn new(name: String, dev_type: u16, mtu: u16, flags: u16, hlen: u16, alen: u16) -> Self 
    {
        NetDevice {
            name: name,
            dev_type: dev_type,
            mtu: mtu,
            flags: flags,
            hlen: hlen,
            alen: alen,
        }
    }

    fn open(&self) -> i8
    {
        0
    }

    fn close(&self) -> i8
    {
        0
    }

    fn transmit(&self, dev_type: u16, data: [u8; 48], len: usize) -> i8
    {
        if self.dev_type == NET_DEVICE_TYPE_DUMMY {
            debug!("dev={}, type={:04X}, len={}", self.name, dev_type, len);
        }

        0
    }

    fn open_device(&mut self) -> i8 
    {
        if is_up_net_device(self.flags) {
            error!("already opened, dev={}", self.name);
            return -1;
        }
        
        if self.open() == -1 {
            error!("open() failure, dev={}", self.name);
            return -1;
        }

        self.flags |= NET_DEVICE_FLAG_UP;
        info!("dev={}, state={}", self.name, get_state(self.flags));
        0
    }

    fn close_device(&mut self) -> i8
    {
        if !is_up_net_device(self.flags) {
            error!("not opened, dev={}", self.name);
            return -1;
        }
        
        if self.close() == -1 {
            error!("close() failure, dev={}", self.name);
            return -1;
        }

        self.flags &= !NET_DEVICE_FLAG_UP;
        info!("dev={}, state={}", self.name, get_state(self.flags));
        0
    }

    fn output(&self, dev_type: u16, data: [u8; 48], len: usize) -> i8
    {
        if !is_up_net_device(self.flags)  {
            error!("not opened, dev={}", self.name);
            return -1;
        }
        if len > self.mtu.into() {
            error!("too long, dev={}, mtu={}, len={}", self.name, self.mtu, len);
            return -1;
        }
        
        debug!("dev={}, type=0x{:04X}, len={}", self.name, dev_type, len);
        if self.transmit(dev_type, data, len) == -1 {
            error!("transmit() failure, dev={}, len={}", self.name, len);
        }

        0
    }

    fn input(&self, dev_type: u16, data: u8, len: u16) -> i8
    {
        debug!("dev={}, type={}, len={}", self.name, dev_type, len);
        0
    }
}

fn is_up_net_device(flags: u16) -> bool 
{
    flags & NET_DEVICE_FLAG_UP == 1

}

fn get_state(flags: u16) -> String
{
    if is_up_net_device(flags) {
        String::from("up")
    } else {
        String::from("down")
    }
}

pub fn net_run(devices: &mut Vec<NetDevice>) -> i8 
{
    for device in devices.iter_mut() {
        let ret = device.open_device();
        if ret == -1 {
            return -1;
        }
    }
    debug!("running...");
    0
}

pub fn net_shutdown(devices: &mut Vec<NetDevice>) -> i8
{
    for device in devices.iter_mut() {
        let ret = device.close_device();
        if ret == -1 {
            return -1;
        }
    }
    debug!("shutting down...");
    0
}

pub fn net_output(devices: &mut Vec<NetDevice>, dev_type: u16, 
                data: [u8; 48], len: usize) -> i8
{
    for device in devices.iter() {
        let ret = device.output(dev_type, data, len);
        if ret == -1 {
            return -1;
        }
    }
    0
}

pub fn net_init() -> i8
{
    info!("initialized");
    0
}
