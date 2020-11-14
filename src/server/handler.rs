use super::commands_request::*;

pub trait HandlerFactory<T: FullHandler> {}

pub trait FullHandler:
    Handler<CreateToolHelp32SnapshotRequest>
    + Handler<Process32FirstRequest>
    + Handler<Process32NextRequest>
    + Handler<Module32FirstRequest>
    + Handler<Module32NextRequest>
    + Handler<CloseHandleRequest>
    + Handler<GetArchitectureRequest>
    + Handler<OpenProcessRequest>
    + Handler<GetSymbolListFromFileRequest>
    + Handler<ReadProcessMemoryRequest>
    + Handler<WriteProcessMemoryRequest>
{
    fn create() -> Self;
}

pub trait Handler<TReq: CERequest> {
    fn handle(&self, req: TReq) -> TReq::Response;
}
