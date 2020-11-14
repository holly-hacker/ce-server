use std::fmt::Debug;

use bytes::Buf;

use super::{command::*, commands_response::*, ce_common::*};

pub trait CERequest : Debug + Send {
    type Response: CEResponse + Debug + Send;

    const ID: Command;
    fn read(buf: &mut dyn Buf) -> Self;
}

#[derive(Debug)]
pub struct CreateToolHelp32SnapshotRequest {
    pub snapshot_flags: u32,
    pub process_id: u32,
}

impl CERequest for CreateToolHelp32SnapshotRequest {
    type Response = HandleResponse;

    const ID: Command = CMD_CREATETOOLHELP32SNAPSHOT;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            snapshot_flags: buf.get_u32_le(),
            process_id: buf.get_u32_le(),
        }
    }
}

#[derive(Debug)]
pub struct Process32FirstRequest {
    pub handle: usize,
}

impl CERequest for Process32FirstRequest {
    type Response = Process32Response;

    const ID: Command = CMD_PROCESS32FIRST;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: read_usize(buf),
        }
    }
}
