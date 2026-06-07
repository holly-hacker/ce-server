mod dummy_data;
mod handle_allocator;

use std::sync::{LazyLock, Mutex};

use log::{info, warn};

use crate::server::{
    ce_common::{
        compress_data, get_process_architecture, CeModuleEntry, CeProcessEntry, ReadMemoryData,
        RegionInfo,
    },
    commands_request::*,
    commands_response::*,
    handler::{FullHandler, Handler},
    handlers::dummy::dummy_data::{DummyProcess, HandleObject, Snapshot, World},
};

const REGION_SIZE: u64 = 0x1000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_IMAGE: u32 = 0x100_0000;

static WORLD: LazyLock<Mutex<World>> = LazyLock::new(|| Mutex::new(World::new()));

pub struct DummyHandler;

impl FullHandler for DummyHandler {
    fn create() -> Self {
        Self
    }
}

impl Handler<CreateToolHelp32SnapshotRequest> for DummyHandler {
    fn handle(&self, req: CreateToolHelp32SnapshotRequest) -> HandleResponse {
        let mut world = WORLD.lock().unwrap();

        // A process_id of 0 captures every process.
        let mut snapshot = Snapshot::default();
        for process in &world.processes {
            if req.process_id != 0 && req.process_id != process.pid as u32 {
                continue;
            }

            for module_idx in 0..process.modules.len() {
                snapshot.modules.push((process.pid, module_idx));
            }
            snapshot.processes.push(process.pid);
        }

        let handle = world.handles.register(HandleObject::Snapshot(snapshot));
        HandleResponse { handle }
    }
}

impl Handler<Process32FirstRequest> for DummyHandler {
    fn handle(&self, req: Process32FirstRequest) -> Process32Response {
        handle_process32(req.handle, false)
    }
}

impl Handler<Process32NextRequest> for DummyHandler {
    fn handle(&self, req: Process32NextRequest) -> Process32Response {
        handle_process32(req.handle, true)
    }
}

fn handle_process32(handle: usize, is_next: bool) -> Process32Response {
    let mut world = WORLD.lock().unwrap();

    let Some(HandleObject::Snapshot(snapshot)) = world.handles.get_mut(handle) else {
        return Process32Response { entry: None };
    };
    snapshot.process_idx = if is_next { snapshot.process_idx + 1 } else { 0 };
    let Some(&pid) = snapshot.processes.get(snapshot.process_idx) else {
        return Process32Response { entry: None };
    };

    Process32Response {
        entry: world.process(pid).map(|process| CeProcessEntry {
            pid: process.pid,
            process_name: process.name.to_string(),
        }),
    }
}

impl Handler<Module32FirstRequest> for DummyHandler {
    fn handle(&self, req: Module32FirstRequest) -> Module32Response {
        handle_module32(req.handle, false)
    }
}

impl Handler<Module32NextRequest> for DummyHandler {
    fn handle(&self, req: Module32NextRequest) -> Module32Response {
        handle_module32(req.handle, true)
    }
}

fn handle_module32(handle: usize, is_next: bool) -> Module32Response {
    let mut world = WORLD.lock().unwrap();

    let Some(HandleObject::Snapshot(snapshot)) = world.handles.get_mut(handle) else {
        return Module32Response { entry: None };
    };
    snapshot.module_idx = if is_next { snapshot.module_idx + 1 } else { 0 };
    let Some(&(pid, module_idx)) = snapshot.modules.get(snapshot.module_idx) else {
        return Module32Response { entry: None };
    };

    Module32Response {
        entry: world
            .process(pid)
            .and_then(|process| process.modules.get(module_idx))
            .map(|module| CeModuleEntry {
                module_base: module.base,
                module_part: 0,
                module_size: module.memory.len() as i32,
                module_file_offset: 0,
                module_name: module.name.to_string(),
            }),
    }
}

impl Handler<CloseHandleRequest> for DummyHandler {
    fn handle(&self, req: CloseHandleRequest) -> I32Response {
        let closed = WORLD.lock().unwrap().handles.close(req.handle).is_some();

        if !closed {
            warn!("CloseHandle called on unknown handle {:#x}", req.handle);
        }

        I32Response {
            response: closed as i32,
        }
    }
}

impl Handler<GetVersionRequest> for DummyHandler {
    fn handle(&self, _req: GetVersionRequest) -> GetVersionResponse {
        GetVersionResponse {
            version: CESERVER_VERSION,
            name: CESERVER_VERSION_STRING.to_string(),
        }
    }
}

impl Handler<GetArchitectureRequest> for DummyHandler {
    fn handle(&self, _req: GetArchitectureRequest) -> ArchitectureResponse {
        ArchitectureResponse {
            response: get_process_architecture(),
        }
    }
}

impl Handler<OpenProcessRequest> for DummyHandler {
    fn handle(&self, req: OpenProcessRequest) -> HandleResponse {
        let mut world = WORLD.lock().unwrap();

        let handle = if world.process(req.pid).is_some() {
            world
                .handles
                .register(HandleObject::Process { pid: req.pid })
        } else {
            warn!("OpenProcess called on unknown pid {}", req.pid);
            0
        };

        HandleResponse { handle }
    }
}

impl Handler<GetSymbolListFromFileRequest> for DummyHandler {
    fn handle(&self, req: GetSymbolListFromFileRequest) -> GetSymbolListFromFileResponse {
        warn!("STUBBED GetSymbolListFromFileRequest({})", req.symbol_path);
        GetSymbolListFromFileResponse
    }
}

impl Handler<ReadProcessMemoryRequest> for DummyHandler {
    fn handle(&self, req: ReadProcessMemoryRequest) -> ReadProcessMemoryResponse {
        let world = WORLD.lock().unwrap();

        let mut buffer = vec![0u8; req.size as usize];
        let bytes_read = resolve_process(&world, req.handle as usize)
            .map(|process| read_into(process, req.address, &mut buffer))
            .unwrap_or(0);

        let data = if req.compression_level != 0 {
            ReadMemoryData::Compressed {
                uncompressed_size: bytes_read as u32,
                compressed_data: compress_data(buffer, req.compression_level),
            }
        } else {
            ReadMemoryData::Uncompressed { data: buffer }
        };

        ReadProcessMemoryResponse { data }
    }
}

impl Handler<WriteProcessMemoryRequest> for DummyHandler {
    fn handle(&self, req: WriteProcessMemoryRequest) -> WriteProcessMemoryResponse {
        let mut world = WORLD.lock().unwrap();

        let written = resolve_process_mut(&mut world, req.handle as usize)
            .map(|process| write_into(process, req.address, &req.data))
            .unwrap_or(0);

        WriteProcessMemoryResponse {
            written: written as u32,
        }
    }
}

impl Handler<VirtualQueryExRequest> for DummyHandler {
    fn handle(&self, req: VirtualQueryExRequest) -> VirtualQueryExResponse {
        let world = WORLD.lock().unwrap();

        let info = resolve_process(&world, req.handle as usize)
            .and_then(|process| query_region(process, req.base_address));

        VirtualQueryExResponse { info }
    }
}

impl Handler<VirtualQueryExFullRequest> for DummyHandler {
    fn handle(&self, req: VirtualQueryExFullRequest) -> VirtualQueryExFullResponse {
        let world = WORLD.lock().unwrap();

        let info = resolve_process(&world, req.handle as usize)
            .map(list_regions)
            .unwrap_or_default();

        VirtualQueryExFullResponse { info }
    }
}

impl Handler<SetConnectionNameRequest> for DummyHandler {
    fn handle(&self, req: SetConnectionNameRequest) -> EmptyResponse {
        info!("Connection name set to {:?}", req.name);
        EmptyResponse
    }
}

fn resolve_process(world: &World, handle: usize) -> Option<&DummyProcess> {
    let &HandleObject::Process { pid } = world.handles.get(handle)? else {
        return None;
    };
    world.process(pid)
}

fn resolve_process_mut(world: &mut World, handle: usize) -> Option<&mut DummyProcess> {
    let &HandleObject::Process { pid } = world.handles.get(handle)? else {
        return None;
    };
    world.process_mut(pid)
}

/// Finds the module containing `address`, returning its index and the offset of
/// `address` into that module's memory.
fn locate(process: &DummyProcess, address: u64) -> Option<(usize, usize)> {
    process
        .modules
        .iter()
        .enumerate()
        .find_map(|(index, module)| {
            let start = module.base as u64;
            let end = start + module.memory.len() as u64;
            (start..end)
                .contains(&address)
                .then(|| (index, (address - start) as usize))
        })
}

fn read_into(process: &DummyProcess, address: u64, buffer: &mut [u8]) -> usize {
    let Some((index, offset)) = locate(process, address) else {
        return 0;
    };

    let source = &process.modules[index].memory[offset..];
    let n = source.len().min(buffer.len());
    buffer[..n].copy_from_slice(&source[..n]);
    n
}

fn write_into(process: &mut DummyProcess, address: u64, data: &[u8]) -> usize {
    let Some((index, offset)) = locate(process, address) else {
        return 0;
    };

    let dest = &mut process.modules[index].memory[offset..];
    let n = dest.len().min(data.len());
    dest[..n].copy_from_slice(&data[..n]);
    n
}

fn query_region(process: &DummyProcess, address: u64) -> Option<RegionInfo> {
    locate(process, address)?;
    Some(region_at(address & !(REGION_SIZE - 1)))
}

fn list_regions(process: &DummyProcess) -> Vec<RegionInfo> {
    process
        .modules
        .iter()
        .flat_map(|module| {
            let base = module.base as u64;
            let pages = (module.memory.len() as u64).div_ceil(REGION_SIZE);
            (0..pages).map(move |page| region_at(base + page * REGION_SIZE))
        })
        .collect()
}

fn region_at(base_address: u64) -> RegionInfo {
    RegionInfo {
        base_address,
        size: REGION_SIZE,
        protection: PAGE_EXECUTE_READWRITE,
        memory_type: MEM_IMAGE,
    }
}
