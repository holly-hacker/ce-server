use super::commands_request::*;

pub trait HandlerFactory<T: FullHandler> {
}

pub trait FullHandler
    : Handler<CreateToolHelp32SnapshotRequest>
    + Handler<Process32FirstRequest> {
    fn create() -> Self;
}

pub trait Handler<TReq: CERequest> {
    fn handle(&self, req: TReq) -> TReq::Response;
}
