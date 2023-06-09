use bend::{node::Node, protocol::{ SyncMessage}};
use crdts::{CmRDT, Dot};
use futures::executor::block_on;
use std::{
    collections::{btree_set::Range, BTreeSet},
    error::Error,
    hash::Hash,
    net::SocketAddr,
    sync::Arc,
};
//use matrix_rs::Matrix;

#[tokio::main]
async fn main() {
    block_on(daemon())
}

async fn daemon() {
    let mut bend = Node::new();
    let server = bend.endpoint.clone();
    println!("Listening on: {:?}", server.local_addr().unwrap());

    //Insert
    bend.store.insert("nvdfvjksnfvjdkdv".to_string());
    bend.store.insert("ndasjfgnerwiebewb".to_string());
    bend.store.insert("fsd dvnenvsdnvdv".to_string());
    bend.store.insert("dfbgnsdjsdavnvjkanlva".to_string());
    bend.store.insert("cbsdjhcbshjks".to_string());

    //register clock events
    bend.clock.local_casual.apply(Dot::new("A".to_string(), 5));

    let mut matric_clock: Vec<Vec<i32>> = Vec::new();

   
    let a = bend.clock.local_casual.get(&"A".to_string()) as i32;
    
   

   println!("{:?}", a);



    loop {
        while let Some(con) = server.accept().await {
            let mut peer = con.await.unwrap();
            println!("Connected to {:?}", peer.remote_address());
            match bend.out_going_connections.try_write() {
                Ok(mut guard) => {
                    let addr = peer.remote_address().clone();
                    guard.push(addr);
                    println!("{:?}", bend.out_going_connections.read().unwrap())
                }
                Err(_) => todo!(),
            }
            bend.handle_conn(peer).await;
       }
    }
}

