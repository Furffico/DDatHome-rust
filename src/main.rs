use tungstenite::{connect, WebSocket, stream::MaybeTlsStream};
use std::{net::TcpStream};
use std::time::Duration;
use std::thread::sleep;

mod dddhttp;

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
    let (mut socket, response) =
        connect("wss://cluster.vtbs.moe/?runtime=rust1.61.0&version=0.1&platform=linux&uuid=37ad8ceb-80aa-42b6-b33d-a949edc8924b&name=nagakawa.3").expect("Can't connect to the server");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    
    loop {
        sleep(Duration::from_secs(1));
        match session(&mut socket){
            Err(e)=>println!("Error occurred: {:?}",e),
            _=>{},
        };
    }
}