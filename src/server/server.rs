use bytes::Buf;
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use super::{commands_request::*, commands_response::*, handler::*};

const DEFAULT_PORT: u16 = 52736;

pub async fn run<T: FullHandler + Send>() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("127.0.0.1", DEFAULT_PORT)).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New socket opened: {}", addr);

        tokio::spawn(async move {
            let conn = CheatEngineConnection(socket);
            let handler = T::create();
            conn.run(handler).await;
        });
    }
}

pub struct CheatEngineConnection(TcpStream);

// generics werent powerful enough
macro_rules! gen_request_dispatch {
    ($bytes: ident for $handler: ident, $($request: ty,)*) => {
        match $bytes.get_u8() {
            $(
                <$request>::ID => gen_request_dispatch!($request, $bytes, $handler),
            )*
            byte => {
                error!("Unimplemented packet byte {} (data: {:?})", byte, $bytes);
                todo!("Unimplemented packet byte {}", byte);
            },
        }
    };
    ($request: ty, $bytes: ident, $handler: ident) => {
        {
            let mut buffer = Vec::new();

            let request = <$request>::read(&mut $bytes);

            debug!("Received item {:?}", request);
            let response = $handler.handle(request);
            debug!("... responding with {:?}", response);
            response.serialize(&mut buffer);

            buffer
        }
    };
}

impl CheatEngineConnection {
    pub async fn run(self, handler: impl FullHandler) {
        let mut buf = [0; 1024];

        // In a loop, read data from the socket and write the data back.
        let mut socket = self.0;
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
            let response = Self::handle(request, &handler);

            // Write the data back
            if let Err(e) = socket.write_all(&response[..]).await {
                error!("failed to write to socket; err = {:?}", e);
                return;
            }
        }
    }

    fn handle(mut bytes: &[u8], handler: &impl FullHandler) -> Vec<u8> {
        gen_request_dispatch!(
            bytes for handler,
            CreateToolHelp32SnapshotRequest,
            Process32FirstRequest,
            Process32NextRequest,
            Module32FirstRequest,
            Module32NextRequest,
            CloseHandleRequest,
            GetArchitectureRequest,
            OpenProcessRequest,
            GetSymbolListFromFileRequest,
            ReadProcessMemoryRequest,
            WriteProcessMemoryRequest,
            VirtualQueryExRequest,
            VirtualQueryExFullRequest,
        )
    }
}
