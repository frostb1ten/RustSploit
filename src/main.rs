#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]

use serde_json;
use std::fs::File;
use snailquote::unescape;
use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;

extern crate glob;


fn main() {
    clearscreen::clear().unwrap();
    println!("Welcome to RustSploit");
    println!("Type WEB for Web Exploits");
    println!("Type PWN for Web Exploits");
    println!("Type MISC for Web Exploits");
    println!("Type exit to quit");
    println!("   ");

    let mut mminput = String::new();
    println!("Enter Command");
    std::io::stdin().read_line(&mut mminput).unwrap();
    if mminput.contains("WEB") {
        webmenu();
    }
    if mminput == "PWN" {
        pwnmenu();
    }
    if mminput == "MISC" {
        miscmenu();
    }
    if mminput == "exit" {
        std::process::abort();
    } else {
        println!("Incorrect command!");
        main();
    }
}

fn webmenu() {
    clearscreen::clear().unwrap();
    let paths = fs::read_dir("plugins").unwrap();
    println!("****List of web exploits****");
    for path in paths {
        let file = File::open(path.unwrap().path())
            .expect("No plugins detected.");
        let json: serde_json::Value = serde_json::from_reader(file)
            .expect("Invalid JSON file located.");
        let exploit_name = json.get("name").unwrap().to_string();
        let exploit_type = json.get("type").unwrap().to_string();
        let exploit_description = json.get("description").unwrap().to_string();
        if exploit_type.contains("WEB") {
            println!("{} - {}", exploit_name, exploit_description);
        }
    }
    let mut webinput = String::new();
    println!("Enter exploit name: ");
    std::io::stdin().read_line(&mut webinput);
    exploit(webinput);
}

fn pwnmenu() {
    clearscreen::clear().unwrap();
    let paths = fs::read_dir("plugins").unwrap();
    println!("-List of pwn exploits-");
    for path in paths {
        let file = File::open(path.unwrap().path())
            .expect("No plugins detected.");
        let json: serde_json::Value = serde_json::from_reader(file)
            .expect("Invalid JSON file located.");
        let exploit_name = json.get("name").unwrap().to_string();
        let exploit_type = json.get("type").unwrap().to_string();
        let exploit_description = json.get("description").unwrap().to_string();
        if exploit_type.contains("PWN") {
            println!("{} - {}", exploit_name, exploit_description);
        }
    }
    let mut pwninput = String::new();
    println!("Enter exploit name: ");
    std::io::stdin().read_line(&mut pwninput);
    exploit(pwninput);
}

fn miscmenu() {
    clearscreen::clear().unwrap();
    let paths = fs::read_dir("plugins").unwrap();
    println!("-List of pwn exploits-");
    for path in paths {
        let file = File::open(path.unwrap().path())
            .expect("No plugins detected.");
        let json: serde_json::Value = serde_json::from_reader(file)
            .expect("Invalid JSON file located.");
        let exploit_name = json.get("name").unwrap().to_string();
        let exploit_type = json.get("type").unwrap().to_string();
        let exploit_description = json.get("description").unwrap().to_string();
        if exploit_type.contains("MISC") {
            println!("{} - {}", exploit_name, exploit_description);
        }
    }

    println!("Enter exploit name: ");
    std::io::stdout().flush();
    let mut miscinput: String = String::new();
    std::io::stdin().read_line(&mut miscinput);
    exploit(miscinput);
}

fn exploit(exploit_name: String) {
    clearscreen::clear().unwrap();
    let plugindir = ".\\plugins\\".to_owned() + &exploit_name.trim();
    println!("DEBUG: {}", plugindir);
    let file = File::open(plugindir)
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

//variable
    let exploit_type = json.get("type").unwrap().to_string();
    let exploit_version = json.get("version").unwrap().to_string();
    let exploit_payload = unescape(json.get("payload").unwrap().to_string().as_str()).unwrap();

//Exploit Handler
    if exploit_type.contains("WEB") {
        let useragent = json.get("useragent").unwrap().to_string();
        let httpmethod = json.get("httpmethod").unwrap().to_string();
        if httpmethod.contains("GET") {
            getrequest(exploit_payload, useragent);
        } else if httpmethod.contains("POST") {
            postrequest(exploit_payload, useragent);
        }
    } else {
        //DEBUG
        println!("Exploit: {}", exploit_name);
        println!("Version: {}", exploit_version);
        println!("Exploit_type: {}", exploit_type);
        println!("Payload: {}", exploit_payload);
    }
}

#[tokio::main]
async fn getrequest(payload: String, agent: String) {
    let mut line = String::new();
    println!("Enter URL:");
    std::io::stdin().read_line(&mut line).unwrap();
    let result = reqwest::get(line.to_owned() + &payload).await.expect("Error with the URL");
    println!("{:?}", result);
    main();
}

#[tokio::main]
async fn postrequest(payload: String, agent: String) {
    let client = reqwest::Client::new();
    let mut line = String::new();
    println!("Enter URL:");
    std::io::stdin().read_line(&mut line).unwrap();
    let response = client
        .post(line)
        .body(payload)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    main();
}

#[tokio::main]
async fn pwn() {
    let mut RHOST = String::new();
    println!("RHOST:");
    std::io::stdin().read_line(&mut RHOST);

    let mut RPORT = String::new();
    println!("RPORT:");
    std::io::stdin().read_line(&mut RPORT);
    let mut RHOST = RHOST.to_owned() + ":";
    RHOST.push_str(&RPORT);
    println!("{}", RHOST);
    match TcpStream::connect(RHOST) {
        Ok(mut stream) => {
            println!("Successfully connected to server");
            let msg = b"Hello!";
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    main();
}
