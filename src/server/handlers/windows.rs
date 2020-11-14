use crate::server::{ce_common::*, commands_request::*, commands_response::*, handler::*};
use log::warn;
use winapi::{
    shared::minwindef::FALSE,
    um::{
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        tlhelp32::{
            CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
            LPMODULEENTRY32, LPPROCESSENTRY32,
        },
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
    },
};

pub struct WindowsHandler;

impl FullHandler for WindowsHandler {
    fn create() -> WindowsHandler {
        WindowsHandler
    }
}

impl Handler<CreateToolHelp32SnapshotRequest> for WindowsHandler {
    fn handle(&self, req: CreateToolHelp32SnapshotRequest) -> HandleResponse {
        unsafe {
            let ret = CreateToolhelp32Snapshot(req.snapshot_flags, req.process_id);

            HandleResponse {
                handle: ret as usize,
            }
        }
    }
}

impl Handler<Process32FirstRequest> for WindowsHandler {
    fn handle(&self, req: Process32FirstRequest) -> Process32Response {
        unsafe { get_process_response(req.handle, |x, y| Process32First(x, y)) }
    }
}

impl Handler<Process32NextRequest> for WindowsHandler {
    fn handle(&self, req: Process32NextRequest) -> Process32Response {
        unsafe { get_process_response(req.handle, |x, y| Process32Next(x, y)) }
    }
}

unsafe fn get_process_response<F>(handle: usize, func: F) -> Process32Response
where
    F: FnOnce(HANDLE, LPPROCESSENTRY32) -> i32,
{
    let mut entry = std::mem::MaybeUninit::uninit().assume_init();
    let response = func(std::mem::transmute(handle), &mut entry);

    if response != 0 {
        Process32Response {
            entry: Some(CeProcessEntry {
                pid: entry.th32ProcessID as i32,
                process_name: cstring_to_string(std::mem::transmute(&entry.szExeFile[..])),
            }),
        }
    } else {
        Process32Response { entry: None }
    }
}

impl Handler<Module32FirstRequest> for WindowsHandler {
    fn handle(&self, req: Module32FirstRequest) -> Module32Response {
        unsafe { get_module_response(req.handle, |x, y| Module32First(x, y)) }
    }
}

impl Handler<Module32NextRequest> for WindowsHandler {
    fn handle(&self, req: Module32NextRequest) -> Module32Response {
        unsafe { get_module_response(req.handle, |x, y| Module32Next(x, y)) }
    }
}

unsafe fn get_module_response<F>(handle: usize, func: F) -> Module32Response
where
    F: FnOnce(HANDLE, LPMODULEENTRY32) -> i32,
{
    let mut entry = std::mem::MaybeUninit::uninit().assume_init();
    let response = func(std::mem::transmute(handle), &mut entry);

    if response != 0 {
        Module32Response {
            entry: Some(CeModuleEntry {
                module_base: entry.modBaseAddr as i64,
                module_size: entry.modBaseSize as i32,
                module_name: cstring_to_string(std::mem::transmute(&entry.szModule[..])),
            }),
        }
    } else {
        Module32Response { entry: None }
    }
}

impl Handler<CloseHandleRequest> for WindowsHandler {
    fn handle(&self, req: CloseHandleRequest) -> I32Response {
        unsafe {
            let response = CloseHandle(std::mem::transmute(req.handle));

            I32Response { response }
        }
    }
}

impl Handler<GetArchitectureRequest> for WindowsHandler {
    fn handle(&self, _eq: GetArchitectureRequest) -> ArchitectureResponse {
        ArchitectureResponse {
            response: get_process_architecture(),
        }
    }
}

impl Handler<OpenProcessRequest> for WindowsHandler {
    fn handle(&self, req: OpenProcessRequest) -> HandleResponse {
        unsafe {
            let resp = OpenProcess(PROCESS_ALL_ACCESS, FALSE, req.pid as u32);
            HandleResponse {
                handle: resp as usize,
            }
        }
    }
}

impl Handler<GetSymbolListFromFileRequest> for WindowsHandler {
    fn handle(&self, req: GetSymbolListFromFileRequest) -> GetSymbolListFromFileResponse {
        // TODO: see CheatEngine Server GetSymbolListFromFile
        // https://github.com/cheat-engine/cheat-engine/blob/master/Cheat%20Engine/ceserver/symbols.c
        warn!("STUBBED GetSymbolListFromFileRequest({})", req.symbol_path);
        GetSymbolListFromFileResponse
    }
}
