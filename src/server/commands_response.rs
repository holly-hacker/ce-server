use bytes::BufMut;

pub trait CEResponse {
    fn serialize(self, writer: &mut dyn BufMut);
}

#[derive(Debug)]
pub struct CreateToolHelp32SnapshotResponse {
    pub handle: u32,
}

impl CEResponse for CreateToolHelp32SnapshotResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        writer.put_u32_le(self.handle);
    }
}

#[derive(Debug)]
pub struct Process32FirstResponse {
    pub result: bool,
    // TODO: process entries!!
}

impl CEResponse for Process32FirstResponse {
    fn serialize(self, writer: &mut dyn BufMut) {
        writer.put_u32_le(self.result as u32);

        if self.result {
            todo!()
        } else {
            writer.put_u32_le(0);
            writer.put_u32_le(0);
        }
    }
}
