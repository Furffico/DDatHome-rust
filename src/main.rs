use tungstenite::{connect, WebSocket, stream::MaybeTlsStream};
use std::{net::TcpStream};
use std::time::Duration;
use std::thread::sleep;

mod utils;
mod dddhttp;
mod getconfig;
use getconfig::getconfig;
use utils::timeprefix;

fn session(socket:&mut WebSocket<MaybeTlsStream<TcpStream>>)->Result<(),Box<dyn std::error::Error>>{
    socket.write_message("DDDhttp".into())?;

    let msg = socket.read_message()?;
    let task=dddhttp::DDDPack::bind(msg.into_text()?)?;
    match task.execute()?{
        Some(data)=>{
            socket.write_message(data.into())?;
        },
        None=>{},
    }
    Ok(())
}

fn main() {
    let config=getconfig();

    let url=config.geturl();
    println!("[{}] Running client with url: {}",timeprefix(),&url);

    let (mut socket, _)=connect(&url).expect("Can't connect to the server");

    println!("[{}] Connected to the server",timeprefix());
    
    loop {
        sleep(Duration::from_millis(config.interval));
        match session(&mut socket){
            Err(e)=>println!("[{}] Error occurred: {:?}",timeprefix(),e),
            _=>{},
        };
    }
}