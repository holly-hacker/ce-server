mod server;

use log::info;

fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug).init();
    info!("Hello, world!");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let server = server::CheatEngineServer::new();
        server.run().await.unwrap();
    })
}

