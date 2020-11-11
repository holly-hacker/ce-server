use std::fmt::Debug;

use bytes::Buf;
use log::warn;

use super::{command::*, commands_response::*};
use super::command::Command;

pub trait CERequest : Debug {
    type Response: CEResponse + Debug;

    const ID: Command;
    fn read(buf: &mut dyn Buf) -> Self;
    fn process(self) -> Self::Response;
}

#[derive(Debug)]
pub struct CreateToolHelp32SnapshotRequest {
    pub snapshot_flags: u32,
    pub process_id: u32,
}

impl CERequest for CreateToolHelp32SnapshotRequest {
    type Response = CreateToolHelp32SnapshotResponse;

    const ID: Command = CMD_CREATETOOLHELP32SNAPSHOT;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            snapshot_flags: buf.get_u32_le(),
            process_id: buf.get_u32_le(),
        }
    }

    fn process(self) -> CreateToolHelp32SnapshotResponse {
        warn!("Stubbed CreateToolHelp32SnapshotResponse::process"); // TODO
        CreateToolHelp32SnapshotResponse {
            handle: 0
        }
    }
}

#[derive(Debug)]
pub struct Process32FirstRequest {
    pub handle: u32,
}

impl CERequest for Process32FirstRequest {
    type Response = Process32FirstResponse;

    const ID: Command = CMD_PROCESS32FIRST;

    fn read(buf: &mut dyn Buf) -> Self {
        Self {
            handle: buf.get_u32_le(),
        }
    }

    fn process(self) -> Process32FirstResponse {
        warn!("Stubbed Process32FirstResponse::process"); // TODO
        Process32FirstResponse {
            result: false,
        }
    }
}
