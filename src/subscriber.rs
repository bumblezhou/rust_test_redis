pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} publisher [channel_name]", args[0]);
        return Ok(());
    }

    let channel_name = &args[2];

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();
    
    pubsub.subscribe(channel_name)?;
    loop {
        let msg = pubsub.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);

        if payload == "exit" || payload == "bye" {
            println!("unsubscribe channel '{}'.", msg.get_channel_name());
            pubsub.unsubscribe(channel_name)?;
            break;
        }
    }

    Ok(())
}