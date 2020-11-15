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

#[derive(Debug)]
// TODO: use enums
pub struct RegionInfo {
    pub base_address: u64,
    pub size: u64,
    pub protection: u32,
    pub memory_type: u32,
}

#[derive(Debug)]
#[allow(dead_code)] // only the one matching this arch will be used
pub enum Architecture {
    I386 = 1,
    X86_64 = 2,
    Arm = 3,
    AArch64 = 4,
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

pub fn read_u32_prefixed_string(buf: &mut dyn Buf) -> String {
    let len = buf.get_u32_le();
    let mut v = vec![];
    v.put(buf.take(len as usize));
    unsafe { String::from_utf8_unchecked(v) }
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

pub fn get_process_architecture() -> Architecture {
    #[cfg(target_arch = "x86")]
    {
        Architecture::I386
    }
    #[cfg(target_arch = "x86_64")]
    {
        Architecture::X86_64
    }
    #[cfg(target_arch = "arm")]
    {
        Architecture::Arm
    }
    #[cfg(target_arch = "aarch64")]
    {
        Architecture::AArch64
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
    )))]
    {
        std::compile_error!("Current architecture is not supported by Cheat Engine")
    }
}
