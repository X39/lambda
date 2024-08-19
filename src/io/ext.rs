use std::io::{Read, Write};
use std::mem::size_of;

pub fn write_bool(writer: &mut dyn Write, value: bool) -> Result<usize, std::io::Error> {
    if value {
        write_u8(writer, 1)
    } else {
        write_u8(writer, 0)
    }
}

pub fn write_u8(writer: &mut dyn Write, value: u8) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_u16(writer: &mut dyn Write, value: u16) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_u32(writer: &mut dyn Write, value: u32) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_u64(writer: &mut dyn Write, value: u64) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_i8(writer: &mut dyn Write, value: i8) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_i16(writer: &mut dyn Write, value: i16) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_i32(writer: &mut dyn Write, value: i32) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_i64(writer: &mut dyn Write, value: i64) -> Result<usize, std::io::Error> {
    let para = value.to_le_bytes();
    writer.write(para.as_slice())
}

pub fn write_string(writer: &mut dyn Write, text: &String, index: usize, length: usize) -> Result<usize, std::io::Error> {
    let para = text[index..length].as_bytes();
    writer.write(para)
}


pub fn read_bool(reader: &mut dyn Read) -> Result<bool, std::io::Error> {
    let result = read_u8(reader)?;
    Ok(result > 0)
}

pub fn read_u8(reader: &mut dyn Read) -> Result<u8, std::io::Error> {
    let mut buff: [u8; size_of::<u8>()] = [0; size_of::<u8>()];
    reader.read_exact(&mut buff)?;
    Ok(u8::from_le_bytes(buff))
}

pub fn read_u16(reader: &mut dyn Read) -> Result<u16, std::io::Error> {
    let mut buff: [u8; size_of::<u16>()] = [0; size_of::<u16>()];
    reader.read_exact(&mut buff)?;
    Ok(u16::from_le_bytes(buff))
}

pub fn read_u32(reader: &mut dyn Read) -> Result<u32, std::io::Error> {
    let mut buff: [u8; size_of::<u32>()] = [0; size_of::<u32>()];
    reader.read_exact(&mut buff)?;
    Ok(u32::from_le_bytes(buff))
}

pub fn read_u64(reader: &mut dyn Read) -> Result<u64, std::io::Error> {
    let mut buff: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
    reader.read_exact(&mut buff)?;
    Ok(u64::from_le_bytes(buff))
}

pub fn read_i8(reader: &mut dyn Read) -> Result<i8, std::io::Error> {
    let mut buff: [u8; size_of::<i8>()] = [0; size_of::<i8>()];
    reader.read_exact(&mut buff)?;
    Ok(i8::from_le_bytes(buff))
}

pub fn read_i16(reader: &mut dyn Read) -> Result<i16, std::io::Error> {
    let mut buff: [u8; size_of::<i16>()] = [0; size_of::<i16>()];
    reader.read_exact(&mut buff)?;
    Ok(i16::from_le_bytes(buff))
}

pub fn read_i32(reader: &mut dyn Read) -> Result<i32, std::io::Error> {
    let mut buff: [u8; size_of::<i32>()] = [0; size_of::<i32>()];
    reader.read_exact(&mut buff)?;
    Ok(i32::from_le_bytes(buff))
}

pub fn read_i64(reader: &mut dyn Read) -> Result<i64, std::io::Error> {
    let mut buff: [u8; size_of::<i64>()] = [0; size_of::<i64>()];
    reader.read_exact(&mut buff)?;
    Ok(i64::from_le_bytes(buff))
}

pub fn read_string(reader: &mut dyn Read, length: usize) -> Result<String, std::io::Error> {
    let mut text: String = String::with_capacity(length);
    // ToDo: Find a better way to convert unknown encoding text with as little overhead as possible into rust String to remove unsafe block
    let buff = unsafe { text.as_bytes_mut() };
    reader.read_exact(buff)?;
    Ok(text)
}
