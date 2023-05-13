pub fn is_little_endian() -> bool
{
    true
}

pub fn byteswap(v: T) -> T
{
    // if v is u16
    byteswap16(v);

    // else if v is u32
    byteswap32(v);
}

fn byteswap16(v: u16) -> u16
{
    (v & 0x00ff) << 8 | (v & 0xff00) >> 8
}

fn byteswap32(v: u32) -> u32
{
    (v & 0x000000ff) << 24 | (v & 0x0000ff00 << 8 | (v & 0x00ff0000) >> 8 | (v & 0xff000000) >> 24
}

pub fn hton(h: T) -> T
{
    // if h is u16
    hton16(h);

    // else if v is u32
    hton32(h);
}

fn hton16(h: u16) -> u16
{
    0
}

fn hton32(h: u32) -> u32
{
    0
}

pub fn ntoh(n: T) -> T
{
    // if v is u16
    ntoh16(n);

    // else if v is u32
    ntoh32(n);
}

fn ntoh16(n: u16) -> u16
{
    0
}

fn ntoh32(n: u32) -> u32`
{
    0
}

fn cksum16() -> u16
{
    0
}
