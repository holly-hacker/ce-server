use bytes::{Buf, BufMut};

#[derive(Debug)]
pub struct CeProcessEntry {
    pub pid: i32,
    pub process_name: String,
}

#[derive(Debug)]
pub struct CeModuleEntry {
    pub module_base: i64,
    pub module_size: i32,
    pub module_name: String,
}

// NOTE: all handles seem to be 32-bit
pub fn read_usize(buf: &mut dyn Buf) -> usize {
    match std::mem::size_of::<usize>() {
        4 | 8 => buf.get_u32_le() as usize,
        _ => unreachable!(),
    }
}

pub fn write_usize(buf: &mut dyn BufMut, value: usize) {
    match std::mem::size_of::<usize>() {
        4 | 8 => buf.put_u32_le(value as u32),
        _ => unreachable!(),
    }
}

pub fn write_i32_prefixed_string(buf: &mut dyn BufMut, value: String) {
    let bytes = value.into_bytes();

    buf.put_i32_le(bytes.len() as i32);
    buf.put_slice(&bytes[..]);
}

pub fn cstring_to_string(with_zeroes: &[u8]) -> String {
    let len = with_zeroes
        .iter()
        .position(|i| *i == 0)
        .unwrap_or(with_zeroes.len());
    let without_zeroes = &with_zeroes[0..len];
    String::from_utf8(without_zeroes.to_vec()).unwrap()
}

#[test]
fn test_cstring_to_string() {
    assert_eq!(String::from(""), cstring_to_string(b""));
    assert_eq!(String::from(""), cstring_to_string(b"\0"));
    assert_eq!(String::from("abc"), cstring_to_string(b"abc"));
    assert_eq!(String::from("abc"), cstring_to_string(b"abc\0"));
    assert_eq!(String::from("abc"), cstring_to_string(b"abc\0\0abc"));
}
