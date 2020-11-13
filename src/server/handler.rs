use log::warn;

use super::{commands_response::*, commands_request::*};

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

pub struct TestHandler;

impl FullHandler for TestHandler {
    fn create() -> TestHandler { TestHandler }
}

impl Handler<CreateToolHelp32SnapshotRequest> for TestHandler {
    fn handle(&self, _req: CreateToolHelp32SnapshotRequest) -> CreateToolHelp32SnapshotResponse {
        warn!("Stubbed CreateToolHelp32SnapshotResponse::process"); // TODO
        CreateToolHelp32SnapshotResponse {
            handle: 0
        }
    }
}

impl Handler<Process32FirstRequest> for TestHandler {
    fn handle(&self, _req: Process32FirstRequest) -> Process32FirstResponse {
        warn!("Stubbed Process32FirstResponse::process"); // TODO
        Process32FirstResponse {
            result: false,
        }
    }
}
