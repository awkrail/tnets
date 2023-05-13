pub fn is_little_endian() -> bool
{
    true
}

pub trait Byteswap 
{
    fn byteswap(&self) -> Self
    where
        Self: Sized;
}

impl Byteswap for u16
{
    fn byteswap(&self) -> u16 {
        (*self & 0x00ff) << 8 | (*self & 0xff00) >> 8
    }
}

impl Byteswap for u32
{
    fn byteswap(&self) -> u32 {
        (*self & 0x000000ff) << 24 | (*self & 0x0000ff00) << 8 
            | (*self & 0x00ff0000) >> 8 | (*self & 0xff000000) >> 24
    }
}

pub fn byteswap<T>(v: T) -> T
where T: Byteswap
{
    v.byteswap()
}

pub fn hton<T: Byteswap>(h: T) -> T
{
    if is_little_endian() {
        byteswap(h)
    } else {
        h
    }
}

pub fn ntoh<T: Byteswap>(n: T) -> T
{
    if is_little_endian() {
        byteswap(n)
    } else {
        n
    }
}

pub fn cksum16() -> u16
{
    0
}
