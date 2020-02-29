extern crate simple_logger;
extern crate clap;

use log::info;
use clap::{App, Arg};
use ruft::Node;
use std::thread;

macro_rules! start_node {
    ($id: expr) => {
        let mut peer1 = vec![String::from("127.0.0.1:8000"), String::from("127.0.0.1:8010")];
        let mut peer2 = vec![String::from("127.0.0.1:8001"), String::from("127.0.0.1:8011")];
        let mut peer3 = vec![String::from("127.0.0.1:8002"), String::from("127.0.0.1:8012")];
        let mut peer4 = vec![String::from("127.0.0.1:8003"), String::from("127.0.0.1:8013")];
        let mut peer5 = vec![String::from("127.0.0.1:8004"), String::from("127.0.0.1:8014")];
        let mut peers = vec![peer1, peer2, peer3, peer4, peer5];
        peers.remove($id);
        let node = Node::new(
            String::from("127.0.0.1"), 
            8000 + ($id as u16),
            String::from("127.0.0.1"),
            8010 + ($id as u16),
            5,
            50,
            peers
        );
        let mut node = match node {
            Ok(node) => node,
            Err(error) => panic!("Creating Node Error: {}", error),
        };
        match node.run() {
            Ok(()) => info!("Node Stopped"),
            Err(error) => panic!("Running Node Error: {}", error),
        };
    };
}

fn main() {
    simple_logger::init().unwrap();
    // Init command line args handler
    // let matches = App::new("ruft")
    //     .version("0.1")
    //     .author("JmPotato <ghzpotato@gmail.com>")
    //     .about("Rust implementation of raft distributed consensus algorithm")
    //     .arg(
    //         Arg::with_name("host")
    //             .long("host")
    //             .value_name("HOST")
    //             .help("Sets the node's host")
    //             .takes_value(true)
    //             .required(true),
    //     )
    //     .arg(
    //         Arg::with_name("port")
    //             .long("port")
    //             .value_name("PORT")
    //             .help("Sets the node's port")
    //             .takes_value(true)
    //             .required(true),
    //     )
    //     .arg(
    //         Arg::with_name("num")
    //             .long("num")
    //             .value_name("NUM")
    //             .help("Sets the cluster's node number")
    //             .takes_value(true),
    //     )
    //     .get_matches();

    // println!("[Node Configuration]");

    // let mut node_host: String = String::from("127.0.0.1");
    // let mut node_port: u16 = 5299;
    // let mut node_num: u32 = 5;
    // let heartbeat_interval: u32 = 5;
    // let node_list: Vec<String> = Vec::new();

    // if let Some(host) = matches.value_of("host") {
    //     println!("Host: {}", host);
    //     node_host = String::from(host);
    // }

    // if let Some(port) = matches.value_of("port") {
    //     println!("Port: {}", port);
    //     node_port = port.parse::<u16>().unwrap();
    // }

    // if let Some(num) = matches.value_of("num") {
    //     println!("Num: {}", num);
    //     node_num = num.parse::<u32>().unwrap();
    // }

    // println!("\n[Node Logs]");

    // let ruft_node = Node::new(
    //     node_host,
    //     node_port,
    //     node_num,
    //     heartbeat_interval,
    //     node_list,
    // );

    // let mut ruft_node = match ruft_node {
    //     Ok(node) => node,
    //     Err(error) => panic!("Creating Node Error: {}", error),
    // };

    // match ruft_node.run() {
    //     Ok(()) => println!("Node Stopped"),
    //     Err(error) => panic!("Running Node Error: {}", error),
    // };

    for id in 0..5 {
        let child = thread::spawn(move || {
            start_node!(id);
        });
    }
    thread::sleep_ms(3000);
}
