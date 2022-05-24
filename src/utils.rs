use chrono::Local;

pub fn timeprefix()->String{
    let now=Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}