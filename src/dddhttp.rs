use json;
use std::time::Duration;
use std::thread::sleep;
use std::error::Error;
use crate::getconfig::Config;
use reqwest::blocking::{ClientBuilder,Client};
use tungstenite::{connect, WebSocket, stream::MaybeTlsStream};
use std::net::TcpStream;
use chrono::Local;

macro_rules! log {
    ( $($p:expr),* ; $($x:expr),* ) => {
        {
            print!("[{}]",Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            $(
                print!("[{}]",$p);
            )* 
            print!(" ");
            println!($($x),*);
        }
    };
}

enum DDDPack{
    Task { key:String, url:String },
    Wait,
}

impl DDDPack{
    pub fn bind(data:String)->Result<DDDPack,Box<dyn Error>>{
        let value=json::parse(&data)?;
        let tasktype=value["data"]["type"].as_str();
        match tasktype{
            Some("http") => Ok(DDDPack::Task{
                key:value["key"].as_str().ok_or(DDDPackError::KeyError("key `key` not found in taskjson.".to_string()))?.to_string(),
                url:value["data"]["url"].as_str().ok_or(DDDPackError::KeyError("key `data.url` not found in taskjson.".to_string()))?.to_string(),
            }),
            Some("wait") => Ok(DDDPack::Wait{}),
            None=>Err(Box::new(DDDPackError::KeyError("No task type received".to_string()))),
            _ => Err(Box::new(DDDPackError::UnknownTaskError(format!("Unknown type: {}",tasktype.unwrap())))),
        }
    }
}


pub struct DDDClient{
    config: Config,
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    httpclient: Client,
    tasks: Vec<DDDPack>,
}

impl DDDClient{
    pub fn new(config:Config)->Self{
        let url=config.geturl();
        log!(;"Running client with url: {}",&url);

        let (socket, _)=connect(&url).expect("Can't connect to the server");
        log!(;"Connected to the server");

        let dddclient=DDDClient{
            config:config,
            socket:socket,
            httpclient: ClientBuilder::new().use_rustls_tls().build().expect("can't build http client"),
            tasks:vec![]
        };
        dddclient
    }

    pub fn mainloop(&mut self){
        loop {
            sleep(self.config.interval);
            match self.session(){
                Err(e)=>log!(;"Error occurred: {:?}",e),
                _=>{},
            };
        }
    }

    fn session(&mut self)->Result<(),Box<dyn std::error::Error>>{
        self.fetchtask()?;
        match self.process_task()?{
            Some(data)=>{
                self.socket.write_message(data.into())?;
            },
            None=>{},
        }
        Ok(())
    }

    fn fetchtask(&mut self)->Result<(),Box<dyn std::error::Error>>{
        self.socket.write_message("DDDhttp".into())?;
        let msg = self.socket.read_message()?;
        let task=DDDPack::bind(msg.into_text()?)?;
        self.tasks.push(task);
        Ok(())
    }

    fn process_task(&mut self)->Result<Option<String>,Box<dyn Error>>{
        match self.tasks.pop(){
            Some(DDDPack::Task{key,url})=>{
                log!(key;"Processing task: {}",url);
                let result=self.httpclient.get(url).send()?.text()?;
                let response=json::object!{
                    key:&key[..],
                    data:result,
                }.dump();
                log!(key;"Task processed");
                Ok(Some(response))
            },
            Some(DDDPack::Wait)=>{
                log!(;"Sleep for 5 seconds.");
                sleep(Duration::from_secs(5));
                Ok(None)
            },
            None=>Ok(None),
        }
    }
}


#[derive(Debug)]
pub enum DDDPackError{
    UnknownTaskError(String),
    KeyError(String),
}

impl std::error::Error for DDDPackError{
    fn description(&self) -> &str {
        match *self {
            DDDPackError::UnknownTaskError(ref e) => e,
            DDDPackError::KeyError(ref e) => e,
        }
    }
}

impl std::fmt::Display for DDDPackError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DDDPackError::UnknownTaskError(ref e) => write!(f, "UnknownTaskError: {}", e),
            DDDPackError::KeyError(ref e) => write!(f, "KeyError: {}", e),
        }
    }
}