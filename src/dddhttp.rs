use json;
use std::time::Duration;
use std::thread::sleep;
use std::error::Error;
use reqwest::blocking::get;
use crate::utils::timeprefix;

pub enum DDDPack{
    Task { key:String, url:String },
    Wait { key:String },
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
            Some("wait") => Ok(DDDPack::Wait{
                key:value["key"].as_str().unwrap_or("").to_string(),
            }),
            None=>Err(Box::new(DDDPackError::UnknownTaskError("No task type received".to_string()))),
            _ => Err(Box::new(DDDPackError::UnknownTaskError(format!("Unknown type: {}",tasktype.unwrap())))),
        }
    }
    pub fn execute(&self)->Result<Option<String>,Box<dyn Error>>{
        match self{
            DDDPack::Task{key,url}=>{
                println!("[{}][{}] Executing task: {}",timeprefix(),key,url);
                let result=get(url)?.text()?;
                let response=json::object!{
                    key:&key[..],
                    data:result,
                }.dump();
                println!("[{}][{}] Task executed.",timeprefix(),key);
                Ok(Some(response))
            },
            DDDPack::Wait{key:_}=>{
                println!("[{}] Sleep for 5 seconds.",timeprefix());
                sleep(Duration::from_secs(5));
                Ok(None)
            },
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