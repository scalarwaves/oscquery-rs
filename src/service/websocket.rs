use std::net::{SocketAddr, TcpListener, ToSocketAddrs};
use std::thread::{spawn, JoinHandle};

use serde::{Deserialize, Serialize};

use tungstenite::{accept, error::Error, Message};

use crate::root::RootInner;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

//what we set the TCP stream read timeout to
const READ_TIMEOUT: Duration = Duration::from_millis(1);

pub struct WSService {
    handle: Option<JoinHandle<()>>,
    local_addr: SocketAddr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum ClientServerCmd {
    Listen,
    Ignore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ServerClientCmd {
    PathChanged,
    PathRenamed,
    PathRemoved,
    PathAdded,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct WSCommandPacket<T> {
    command: T,
    data: String,
}

impl WSService {
    pub(crate) fn new<A: ToSocketAddrs>(
        root: Arc<RwLock<RootInner>>,
        addr: A,
    ) -> Result<Self, std::io::Error> {
        let server = TcpListener::bind(addr)?;
        let local_addr = server.local_addr()?;
        let handle = spawn(move || {
            for stream in server.incoming() {
                if stream.is_err() {
                    continue;
                }
                let stream = stream.unwrap();
                stream
                    .set_read_timeout(Some(READ_TIMEOUT))
                    .expect("cannot set read timeout");
                let root = root.clone();
                spawn(move || {
                    if let Ok(mut websocket) = accept(stream) {
                        loop {
                            match websocket.read_message() {
                                Ok(msg) => {
                                    match msg {
                                        //binary messages are OSC packets
                                        Message::Binary(v) => {
                                            if let Ok(packet) = rosc::decoder::decode(&v) {
                                                if let Ok(root) = root.read() {
                                                    root.handle_osc_packet(&packet, None, None);
                                                }
                                            }
                                        }
                                        Message::Text(s) => {
                                            if let Ok(cmd) =
                                                serde_json::from_str::<
                                                    WSCommandPacket<ClientServerCmd>,
                                                >(&s)
                                            {
                                                println!("got command {:?}", cmd);
                                            }
                                        }
                                        Message::Close(..) => return,
                                        Message::Ping(d) => {
                                            //TODO if err, return?
                                            let _ = websocket.write_message(Message::Pong(d));
                                        }
                                        Message::Pong(..) => println!("pong"),
                                    };
                                }
                                Err(Error::ConnectionClosed) | Err(Error::AlreadyClosed) => {
                                    return;
                                }
                                Err(Error::Io(e)) => match e.kind() {
                                    std::io::ErrorKind::WouldBlock
                                    | std::io::ErrorKind::TimedOut => (),
                                    e @ _ => {
                                        println!("io ErrorKind {:?}", e);
                                        return;
                                    }
                                },
                                Err(..) => (), //TODO
                            }
                        }
                    }
                });
            }
        });
        Ok(Self {
            handle: Some(handle),
            local_addr,
        })
    }

    /// Returns the `SocketAddr` that the service bound to.
    pub fn local_addr(&self) -> &SocketAddr {
        &self.local_addr
    }
}

impl Drop for WSService {
    fn drop(&mut self) {
        //TODO send command to close and join
        /*
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        */
    }
}
