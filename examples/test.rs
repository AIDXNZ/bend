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
    bend.clock.apply(Dot::new("A".to_string(), 5));

    let mut msg = SyncMessage {
        //Intentionally set the id to kick off sync
        id: "Aidan".to_string(),
        len: 4,
        start: "cbsdjhcbshjks".to_string(),
        end: "zvdfvjksnfvjdkdv".to_string(),
        range_len: 5,
        items: None,
    };
    

    //let resp = bend.handle_msg(msg).unwrap().unwrap();
   

    loop {
        while let Some(con) = server.accept().await {
            let mut peer = con.await.unwrap();
            println!("Connected to {:?}", peer.remote_address());
            match bend.out_going_connections.try_write() {
                Ok(mut guard) => {
                    let addr = peer.remote_address().clone();
                    guard.push(addr);
                    peer.accept_bi().await.unwrap().0.write_all(msg.start.as_bytes()).await.unwrap();
                    println!("{:?}", bend.out_going_connections.read().unwrap())
                }
                Err(_) => todo!(),
            }
        }
    }
}

