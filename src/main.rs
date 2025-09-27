use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown;
use std::fs;
use serde::Deserialize;
use rand::Rng;


fn handle(mut stream: TcpStream)  -> std::io::Result<()> {
    let data = include_str!("quote.json");
    let data_json: serde_json::Value =serde_json::from_str(&data)?;
    let quotes = data_json["quotes"].clone();
    let num = rand::thread_rng().gen_range(0..quotes.as_array().unwrap().len());
    let quote = quotes[num].as_str().unwrap();
    stream.write_all(quote.as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:19")?;



    for stream in listener.incoming() {

        let mut stream = stream?;
        let peer = stream.peer_addr()?;
        println!("[NEWCON] {:?}", peer);

        handle(stream);
    }

    Ok(())
}


