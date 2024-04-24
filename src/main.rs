//main.rs
pub mod subscriber;
pub mod publisher;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();     
    if args.len() >= 2 {
        match &args[1][..] {
            "subscriber" => return subscriber::main().await,
            "publisher" => return publisher::main().await,
            _ => (),
        }
    }

    println!("usage: {} [subscriber | publisher] [channel] [message]", args[0]);
    Ok(())
}
