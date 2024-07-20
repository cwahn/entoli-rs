use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use tungstenite::{connect, Message};
use url::Url;

use crate::prelude::Io;

type Ws = Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

pub struct WsRun<F> {
    pub url: Url,
    pub f: F,
}

impl<I, F> Io for WsRun<F>
where
    I: Io<Output = ()>,
    F: FnMut(Ws) -> I,
{
    type Output = ();

    fn run(mut self) -> Self::Output {
        let socket = Arc::new(Mutex::new(
            connect(self.url.clone()).expect("Failed to connect").0,
        ));
        (self.f)(socket).run();
    }
}

pub fn ws_run<F>(url: Url, f: F) -> WsRun<F> {
    WsRun { url, f }
}

pub struct WsSend {
    pub socket: Ws,
    pub message: Message,
}

impl Io for WsSend {
    type Output = ();

    fn run(self) -> Self::Output {
        self.socket
            .lock()
            .unwrap()
            .write_message(self.message)
            .unwrap();
    }
}

pub fn ws_send(socket: Ws, message: Message) -> WsSend {
    WsSend { socket, message }
}

pub struct WsRecv {
    pub socket: Ws,
}

impl Io for WsRecv {
    type Output = Message;

    fn run(self) -> Self::Output {
        self.socket.lock().unwrap().read_message().unwrap()
    }
}

pub fn ws_recv(socket: Ws) -> WsRecv {
    WsRecv { socket }
}

pub struct WsClose {
    pub socket: Ws,
}

impl Io for WsClose {
    type Output = ();

    fn run(self) -> Self::Output {
        self.socket.lock().unwrap().close(None).unwrap();
    }
}

pub fn ws_close(socket: Ws) -> WsClose {
    WsClose { socket }
}
