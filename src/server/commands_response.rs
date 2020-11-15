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
            writer.put_i32_le(entry.pid);
            write_i32_prefixed_string(writer, entry.process_name)
        } else {
            writer.put_u32_le(0u32);
            writer.put_i32_le(0); // pid
            writer.put_i32_le(0); // process name length
        }
    }
}

#[derive(Debug)]
pub struct Module32Response {
    pub entry: Option<CeModuleEntry>,
}

impl CEResponse for Module32Response {
    fn serialize(self, writer: &mut dyn BufMut) {
        if let Some(entry) = self.entry {
            writer.put_u32_le(1u32);
            writer.put_i64_le(entry.module_base);
            writer.put_i32_le(entry.module_size);
            write_i32_prefixed_string(writer, entry.module_name)
        } else {
            writer.put_u32_le(0u32);
            writer.put_i64_le(0); // base
            writer.put_i32_le(0); // size
            writer.put_i32_le(0); // module name length
        }
    }
}

#[derive(Debug)]
pub struct I32Response {
    pub response: i32,
}

impl CEResponse for I32Response {
    fn serialize(self, writer: &mut dyn BufMut) {
        writer.put_i32_le(self.response);
    }
}

#[derive(Debug)]
pub struct ArchitectureResponse {
    pub response: Architecture,
}

impl CEResponse for ArchitectureResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        writer.put_u8(self.response as u8);
    }
}

#[derive(Debug)]
pub struct GetSymbolListFromFileResponse; // TODO implement

impl CEResponse for GetSymbolListFromFileResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        // writing 0 for now
        writer.put_i32_le(0);
    }
}

#[derive(Debug)]
pub struct ReadProcessMemoryResponse {
    pub data: ReadMemoryData,
}

impl CEResponse for ReadProcessMemoryResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        match self.data {
            ReadMemoryData::Uncompressed { data } => {
                writer.put_i32_le(data.len() as i32);
                writer.put_slice(&data[..]);
            }
            ReadMemoryData::Compressed {
                uncompressed_size,
                compressed_data,
            } => {
                writer.put_u32_le(uncompressed_size);
                writer.put_u32_le(compressed_data.len() as u32);
                writer.put_slice(&compressed_data[..]);
            }
        }
    }
}

#[derive(Debug)]
pub struct WriteProcessMemoryResponse {
    pub written: u32,
}

impl CEResponse for WriteProcessMemoryResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        writer.put_u32_le(self.written);
    }
}

#[derive(Debug)]
pub struct VirtualQueryExResponse {
    pub info: Option<RegionInfo>,
}

impl CEResponse for VirtualQueryExResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        if let Some(info) = self.info {
            writer.put_u8(1);
            writer.put_u32_le(info.protection);
            writer.put_u32_le(info.memory_type);
            writer.put_u64_le(info.base_address);
            writer.put_u64_le(info.size);
        } else {
            writer.put_u8(0);
            writer.put_u32_le(0);
            writer.put_u32_le(0);
            writer.put_u64_le(0);
            writer.put_u64_le(0);
        }
    }
}

#[derive(Debug)]
pub struct VirtualQueryExFullResponse {
    pub info: Vec<RegionInfo>,
}

impl CEResponse for VirtualQueryExFullResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        writer.put_u32_le(self.info.len() as u32);
        for region in self.info {
            // Sends RegionInfo struct, not VirtualQueryExFullResponse
            writer.put_u64_le(region.base_address);
            writer.put_u64_le(region.size);
            writer.put_u32_le(region.protection);
            writer.put_u32_le(region.memory_type);
        }
    }
}
