mod get_data;

use std::borrow::{Borrow, BorrowMut};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use websocket::client::ClientBuilder;
use websocket::{OwnedMessage, Message};
use tracing::{span, Level, event};

#[derive(Serialize, Deserialize)]
struct TradeData {
    e: String,
    E: u64,
    s: String,
    t: u64,
    p: String,
    q: String,
    b: u64,
    a: u64,
    T: u64,
    m: bool,
    M: bool
}

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    const CONNECTION_ADDR: &str = "wss://stream.binance.com:9443/ws/btcusdt@trade";
    // let mut s: TcpStream;
    let client;
    match ClientBuilder::new(CONNECTION_ADDR).unwrap().connect_secure(None) {
        Ok(c) => client = c,
        Err(e) => {
            event!(target: "connection init", Level::ERROR, "unable to connect to websocket");
            panic!("abc");
        }
    }

    let span = span!(Level::INFO, CONNECTION_ADDR, "connected to websocket");
    let _enter = span.enter();
    loop {
        // Receive loop
        for message in client.borrow().incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(_) => {
                    event!(Level::ERROR, "unable to parse message");
                    break;
                }
            };

            match message {
                OwnedMessage::Close(_) => {
                    client.borrow().send_message(&Message::close());
                    event!(Level::INFO, "closing websocket");
                },
                OwnedMessage::Ping(data) => {
                    client.borrow().send_message(&Message::pong(Vec::new()));
                    event!(Level::INFO, "sent ping");
                },
                OwnedMessage::Text(s) => {
                    let json_v:TradeData;
                    match serde_json::from_str(s.into_boxed_str().borrow()) {
                        Ok(d) => json_v = d,
                        Err(e) => {
                            event!(Level::ERROR, "unable to parse response data");
                            break;
                        }
                    }
                    //
                    // println!("{}", json_v.p);
                }
                _ => event!(Level::ERROR, "unable to parse response data"),
            }
        }

        println!("reahced");
    }
}