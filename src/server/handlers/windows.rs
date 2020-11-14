use crate::server::{ce_common::CeProcessEntry, commands_request::*, commands_response::*, ce_common::cstring_to_string, handler::*};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First};

pub struct WindowsHandler;

impl FullHandler for WindowsHandler {
    fn create() -> WindowsHandler { WindowsHandler }
}

impl Handler<CreateToolHelp32SnapshotRequest> for WindowsHandler {
    fn handle(&self, req: CreateToolHelp32SnapshotRequest) -> HandleResponse {
        unsafe {
            let ret = CreateToolhelp32Snapshot(req.snapshot_flags, req.process_id);

            HandleResponse {
                handle: ret as usize
            }
        }
    }
}

impl Handler<Process32FirstRequest> for WindowsHandler {
    fn handle(&self, req: Process32FirstRequest) -> Process32Response {
        unsafe {
            let mut entry = std::mem::MaybeUninit::uninit().assume_init();
            let response = Process32First(std::mem::transmute(req.handle), &mut entry);

            if response != 0 {
                Process32Response {
                    entry: Some(
                        CeProcessEntry {
                            pid: entry.th32ProcessID as i32,
                            process_name: cstring_to_string(std::mem::transmute(&entry.szExeFile[..])),
                        }
                    )
                }
            } else {
                Process32Response {
                    entry: None
                }
            }
        }
    }
}
