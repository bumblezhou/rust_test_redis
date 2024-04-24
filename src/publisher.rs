use redis::Commands;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 4 {
        println!("usage: {} publisher [channel_name] [message]", args[0]);
        return Ok(());
    }

    let channel_name = &args[2];
    let message = &args[3];

    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let mut pub_conn = client.get_connection().unwrap();

    pub_conn.publish(channel_name, message)?;

    Ok(())
}