use embedded_io::asynch::{Read, Write};
use embedded_svc::executor::asynch::Blocker;
use embedded_svc::mutex::StdRawCondvar;
use embedded_svc::utils::asynch::executor::embedded::{CondvarWait, EmbeddedBlocker};

use embedded_svc_impl::asynch::{stdnal::StdTcpClientSocket, tcp::TcpClientSocket};

fn main() {
    let wait = CondvarWait::<StdRawCondvar>::new();

    EmbeddedBlocker::new(wait.notify_factory(), wait)
        .block_on(read())
        .unwrap();
}

async fn read() -> anyhow::Result<()> {
    println!("About to open a TCP connection to 1.1.1.1 port 80");

    let mut socket = StdTcpClientSocket::new();

    socket.connect("1.1.1.1:80".parse().unwrap()).await?;

    socket.write_all("GET / HTTP/1.0\n\n".as_bytes()).await?;

    let mut result = Vec::new();

    let mut buf = [0_u8; 1024];

    loop {
        let len = socket.read(&mut buf).await?;

        if len > 0 {
            result.extend_from_slice(&buf[0..len]);
        } else {
            break;
        }
    }

    println!(
        "1.1.1.1 returned:\n=================\n{}\n=================\nSince it returned something, all seems OK!",
        std::str::from_utf8(&result)?);

    Ok(())
}
