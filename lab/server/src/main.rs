use std::error::Error;
use std::net::Ipv4Addr;

use tokio::net::TcpListener;
use tokio::time::Duration;
use tokio::time::timeout;
use tokio::io::AsyncWriteExt;

use log::info;

use rand::prelude::*;

async fn serve_port(port: u16, message: &str, wait_forever: bool) -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await.unwrap();

    let (mut socket, _) = if wait_forever {
        listener.accept().await?
    } else {
        timeout(Duration::from_millis(300u64), listener.accept()).await??
    };

    socket.write_all(message.as_bytes()).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    flexi_logger::Logger::with_env_or_str("info, luhack_script_lab = debug")
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .format_for_stderr(flexi_logger::colored_detailed_format)
        .start()
        .unwrap();

    let mut rng = rand::thread_rng();
    let range = rand::distributions::Uniform::new(10000, 10500);

    'outer: loop {
        info!("outer loop");
        let mut first = true;
        let mut port = 6969;
        let mut next_port = rng.sample(range);

        for _ in 0..1000 {
            info!("inner loop: {}", port);
            let msg = format!("The next port is: {}", next_port);
            match serve_port(port, &msg, first).await {
                Ok(_) => (),
                Err(_) => continue 'outer,
            };

            first = false;
            port = next_port;
            next_port = rng.sample(range);
        }

        let msg = format!("The flag is: {}", std::env::var("FLAG").unwrap());
        let _ = serve_port(port, &msg, false).await;
    }
}
