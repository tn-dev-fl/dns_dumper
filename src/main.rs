use std::fs::OpenOptions;
use std::io::{self, Read};
use std::path::{PathBuf, self};
struct pep;
use scraper::{Html, Selector, ElementRef};
use std::fs::File;
use std::io::BufReader;
use std::fs;
use std::env;
use std::path::Path;
use rand::Rng;
use tokio::time::{sleep, Duration};
use lazy_static::lazy_static;
use std::io::prelude::*;
use proxies::read_file_proxies;
//
lazy_static! {
   pub static ref list_proxies: Vec<reqwest::Proxy> = read_file_proxies();

}
async fn write(content:String){
    if Path::new("sites.txt").exists()==false {
        let file=File::create("sites.txt");
        
    }
    let mut ws=content+("\n");
    //let file =File::open("sites.txt");
    let mut option=OpenOptions::new().append(true).write(true).open("sites.txt").unwrap();
    option.write(ws.as_bytes());
}
async fn ip_req(mut prox:Vec<reqwest::Proxy>)->Result<String,()>{
    if prox.clone().is_empty()==true{
        prox=read_file_proxies();
    }
    
        
    
    //await;
    let reqs =reqwest::Client::builder().proxy(prox.pop().unwrap()).build().unwrap();
    let resp=reqs.get(("https://api.hackertarget.com/reverseiplookup/?q=".to_owned()+ip_gen().as_str())).send().await;

    if resp.is_ok(){
        
     let c=resp.unwrap().text().await.unwrap(); if c.contains("DNS")==false && c.contains("API")==false&& c.contains("error")==false {
        println!("==>{:?}",c.clone());
        write(c.clone()).await;
        let dd=c.clone();
        Ok((dd))
    }
    else {
        return Err(())
    }
    
}
    else  {
        println!("the proxies error");
        return Err(())
    }
}
fn ip_gen()->String{

    let mut rng = rand::thread_rng();
    
    let part1=rng.gen_range(0..255);
    let part2=rng.gen_range(0..255);
    let part3=rng.gen_range(0..255);
    let part4=rng.gen_range(0..255);
    format!("{}.{}.{}.{}",part1,part2,part3,part4)
    
    }
async fn ip_search(){
    let mut proxies= proxies::read_file_proxies();

  loop {
    let mut handles = Vec::with_capacity(300);

      
  
  for fut in 0..300 {
      handles.push(tokio::spawn(ip_req(proxies.clone())));
  }
  
  let mut results = Vec::with_capacity(handles.len());
  for handle in handles {
    
      results.push(handle.await.unwrap());
  }
  }

}
#[tokio::main]

async fn main() {
let test=ip_search().await;
}
    
    
