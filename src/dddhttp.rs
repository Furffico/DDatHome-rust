use json;
use std::time::Duration;
use std::thread::sleep;
use std::error::Error;

#[derive(Debug)]
pub enum DDDPackError{
    UnknownTaskError(String),
}

#[derive(Debug,PartialEq)]
pub enum DDDPack{
    Task { key:String, url:String },
    Wait { key:String },
}

impl Error for DDDPackError{
    fn description(&self) -> &str {
        match *self {
            DDDPackError::UnknownTaskError(ref e) => e,
        }
    }
}

impl std::fmt::Display for DDDPackError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DDDPackError::UnknownTaskError(ref e) => write!(f, "UnknownTaskError: {}", e),
        }
    }
}

impl DDDPack{
    pub fn bind(data:String)->Result<DDDPack,Box<dyn Error>>{
        let value=json::parse(&data)?;
        let tasktype=value["data"]["type"].as_str();
        match tasktype{
            Some("http") => Ok(DDDPack::Task{
                key:value["key"].as_str().unwrap().to_string(),
                url:value["data"]["url"].as_str().unwrap().to_string(),
            }),
            Some("wait") => Ok(DDDPack::Wait{
                key:value["key"].as_str().unwrap().to_string(),
            }),
            None=>Err(Box::new(DDDPackError::UnknownTaskError("No task type received".to_string()))),
            _ => Err(Box::new(DDDPackError::UnknownTaskError(format!("Unknown type: {}",tasktype.unwrap())))),
        }
    }
    pub fn execute(&self)->Result<Option<String>,Box<dyn Error>>{
        match self{
            DDDPack::Task{key,url}=>{
                println!("[{}] Executing task: {}",key,url);
                let result=httpfetch(url)?;
                let response=json::object!{
                    key:&key[..],
                    data:result,
                }.dump();
                println!("[{}] Task executed with response: {}",key,&response[..std::cmp::min(100, response.len()-1)]);
                Ok(Some(response))
            },
            DDDPack::Wait{key:_}=>{
                println!("Sleep for 10 seconds.");
                sleep(Duration::from_secs(4));
                Ok(None)
            },
        }
    }
}


fn httpfetch(url:&str)->Result<String, Box<dyn Error>>{
    let response=reqwest::blocking::get(url)?.text()?;
    Ok(response)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_dddpack(){
        let data="{\"type\":\"task\",\"key\":\"test\",\"url\":\"http://www.baidu.com\"}";
        let pack=DDDPack::bind(data.to_string()).unwrap();
        assert_eq!(pack,DDDPack::Task{key:"test".to_string(),url:"http://www.baidu.com".to_string()});

        let data="{\"type\":\"wait\",\"key\":\"test\"}";
        let pack=DDDPack::bind(data.to_string()).unwrap();
        assert_eq!(pack,DDDPack::Wait{key:"test".to_string()});
    }
    
    #[test]
    fn test_fetchurl(){
        println!("{:?}",httpfetch("https://api.bilibili.com/x/space/acc/info?mid=1473830"));
    }

    #[test]
    fn test_execute_task(){
        let data="{\"type\":\"task\",\"key\":\"test2\",\"url\":\"https://api.bilibili.com/x/space/acc/info?mid=198297\"}";
        let task=DDDPack::bind(data.to_string()).unwrap();
        let result=task.execute().unwrap().unwrap();
        println!("{:?}",result)
    }
}