use axum::{
    extract::ConnectInfo, routing::{get,post}, Router
};
use std::net::SocketAddr;
use reqwest::{
    Client
};
use dotenv::dotenv;
use std::env;
use tower_http::cors::{CorsLayer,Any};
#[tokio::main]
 async fn main()
 {
    dotenv().ok();
    let cors=CorsLayer::new().allow_origin(Any);  
    let app=Router::new().route("/getip",get(Find)).layer(cors);
    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
 }
 async fn Find(ConnectInfo(x):ConnectInfo<SocketAddr>)->String
 {
   let ip=x.ip().to_string();
   let api_key=env::var("api_key").unwrap();

   let url=format!("https://api.ipdata.co/{ip}?api-key={api_key}&fields=ip,is_eu,city,region,region_code,country_name,country_code,continent_name,continent_code,latitude,longitude,postal,calling_code,flag,emoji_flag,emoji_unicode");
   
   let resp=Client::new().get(url).send().await.unwrap().text().await.unwrap();
   println!("{:?}",resp);

  "Asdf".to_string()
 }