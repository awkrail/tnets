pub struct IrqEntry {
    irq: u16,
    flags: i16,
    name: String,
    dev: NetDevice,
}

impl IrqEntry {
    pub fn new(irq: u16, flags: i16, name: String, dev: NetDevice) -> Self
    {
        IrqEntry {
            irq: u16,
            flags: i16,
            name: String,
            dev: NetDevice
        }
    }
}

fn intr_thread()
{}

pub fn intr_request_irq(irq_entries: &mut Vec<IrqEntry>, irq: u16, flags: i16, name: String, dev: NetDevice)
{
    for irq_entry in irq_entries.iter() {
        if irq_entry.irq == irq {
            if entry.flags ^ INTR_IRQ_SHARED || flags ^ INTR_IRQ_SHARED {
                error!("conflicts with already registered IRQs");
            }
        }
    }

    // TODO: investigate tokio? sigaddset alteranative
    let irq_entry = IrqEntry::new(irq, flags, name, dev);
    irq_entries.push(irq_entry);
}

pub fn intr_raise_irq(irq: u16)
{}

pub intr_run()
{}

pub intr_shutdown()
{}

pub intr_init()
{}
