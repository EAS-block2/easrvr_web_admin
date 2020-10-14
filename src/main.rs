use serde_yaml;
use std::{io::Read, fs::File};
use std::io::prelude::*;
use std::net::TcpStream;
use serde::Deserialize;
use std::str;

fn main(){
    let conf_f = std::fs::File::open("/etc/EAS/reseter_conf.yaml").expect("can't find config");
    let config: Config;
    match serde_yaml::from_reader(conf_f){
        Ok(e) => {config = e;}
        Err(_)=> {config = Config{addr: 0.to_string()};
            render_error("bad YAML config, contact administrator!");}
    }
    let main = "admin".to_string();
    if read_arg() == main{let fault_status: Vec<String>;
        let general_status: Vec<String>;
        let silent_status: Vec<String>;
        match read_status("/etc/EAS/faulted.yaml"){
            Ok(out) => {fault_status = out}
            Err(_) => {fault_status = vec!("Cannot get Fault list".to_string())}
        }
        match read_status("/etc/EAS/gL.yaml"){
            Ok(out) => {general_status = out}
            Err(_) => {general_status = vec!("Cannot get General status".to_string())}
        }
        match read_status("/etc/EAS/sL.yaml"){
            Ok(out) => {silent_status = out}
            Err(_) => {silent_status = vec!("Cannot get Silent status".to_string())}
        }
        render_page(fault_status, general_status, silent_status);}

        else {let arg = read_arg();
          match communicate(config, &arg){
          Ok(_) => {if arg.contains("set"){render_page_s()}
                  else{render_page_r()}}
          Err(e) => {render_error(&e.to_string());}
      }
    }
}
#[derive(Deserialize)]
struct Config{
    addr: String
}
fn render_page_r() {
    println!("Content-type:text/html\r\n\r\n");
    println!("<html>
    \n <head>
    \n <meta charset=\"utf-8\">
    \n <title>EAS | Reset Sequence</title>
    \n <meta http-equiv=\"refresh\" content=\"6; URL=/cgi-bin/main?admin\">
    \n </head>
    \n <body>
    \n <h1 style=\"text-align: center;\">Rest Complete</h1>
    \n <h3 style=\"text-align: center;\">Redirecting after 6 seconds...</h3>
    \n <p>Please allow up to 12 seconds for the alarm to fully deactivate</p>");
}
fn render_page_s() {
    println!("Content-type:text/html\r\n\r\n");
    println!("<html>
    \n <head>
    \n <meta charset=\"utf-8\">
    \n <title>EAS | Activation Sequence</title>
    \n <meta http-equiv=\"refresh\" content=\"4; URL=/cgi-bin/main?admin\">
    \n </head>
    \n <body>
    \n <h1 style=\"text-align: center;\">Alarm Activation Complete</h1>
    \n <h3 style=\"text-align: center;\">Redirecting after 4 seconds...</h3>
    \n <p>Please allow up to 8 seconds for the alarm to fully activate</p>");
}
fn render_error(reason: &str) {
    println!("Content-type:text/html\n\n");
    println!("<html>
    \n <head>
    \n <meta charset=\"utf-8\">
    \n <title>Alarm Control Sequence</title>
    \n </head>
    \n <body>
    \n <h1 style=\"text-align: center;\">Alarm Update Failed!</h1>
    \n <h3>The reset process has failed due to: {}</h3>
    \n <button onclick=\"window.location.href=\'/cgi-bin/main?admin\'\">Go Back</button>
    \n </body>", reason);
    panic!("something went wrong and we told the user about it");
}
fn communicate (conf: Config, command: &String) -> std::io::Result<()>{
    let mut stream = TcpStream::connect(&conf.addr)?;
    println!("Addr: {}, Stream: {:?}",&conf.addr,&stream);
        let to_send = command.clone().into_bytes();
        stream.write(to_send.as_slice())?;
        let mut data = [0 as u8; 50];
        let size = stream.read(&mut data)?;
        match str::from_utf8(&data[0..size]){
        Ok(string_out) => {
            match string_out{
            "ok" => {return Ok(());}
            _ => {render_error("The Ops server refused the command");
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Oh no"));}
        }}
        Err(_)=>{return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Oh no"));}}
}
fn read_arg() -> String{
let mut retn = "bad".to_string();
let acceptable = vec!("admin".to_string(), "fclear".to_string(), "gclear".to_string(), "sclear".to_string(),
    "aclear".to_string(), "gset".to_string(), "sset".to_string());
match std::env::args().nth(1){
    Some(arg) => {
        if acceptable.contains(&arg){retn = arg}
        else {render_error("bad client argument");}}
        None => {render_error("bad client argument");}
}
retn
}

fn render_page(faults: Vec<String>, gen: Vec<String>, sil: Vec<String>) {
    let mut gcolor = "#aaa";
    let mut scolor = "#aaa";
    if !gen.is_empty(){gcolor = "#b53434"}
    if !sil.is_empty(){scolor = "#b53434"}
println!("Content-type:text/html\n\n");
println!("<html>
<head>
\n <title>EAS | main</title>
\n <meta charset=\"utf-8\">
<meta Cache-Control=\"Cache-Control: no-cache, no-store, must-revalidate\">
<meta http-equiv=\"refresh\" content=\"4\"/>
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
<style>
* {{
  box-sizing: border-box;
}}
.row {{
  display: flex;
}}
.column {{
  flex: 50%;
  padding: 10px;
}}
.container {{ 
    height: 200px;
    position: relative;
    border: 3px solid black; 
  }}
  .center {{
    margin: 0;
    position: absolute;
    top: 50%;
    left: 50%;
    -ms-transform: translate(-50%, -50%);
    transform: translate(-50%, -50%);
  }}
  .bcenter {{
    margin: 5;
    position: absolute;
    left: 50%;
    -ms-transform: translate(-50%, -50%);
    transform: translate(-50%, -50%);
  }}
</style>
</head>
<body>
<p>Ellingtech Emergency Alert System Version Alpha-3</p>
<h1 style=\"text-align: center;\">Alarm Control Interface</h1>
<p>&nbsp;</p>
<p>&nbsp;</p>");
if !gen.is_empty() || !sil.is_empty() || !faults.is_empty(){
println!("<div class=\"bcenter\">
<button style=\"color:#60aa47;\" onclick=\"window.location.href=\'/cgi-bin/main?aclear\'\">
Reset All</button>
</div>");}

println!("
<div class=\"row\">
  <div class=\"column\" style=\"background-color:{};\">
    <h2 style=\"text-align: center;\">General Alarm Status</h2>",gcolor);
    if gen.is_empty(){println!("<p>The General Alarm reports all clear and nominal</p>
    \n <p>&nbsp;</p> \n<button style=\"color:#b53434;\"
    onclick=\"window.location.href=\'/cgi-bin/main?gset\'\">OVERRIDE: Activate
    </button>");}
    else{
    println!("<h3>Alarm active, activated from:</h3>");
    for i in gen.iter(){println!("<li>{}</li>",i);}
    println!("\n <p>&nbsp;</p> \n<button
    onclick=\"window.location.href=\'/cgi-bin/main?gclear\'\">Reset
    </button>");}
  println!("</div>
  <div class=\"column\" style=\"background-color:{};\">
    <h2 style=\"text-align: center;\">Silent Alarm Status</h2>", scolor);
    if sil.is_empty(){println!("<p>The Silent Alarm reports all clear and nominal</p>");
    println!("\n <p>&nbsp;</p> \n<button style=\"color:#b53434;\"
    onclick=\"window.location.href=\'/cgi-bin/main?sset\'\">OVERRIDE: Activate
    </button>");}
    else{
    println!("<h3>Alarm active, activated from:</h3>");
    for i in sil.iter(){println!("<li>{}</li>",i);}
    println!("\n <p>&nbsp;</p> \n<button
    onclick=\"window.location.href=\'/cgi-bin/main?sclear\'\">Reset
    </button>");}
println!("</div></div>");
if !faults.is_empty(){
    println!("<div class=\"container\"><div class=\"center\">
    <h2 style=\"text-align: center;\">The Following Points Have Failed to Respond:</h2>
    <p></p>
    <ul>");
    for i in faults.iter(){println!("<li>{}</li>", i);}
    println!("<p>&nbsp;</p> \n<button
    onclick=\"window.location.href=\'/cgi-bin/main?fclear\'\">Reset
    </ul></button style=\"align: center;\"></div></div>");
}
println!("</body></html>");
}
fn read_status(path: &str) -> std::io::Result<Vec<String>>{
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let yam: Vec<String> = serde_yaml::from_str(&content).unwrap();
    Ok(yam)
}
