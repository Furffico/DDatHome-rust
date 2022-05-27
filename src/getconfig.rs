use std::fs::File;
use std::io::{Write,Read};
use std::path::Path;
use uuid::Uuid;
use clap::Parser;
use url::Url;
use std::env;
use std::time::Duration;

pub const VERSION:&str=env!("CARGO_PKG_VERSION");

/// A Rust implimentation of dd@home client
#[derive(Parser, Debug)]
#[clap(author="furffico@github", version=VERSION )]
struct Args {
    /// The path to the config json file. If file not exists, a default config file will be created.
    #[clap(short='c', long="config")]
    config:Option<String>,

    /// The baseurl of the server. [default: wss://cluster.vtbs.moe]
    #[clap(short='b', long="baseurl")]
    baseurl: Option<String>,

    /// The name of the client (for statistics).
    #[clap(short='n', long="name")]
    name: Option<String>,

    /// The uuid of this client (for statistics), randomly generated for default
    #[clap(short='u', long="uuid")]
    uuid:Option<String>,

    /// The interval(ms) between task execution. [default: 500]
    #[clap(short='i', long="interval")]
    interval:Option<u64>,

    /// Count of retries on connection failure, after which the client will exit. Set to 0 for infinite retries. [default: 5]
    #[clap(short='r', long="retry")]
    retry:Option<u8>,
}


pub struct Config {
    pub baseurl:String,
    pub name: String,
    pub uuid: Uuid,
    pub interval: Duration,
    pub retry: u8,
}

pub struct ConfigOpt{
    pub baseurl:Option<String>,
    pub name: Option<String>,
    pub uuid: Option<Uuid>,
    pub interval: Option<Duration>,
    pub retry: Option<u8>,
}

// get config from cli
fn parse_arg(args:Args)->ConfigOpt{
    ConfigOpt{
        baseurl:args.baseurl,
        name:args.name,
        uuid:args.uuid.map_or(None, |u|Uuid::parse_str(&u[..]).map_or(None, |s|Some(s))),
        interval:args.interval.map_or(None, |v|Some(Duration::from_millis(v))),
        retry:args.retry,
    }
}

// get config from env
fn parse_env()->ConfigOpt{
    ConfigOpt{
        baseurl:env::var("BASEURL").ok(),
        name:env::var("NAME").ok(),
        uuid:env::var("UUID").ok().map_or(None, |u|Uuid::parse_str(&u[..]).map_or(None, |s|Some(s))),
        interval:env::var("INTERVAL").ok().map_or(None, |v|Some(Duration::from_millis(v.parse::<u64>().unwrap()))),
        retry:env::var("RETRY").ok().map_or(None, |v|Some(v.parse::<u8>().unwrap())),
    }
}

// get config from json file
fn parse_configfile(path:&str)-> Result<ConfigOpt,Box<dyn std::error::Error>>{
    let mut file=File::open(path)?;
    let mut contents=String::new();
    file.read_to_string(&mut contents)?;
    let value=json::parse(&contents)?;
    Ok(ConfigOpt{
        name:value["name"].as_str().map_or(None, |s:&str|Some(s.to_string())),
        baseurl:value["baseurl"].as_str().map_or(None,|s|Some(s.to_string())),
        uuid:match value["uuid"].as_str(){
            Some("") | None =>None,
            Some(uuid)=>Uuid::parse_str(uuid).map_or(None, |u|Some(u)),
        },
        interval:value["interval"].as_u64().map(|i|Duration::from_millis(i)),
        retry:value["retry"].as_u8(),
    })
}

pub fn getconfig()->Config{
    let args = Args::parse();
    let config_path=args.config.clone();

    // default config 
    let mut config=Config{
        baseurl:"wss://cluster.vtbs.moe".to_string(),
        name:"".to_string(),
        uuid:Uuid::new_v4(),
        interval:Duration::from_millis(500),
        retry:5,
    };

    // from env
    config.update(parse_env());
    
    // from config.json
    if config_path.is_some(){
        let path=config_path.unwrap();
        if Path::new(&path).is_file(){
            match parse_configfile(&path){
                Ok(cfg)=>config.update(cfg),
                Err(e)=>println!("Error occurred while parsing config file: {}\nENV of default config will be used.",e),
            }
        }else{
            // generate default config and exit
            let mut file = File::create(&path).expect(format!("Failed to create `{}`",&path).as_str());
            file.write_all(json::stringify_pretty(json::object!{
                name:"",
                baseurl:"wss://cluster.vtbs.moe",
                uuid:Uuid::new_v4().to_string(),
                interval:500i32,
                retry:5i32,
            },4).as_bytes()).expect(format!("Failed to write to `{}`",&path).as_str());
            println!("Created a default config file at `{}`",path);
            std::process::exit(0);
        }
    }

    // from cil
    config.update(parse_arg(args));
    
    config
}

impl Config{
    pub fn geturl(&self)->String{
        let mut url=Url::parse(self.baseurl.as_str()).expect("baseurl is not a valid url");
        if url.scheme()!="wss" && url.scheme()!="ws" {
            panic!("baseurl is not a ws or wss url");
        }
        url.set_query(Some(
            &format!("runtime=rust&platform={}&version={}&name={}&uuid={}{}",
            env::consts::OS,VERSION,&self.name,&self.uuid.to_string(),
            match std::env::var("DOCKER"){
                Ok(value)=>if value=="docker"{"&docker=docker"}else{""},
                _=> ""
            }
        )));
        url.to_string()
    }

    pub fn update(&mut self,cfg:ConfigOpt){
        if cfg.baseurl.is_some(){ self.baseurl=cfg.baseurl.unwrap() };
        if cfg.name.is_some(){ self.name=cfg.name.unwrap() };
        if cfg.uuid.is_some(){ self.uuid=cfg.uuid.unwrap() };
        if cfg.interval.is_some(){ self.interval=cfg.interval.unwrap() };
        if cfg.retry.is_some(){ self.retry=cfg.retry.unwrap() };
    }
}