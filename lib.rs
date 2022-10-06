use reqwest::*;
use async_trait::async_trait;
use reqwest::header::{COOKIE, USER_AGENT, HeaderMap};
use std::io::{self,BufReader, BufRead, Read};
use std::fs::File;
use rand::Rng;
use regex::*;
use lazy_static::lazy_static;
/*
to do
-parse domain
-parse ip 
-thread implement 
-extract value
-write to a file


*/
lazy_static! {
    static ref rex: Regex = Regex::new(r"[a-zA-Z0-9]{64}").unwrap();
    static ref prox :reqwest::Proxy=reqwest::Proxy::https("https://83.149.70.159:13012").unwrap();
    static ref reqs :reqwest::Client=reqwest::Client::builder().proxy(prox.clone()).build().unwrap();
    static ref urls:String=String::from("https://api.hackertarget.com/reverseiplookup/?q=");
}
pub struct Headers {
    pub user_agents:Vec<String>,
    pub file_path:String
    
}
#[async_trait]
pub trait req {

    fn random_agents(&self)->String{("").to_string()}
    fn read_file(&mut self){}
    async fn get_token(&self){}
    async fn testing_with_burp(&self){}
    async fn ip_gen(&self)->String{("").to_string()}
    async fn ip_search(&self){}
    async fn ip_req(&self){}
}
#[async_trait]
impl req for Headers {


fn read_file(&mut self){
    let file =File::open("src/user_agents").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        self.user_agents.push(line.unwrap())
    }
}

fn random_agents(&self)->String{



let mut rng = rand::thread_rng();

 self.user_agents[rng.gen_range(00..self.user_agents.len())].to_string()



}
async fn ip_req(&self){
    let resp=reqs.get(("https://api.hackertarget.com/reverseiplookup/?q=".to_owned()+&self.ip_gen().await.to_owned())).send().await.unwrap().text().await;
    println!("{:?}",resp)
}
async fn testing_with_burp(&self){
    let mut map = HeaderMap::new();
    map.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0".parse().unwrap());
    map.insert("Referer", "https://dnsdumpster.com/".parse().unwrap());
    let mut buf = Vec::new();
    File::open("cacert.der").unwrap().read_to_end(&mut buf).unwrap();
    let cert = reqwest::Certificate::from_der(&buf).unwrap();
    let t = reqwest::ClientBuilder::cookie_store(reqwest::ClientBuilder::new(), true);
    let c=t.add_root_certificate(cert.clone()).build().unwrap().clone();
    let get1=c.get("https://dnsdumpster.com/").send().await.unwrap().text().await.unwrap();
    let result=rex.captures(get1.as_str()).unwrap();
    let params = [("csrfmiddlewaretoken", result[0].to_string()), ("targetip", "ntbbelajar.web.id".to_string()),("user","free".to_string())];
   let b= c.post("https://dnsdumpster.com/").headers(map).form(&params).send().await.unwrap().text().await.unwrap();
    println!("{:?}",b);
}
async fn get_token(&self){

     

    let test=reqwest::ClientBuilder::cookie_store(reqwest::ClientBuilder::new(), true);
    
    let t=test.build().unwrap().get("https://dnsdumpster.com/").send().await.unwrap().text().await.unwrap();
    //println!("{:?}",t.get("set-cookie"))
   
    let result=rex.captures(t.as_str()).unwrap();
    let params = [("csrfmiddlewaretoken", result[0].to_string()), ("targetip", "readm.org".to_string()),("user","free".to_string())];
   let b= reqs.post("https://dnsdumpster.com/").form(&params).send().await.unwrap().text().await.unwrap();
   println!("{:?}",b);
   

}
async fn ip_gen(&self)->String{

let mut rng = rand::thread_rng();

let part1=rng.gen_range(0..255);
let part2=rng.gen_range(0..255);
let part3=rng.gen_range(0..255);
let part4=rng.gen_range(0..255);
format!("{}.{}.{}.{}",part1,part2,part3,part4)

}
async fn ip_search(&self){
    /* 
let mut handles = Vec::with_capacity(5);
  
  for fut in 0..5 {
      //handles.push(tokio::spawn();
  }
  
  let mut results = Vec::with_capacity(handles.len());
  for handle in handles {
    
      results.push(handle.await.unwrap());
  }
   */


}

}
impl Default for Headers {
    fn default() -> Headers {
        Headers{
            user_agents:vec!["Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:34.0) Gecko/20100101 Firefox/34.0
            ".to_string()],
            file_path:"user_agents".to_string(),
        }
        
        
    }
    
}
