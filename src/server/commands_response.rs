use bytes::BufMut;

use super::ce_common::*;

pub trait CEResponse {
    fn serialize(self, writer: &mut dyn BufMut);
}

#[derive(Debug)]
pub struct HandleResponse {
    pub handle: usize,
}

impl CEResponse for HandleResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        write_usize(writer, self.handle);
    }
}

#[derive(Debug)]
pub struct Process32Response {
    pub entry: Option<CeProcessEntry>,
}

impl CEResponse for Process32Response {
    fn serialize(self, writer: &mut dyn BufMut) {
        if let Some(entry) = self.entry {
            writer.put_u32_le(1u32);
            writer.put_i32(entry.pid);
            write_i32_prefixed_string(writer, entry.process_name)
        }
        else {
            writer.put_u32_le(0u32);
            writer.put_i32_le(0); // pid
            writer.put_i32_le(0); // process name length
        }
    }
}
