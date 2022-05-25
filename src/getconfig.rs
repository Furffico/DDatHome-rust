use std::fs::File;
use std::io::Read;
use std::path::Path;
use uuid::Uuid;
use clap::Parser;
use url::Url;
use std::env;
use std::time::Duration;

pub const VERSION:&str="0.1.1";

/// A Rust implimentation of dd@home client
#[derive(Parser, Debug)]
#[clap(author="furffico@github", version=VERSION )]
struct Args {
    /// The baseurl of the server
    #[clap(short='b', long="baseurl", default_value="wss://cluster.vtbs.moe")]
    baseurl: String,

    /// The name of the client (for statistics)
    #[clap(short='n', long="name", default_value="")]
    name: String,

    /// The uuid of this client (for statistics)
    #[clap(short='u', long="uuid",default_value="random generated")]
    uuid:String,

    /// The interval(ms) between task execution
    #[clap(short='i', long="interval",default_value_t=500)]
    interval:u64,
}


#[derive(Debug)]
pub struct Config {
    pub baseurl:String,
    pub name: String,
    pub uuid: Uuid,
    pub interval: Duration,
}

fn parse_configfile(path:&str,config:&mut Config)-> Result<(),Box<dyn std::error::Error>>{
    let mut file=File::open(path)?;
    let mut contents=String::new();
    file.read_to_string(&mut contents)?;
    let value=json::parse(&contents)?;

    match value["name"].as_str(){
        Some("") | None =>{},
        Some(name)=>config.name=name.to_string(),
    };
    match value["baseurl"].as_str(){
        Some("") | None =>{},
        Some(baseurl)=>config.baseurl=baseurl.to_string(),
    };
    match value["interval"].as_u64(){
        Some(interval)=>config.interval=Duration::from_millis(interval),
        None=>{},
    };
    match value["uuid"].as_str(){
        Some("") | None =>{},
        Some(uuid)=>config.uuid=Uuid::parse_str(uuid)?,
    };
    Ok(())
}

pub fn getconfig()->Config{
    // get config from cli
    let args = Args::parse();
    let mut config=Config{
        baseurl:args.baseurl,
        name:args.name,
        uuid:if args.uuid.len()==36{Uuid::parse_str(&args.uuid).unwrap_or(Uuid::new_v4())}else{Uuid::new_v4()},
        interval:Duration::from_millis(args.interval),
    };

    // get config from json
    let cfgpath="./config.json";
    if Path::new(cfgpath).is_file(){
        match parse_configfile(cfgpath,&mut config){
            Ok(_)=>return config,
            _=>{},
        };
    }

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
}