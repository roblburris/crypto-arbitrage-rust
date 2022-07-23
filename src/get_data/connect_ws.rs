// use std::borrow::Borrow;
// use std::net::TcpStream;
// use serde::{Deserialize, Serialize};
// use websocket::client::ClientBuilder;
// use websocket::{OwnedMessage, Message};
// use tracing::{span, Level, event};
//
// #[derive(Serialize, Deserialize)]
// struct TradeData {
//     e: String,
//     E: u64,
//     s: String,
//     t: u64,
//     p: String,
//     q: String,
//     b: u64,
//     a: u64,
//     T: u64,
//     m: bool,
//     M: bool
// }
//
// fn get_data_from_ws(connection_addr: &str) -> std::io::Result<()> {
//     // let mut s: TcpStream;
//     let mut client;
//     match ClientBuilder::new(connection_addr).unwrap().connect_secure(None) {
//         Ok(c) => client = c,
//         Err(e) => event!(target: "connection init", Level::ERROR, "unable to connect to websocket")
//     }
//
//     span!(Level::INFO, addr = connection_addr, "connected to websocket");
//     let _enter = span.enter();
//     let mut close = false;
//     let mut ping = false;
//     loop {
//         // Receive loop
//         for message in client.incoming_messages() {
//             let message = match message {
//                 Ok(m) => m,
//                 Err(_) => {
//                     event!(Level::ERROR, "unable to parse message");
//                     break;
//                 }
//             };
//
//             match message {
//                 OwnedMessage::Close(_) => close = true,
//                 OwnedMessage::Ping(data) => {
//                     ping = true;
//                 },
//                 OwnedMessage::Text(s) => {
//                     let json_v:TradeData;
//                     match serde_json::from_str(s.into_boxed_str().borrow()) {
//                         Ok(d) => json_v = d,
//                         Err(e) => event!(Level::ERROR, response_data=s, "unable to parse response data")
//                     }
//                     //
//                     println!("{}", json_v.p);
//                 }
//                 _ => event!(Level::ERROR, "unable to parse response data"),
//             }
//         }
//
//         if close {
//             event!(Level::INFO, "closing websocket");
//             client.send_message(&Message::close());
//         }
//
//         if ping {
//             event!(Level::INFO, "sent ping");
//             client.send_message(&Message::pong(Vec::new()));
//         }
//     }
// }