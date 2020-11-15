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
            handle: read_handle(buf),
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
            handle: read_handle(buf),
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
            handle: read_handle(buf),
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
            handle: read_handle(buf),
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
            handle: read_handle(buf),
        }
    }
}

#[derive(Debug)]
pub struct GetArchitectureRequest;

impl CERequest for GetArchitectureRequest {
    type Response = ArchitectureResponse;

    const ID: Command = CMD_GETARCHITECTURE;

    fn read(_buf: &mut dyn Buf) -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct OpenProcessRequest {
    pub pid: i32,
}

impl CERequest for OpenProcessRequest {
    type Response = HandleResponse;

    const ID: Command = CMD_OPENPROCESS;

    fn read(buf: &mut dyn Buf) -> Self {
        OpenProcessRequest {
            pid: buf.get_i32_le(),
        }
    }
}

#[derive(Debug)]
pub struct GetSymbolListFromFileRequest {
    pub symbol_path: String,
}

impl CERequest for GetSymbolListFromFileRequest {
    type Response = GetSymbolListFromFileResponse;

    const ID: Command = CMD_GETSYMBOLLISTFROMFILE;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            symbol_path: read_u32_prefixed_string(buf),
        }
    }
}

#[derive(Debug)]
pub struct ReadProcessMemoryRequest {
    pub handle: u32,
    pub address: u64,
    pub size: u32,
    pub compression_level: u8,
}

impl CERequest for ReadProcessMemoryRequest {
    type Response = ReadProcessMemoryResponse;

    const ID: Command = CMD_READPROCESSMEMORY;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: buf.get_u32_le(),
            address: buf.get_u64_le(),
            size: buf.get_u32_le(),
            compression_level: buf.get_u8(),
        }
    }
}

#[derive(Debug)]
pub struct WriteProcessMemoryRequest {
    pub handle: u32,
    pub address: u64,
    pub data: Vec<u8>,
}

impl CERequest for WriteProcessMemoryRequest {
    type Response = WriteProcessMemoryResponse;

    const ID: Command = CMD_WRITEPROCESSMEMORY;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: buf.get_u32_le(),
            address: buf.get_u64_le(),
            data: {
                let len = buf.get_u32_le();
                Vec::from(buf.take(len as usize).bytes())
            },
        }
    }
}

#[derive(Debug)]
pub struct VirtualQueryExRequest {
    pub handle: i32,
    pub base_address: u64,
}

impl CERequest for VirtualQueryExRequest {
    type Response = VirtualQueryExResponse;

    const ID: Command = CMD_VIRTUALQUERYEX;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: buf.get_i32_le(),
            base_address: buf.get_u64_le(),
        }
    }
}

#[derive(Debug)]
pub struct VirtualQueryExFullRequest {
    pub handle: i32,
    pub flags: u8,
}

impl CERequest for VirtualQueryExFullRequest {
    type Response = VirtualQueryExFullResponse;

    const ID: Command = CMD_VIRTUALQUERYEXFULL;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: buf.get_i32_le(),
            flags: buf.get_u8(),
        }
    }
}
