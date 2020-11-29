extern crate reqwest;
use std::fs::File;
use std::{io::{prelude::*, BufReader},	path::Path};
use clap::{Arg, App};

#[tokio::main]
async fn get_request(url: String, code: String) -> Result<(), reqwest::Error> {
	
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?;
    
    let status_codes = code.split(" ");
    
    for index in status_codes {
        if res.status().to_string().contains(index) {
            
        } else {
            println!("Status : {} - URL : {}", res.status().to_string(), &url);
        }
    }
    


    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let _matches = App::new("Rustfuzz")
        .version("0.1.0")
        .author("Arekkusu")
        .about("WFuzz clone in rust")
        .arg(Arg::with_name("wordlist")
                 .short("w")
                 .long("wordlist")
                 .takes_value(true)
                 .help("Wordlist to fuzz with"))
        .arg(Arg::with_name("URL")
                 .short("u")
                 .long("url")
                 .takes_value(true)
                 .help("URL to fuzz"))
        .arg(Arg::with_name("codes")
                 .short("ec")
                 .long("codes")
                 .takes_value(true)
                 .help("Filter out unwanted status codes : Default 404"))
        .get_matches();
    let url = _matches.value_of("URL").unwrap_or("No Url provided");
    let wordlist = _matches.value_of("wordlist").unwrap_or("input.txt");
    let error_codes = _matches.value_of("codes").unwrap_or("404");
    let lines = lines_from_file(&wordlist);

    println!("Script is running!");

    for line in lines {
        get_request(url.replace("FUZZ", &line), error_codes.to_string());
    }
    

}
