use std::{io, thread, time::Duration};

use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        println!("hello tokio");
        thread::sleep(Duration::from_secs(4));
    });

    tokio::spawn(async {
        println!("hello tokio");
        thread::sleep(Duration::from_secs(4));
    });

    time::sleep(Duration::from_secs(14)).await;
    Ok(())
}