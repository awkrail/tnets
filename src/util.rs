use chrono::Local;
use env_logger::{fmt::Color, Builder};
use log::{Level, trace, debug, info, warn, error};
use std::io::Write;

pub fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|buf, record| {
        let level_color = match record.level() {
            Level::Trace => Color::White,
            Level::Debug => Color::Blue,
            Level::Info => Color::Green,
            Level::Warn => Color::Yellow,
            Level::Error => Color::Red,
        };
        let mut level_style = buf.style();
        level_style.set_color(level_color);
        
        writeln!(
            buf,
            "{file}:{line} {level} [{time}] - {args}",
            file = level_style.value(record.file().unwrap_or("unknown")),
            line = level_style.value(record.line().unwrap_or(0)),
            level = level_style.value(record.level()),
            time = level_style.value(Local::now().format("%Y-%m-%dT%H:%M:%S")),
            args = level_style.value(record.args()),
        )
    });
    builder.filter(None, log::LevelFilter::Trace);
    builder.write_style(env_logger::WriteStyle::Auto);
    builder.init();
}

// byte order
pub fn is_little_endian() -> bool
{
    let x: u32 = 0x00000001;
    let bytes = x.to_le_bytes();
    bytes[0] == 1
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
