mod server;

use log::info;
use simplelog::*;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
        )
        .expect("Could not create terminal logger"),
        #[cfg(feature = "trace")]
        WriteLogger::new(
            log::LevelFilter::Debug,
            Config::default(),
            std::fs::File::create("ce-server.log")
                .expect("Could not create file 'ce-server.log' for logging"),
        ),
    ])
    .expect("Could not create logger");

    info!("Hello, world!");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        server::run::<server::WindowsHandler>().await.unwrap();
    })
}
