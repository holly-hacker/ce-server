use log::{info, debug, error};
use tokio::net::TcpListener;
use tokio::prelude::*;
use bytes::Buf;

use super::{commands_request::*, commands_response::*};

const DEFAULT_PORT: u16 = 52736;

pub struct CheatEngineServer {
    port: u16
}

impl CheatEngineServer {
    pub fn new() -> Self {
        CheatEngineServer::new_port(DEFAULT_PORT)
    }

    pub fn new_port(port: u16) -> Self {
        CheatEngineServer {port}
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(("127.0.0.1", self.port)).await?;

        loop {
            let (mut socket, addr) = listener.accept().await?;
            info!("New socket opened: {}", addr);
    
            tokio::spawn(async move {
                let mut buf = [0; 1024];
    
                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            error!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
    
                    let request = &buf[..n];
                    debug!("incoming data({}): {:?}", request[0], request);
    
                    // Write the data back
                    let response = Self::handle(request);
                    if let Err(e) = socket.write_all(&response[..]).await {
                        error!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }

    fn handle(mut bytes: &[u8]) -> Vec<u8> {
        match bytes.get_u8() {
            CreateToolHelp32SnapshotRequest::ID => Self::handle_packet::<CreateToolHelp32SnapshotRequest>(&mut bytes),
            Process32FirstRequest::ID => Self::handle_packet::<Process32FirstRequest>(&mut bytes),
            _ => todo!(),
        }
    }

    fn handle_packet<TReq: CERequest>(mut bytes: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();

        let request = TReq::read(&mut bytes);
        
        debug!("Received item {:?}", request);
        let response = request.process();
        debug!("... responding with {:?}", response);
        response.serialize(&mut buffer);

        buffer
    }
}
