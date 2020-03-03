// 
use serde_json::Value;
use std::str::from_utf8;
use std::collections::HashSet;

use ws::{listen, CloseCode, OpCode, Sender, Frame, Handler, Handshake, Message, Result, Error, ErrorKind};
use ws::util::{Token, Timeout};

const PING: Token = Token(1);
const EXPIRE: Token = Token(2);

// Server WebSocket handler
struct Server {
    out: Sender,
    ping_timeout: Option<Timeout>,
    expire_timeout: Option<Timeout>,
    bad: HashSet<String>,
    problem: String,
    instance: String,
    answer: String,
    allow_give_up: bool
}

impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // schedule a timeout to send a ping every 5 seconds
        self.out.timeout(5_000, PING)?;
        // schedule a timeout to close the connection if there is no activity for 30 seconds
        self.out.timeout(10_000, EXPIRE)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {

        if let Ok(text) = msg.as_text() {
            if let Ok(data) = serde_json::from_str::<Value>(&text) {
                if data["Solution"] != Value::Null{
                    assert_eq!(data["Solution"].as_str().unwrap(), self.answer);
                    self.out.close(CloseCode::Normal);

                }else if data["User"] != Value::Null {
                    self.out.send(self.problem.clone());
                    self.out.send(self.instance.clone());

                }else if data["Question"] != Value::Null {
                    println!("got a question");
                    if self.bad.contains(data["Question"].as_str().unwrap()){
                        self.out.send(serde_json::json!({"Answer": "bad"}).to_string());
                    }else{
                        self.out.send(serde_json::json!({"Answer": "good"}).to_string());
                    }
                }else{
                    println!("Server got message '{}'. ", msg);
                    self.out.close(CloseCode::Normal);
                    assert_eq!(serde_json::json!("GiveUp").to_string(),text);
                    assert_eq!(self.allow_give_up, true);
                }
            }
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);

        // NOTE: This code demonstrates cleaning up timeouts
        if let Some(t) = self.ping_timeout.take() {
            self.out.cancel(t).unwrap();
        }
        if let Some(t) = self.expire_timeout.take() {
            self.out.cancel(t).unwrap();
        }

        println!("Shutting down server after first connection closes.");
        self.out.shutdown().unwrap();
    }

    fn on_error(&mut self, err: Error) {
        // Shutdown on any error
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            // PING timeout has occured, send a ping and reschedule
            PING => {
                self.out.ping(time::precise_time_ns().to_string().into())?;
                self.ping_timeout.take();
                self.out.timeout(5_000, PING)
            }
            // EXPIRE timeout has occured, this means that the connection is inactive, let's close
            EXPIRE => self.out.close(CloseCode::Away),
            // No other timeouts are possible
            _ => Err(Error::new(ErrorKind::Internal, "Invalid timeout token encountered!")),
        }
    }

    fn on_new_timeout(&mut self, event: Token, timeout: Timeout) -> Result<()> {
        // Cancel the old timeout and replace.
        if event == EXPIRE {
            if let Some(t) = self.expire_timeout.take() {
                self.out.cancel(t)?
            }
            self.expire_timeout = Some(timeout)
        } else {
            // This ensures there is only one ping timeout at a time
            if let Some(t) = self.ping_timeout.take() {
                self.out.cancel(t)?
            }
            self.ping_timeout = Some(timeout)
        }

        Ok(())
    }

    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        // If the frame is a pong, print the round-trip time.
        // The pong should contain data from out ping, but it isn't guaranteed to.
        if frame.opcode() == OpCode::Pong {
            if let Ok(pong) = from_utf8(frame.payload())?.parse::<u64>() {
                let now = time::precise_time_ns();
                println!("RTT is {:.3}ms.", (now - pong) as f64 / 1_000_000f64);
            } else {
                println!("Received bad pong.");
            }
        }

        // Some activity has occured, so reset the expiration
        self.out.timeout(10_000, EXPIRE)?;

        // Run default frame validation
        DefaultHandler.on_frame(frame)
    }
}

// For accessing the default handler implementation
struct DefaultHandler;

impl Handler for DefaultHandler {}

pub fn create_test_server (bad: HashSet<String>, problem: String,instance: String, answer: String, allow_give_up: bool){


    // Run the WebSocket
    listen("127.0.0.1:3012", |out| {
        Server {
            out: out,
            ping_timeout: None,
            expire_timeout: None,
            bad: bad.clone(),
            problem: problem.clone(),
            instance: instance.clone(),
            answer: answer.clone(),
            allow_give_up: allow_give_up
        }
    }).unwrap();
}
