use std::fmt::Debug;

use bytes::Buf;

use super::{ce_common::*, command::*, commands_response::*};

pub trait CERequest: Debug + Send {
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

#[derive(Debug)]
pub struct Process32NextRequest {
    pub handle: usize,
}

impl CERequest for Process32NextRequest {
    type Response = Process32Response;

    const ID: Command = CMD_PROCESS32NEXT;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: read_usize(buf),
        }
    }
}

#[derive(Debug)]
pub struct CloseHandleRequest {
    pub handle: usize,
}

impl CERequest for CloseHandleRequest {
    type Response = I32Response;

    const ID: Command = CMD_CLOSEHANDLE;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: read_usize(buf),
        }
    }
}

#[derive(Debug)]
pub struct Module32FirstRequest {
    pub handle: usize,
}

impl CERequest for Module32FirstRequest {
    type Response = Module32Response;

    const ID: Command = CMD_MODULE32FIRST;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: read_usize(buf),
        }
    }
}

#[derive(Debug)]
pub struct Module32NextRequest {
    pub handle: usize,
}

impl CERequest for Module32NextRequest {
    type Response = Module32Response;

    const ID: Command = CMD_MODULE32NEXT;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: read_usize(buf),
        }
    }
}
