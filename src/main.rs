mod dddhttp;
mod getconfig;

fn main() {
    let config=getconfig::getconfig();
    let mut client=dddhttp::DDDClient::new(config);
    client.mainloop();
}