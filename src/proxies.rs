use std::{*, fs::File, io::{BufReader, BufRead}};
use reqwest::Proxy;
use lazy_static::lazy_static;

static mut types:usize=1;
pub fn read_file_proxies()->Vec<reqwest::Proxy>{
    let Type=1;
    let file =File::open("proxies.txt").expect("erro");
    let buffer=BufReader::new(file);
    let mut proxies:Vec< reqwest::Proxy > =Vec::new(); 
    for i in buffer.lines(){
        let parsc=i.as_ref().unwrap().split(":").collect::<Vec<_>>();
        if parsc.capacity()==4 {
        

            match Type{
                1=>{proxies.push(reqwest::Proxy::all(format!("http://{}:{}",parsc[2],parsc[3])).unwrap().basic_auth(parsc[0], parsc[1]))}
                2=>{proxies.push(reqwest::Proxy::all(format!("https://{}:{}",parsc[2],parsc[3])).unwrap().basic_auth(parsc[0], parsc[1]))}
                3=>{proxies.push(reqwest::Proxy::all(format!("socks4://{}:{}",parsc[2],parsc[3])).unwrap().basic_auth(parsc[0], parsc[1]))}
                4=>{proxies.push(reqwest::Proxy::all(format!("socks5://{}:{}",parsc[2],parsc[3])).unwrap().basic_auth(parsc[0], parsc[1]))}
                _=>{}
        }
    }
    else{
        match Type{
            1=>{proxies.push(reqwest::Proxy::all(format!("http://{}",i.unwrap())).unwrap())}
            2=>{proxies.push(reqwest::Proxy::all(format!("https://{}",i.unwrap())).unwrap())}
            3=>{proxies.push(reqwest::Proxy::all(format!("socks4://{}",i.unwrap())).unwrap())}
            4=>{proxies.push(reqwest::Proxy::all(format!("socks5://{}",i.unwrap())).unwrap())}
            _=>{}




        }
    };

    }
    return proxies
  

}
pub fn proxies_choice()->usize{

let mut proxies=String::new();
println!("Enter type of proxies 1-http  2-https  3-Socks4 4-Socks5");
let Type=std::io::stdin().read_line(&mut proxies).expect("wrong input");
return proxies.trim().parse().unwrap()
}

