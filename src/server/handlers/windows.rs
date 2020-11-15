use crate::server::{ce_common::*, commands_request::*, commands_response::*, handler::*};
use log::warn;
use winapi::{
    shared::minwindef::{FALSE, LPCVOID, LPVOID},
    um::{
        handleapi::CloseHandle,
        memoryapi::ReadProcessMemory,
        memoryapi::VirtualQueryEx,
        memoryapi::WriteProcessMemory,
        processthreadsapi::OpenProcess,
        tlhelp32::{
            CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
            LPMODULEENTRY32, LPPROCESSENTRY32,
        },
        winnt::MEMORY_BASIC_INFORMATION,
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
    let response = func(handle as HANDLE, &mut entry);

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
    let response = func(handle as HANDLE, &mut entry);

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
            let response = CloseHandle(req.handle as HANDLE);

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

impl Handler<ReadProcessMemoryRequest> for WindowsHandler {
    fn handle(&self, req: ReadProcessMemoryRequest) -> ReadProcessMemoryResponse {
        let mut buffer = vec![0u8; req.size as usize];
        let mut bytes_read = 0;
        let _resp = unsafe {
            ReadProcessMemory(
                req.handle as HANDLE,
                req.address as LPVOID,
                buffer.as_mut_ptr() as LPVOID,
                req.size as usize,
                &mut bytes_read,
            )
        };
        if req.compress {
            todo!()
        } else {
            ReadProcessMemoryResponse { data: buffer }
        }
    }
}

impl Handler<WriteProcessMemoryRequest> for WindowsHandler {
    fn handle(&self, req: WriteProcessMemoryRequest) -> WriteProcessMemoryResponse {
        unsafe {
            let mut written = 0usize;
            let mut data = req.data;
            let success = WriteProcessMemory(
                req.handle as HANDLE,
                req.address as LPVOID,
                data.as_mut_ptr() as LPCVOID,
                data.len(),
                &mut written,
            );

            if success == 0 {
                warn!("Writing memory failed");
            } else if written != data.len() {
                warn!(
                    "Expected to write {} bytes but wrote {}",
                    data.len(),
                    written
                );
            }

            WriteProcessMemoryResponse {
                written: written as u32,
            }
        }
    }
}

impl Handler<VirtualQueryExRequest> for WindowsHandler {
    fn handle(&self, req: VirtualQueryExRequest) -> VirtualQueryExResponse {
        unsafe {
            let mut data =
                std::mem::MaybeUninit::<MEMORY_BASIC_INFORMATION>::uninit().assume_init();
            let size = VirtualQueryEx(
                req.handle as HANDLE,
                req.base_address as LPCVOID,
                &mut data,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );

            if size != 0 {
                VirtualQueryExResponse {
                    info: Some(RegionInfo {
                        protection: data.Protect,
                        memory_type: data.Type,
                        base_address: data.BaseAddress as u64,
                        size: data.RegionSize as u64,
                    }),
                }
            } else {
                VirtualQueryExResponse { info: None }
            }
        }
    }
}

impl Handler<VirtualQueryExFullRequest> for WindowsHandler {
    fn handle(&self, req: VirtualQueryExFullRequest) -> VirtualQueryExFullResponse {
        let mut items = vec![];

        let (_paged_only, _dirty_only, _no_shared) =
            (req.flags | 1 != 0, req.flags | 2 != 0, req.flags | 4 != 0);
        // TODO: use these flags?
        // https://github.com/cheat-engine/cheat-engine/blob/master/Cheat%20Engine/ceserver/api.c#L2822

        let mut address = 0usize;

        loop {
            let option = unsafe {
                let mut data =
                    std::mem::MaybeUninit::<MEMORY_BASIC_INFORMATION>::uninit().assume_init();
                let size = VirtualQueryEx(
                    req.handle as HANDLE,
                    address as LPCVOID,
                    &mut data,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                );

                if size != 0 {
                    address += data.RegionSize;

                    Some(RegionInfo {
                        protection: data.Protect,
                        memory_type: data.Type,
                        base_address: data.BaseAddress as u64,
                        size: data.RegionSize as u64,
                    })
                } else {
                    None
                }
            };

            if let Some(info) = option {
                items.push(info);
            } else {
                break;
            }
        }

        VirtualQueryExFullResponse { info: items }
    }
}
