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
    bad: Vec<Vec<HashSet<String>>>,
    problem: Vec<String>,
    instance: Vec<Vec<String>>,
    answer: Vec<String>,
    allow_give_up: bool,
    instance_index: usize,
    repo_index: usize
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
                    println!("got a solution");
                    assert_eq!(data["Solution"].as_str().unwrap(), self.answer[self.repo_index + self.instance_index]);
                    
                    if self.instance_index < self.instance[self.repo_index].len()-1{
                        self.instance_index += 1;
                        println!("server instance >>> {:?}", self.instance[self.repo_index][self.instance_index].clone());
                        self.out.send(self.instance[self.repo_index][self.instance_index].clone());
                    }else if self.repo_index < self.problem.len()-1{
                        self.repo_index = 0;
                        self.instance_index = 0;
                        
                        println!("server prob >>> {:?}", self.problem[self.repo_index].clone());
                        self.out.send(self.problem[self.repo_index].clone());
                        
                        println!("server instance>>> {:?}", self.instance[self.repo_index][self.instance_index].clone());
                        self.out.send(self.instance[self.repo_index][self.instance_index].clone());
                
                    }else{
                        return self.out.close(CloseCode::Normal);
                    }
                    

                }else if data["User"] != Value::Null {
                    self.repo_index = 0;
                    self.instance_index = 0;
                    self.out.send(self.problem[self.repo_index].clone());
                    self.out.send(self.instance[self.repo_index][self.instance_index].clone());

                }else if data["Question"] != Value::Null {
                    println!("question: {} {} {:?}", self.repo_index, self.instance_index, self.bad[self.repo_index][self.instance_index]);
                    if self.bad[self.repo_index][self.instance_index].contains(data["Question"].as_str().unwrap()){
                        self.out.send(serde_json::json!({"Answer": "bad"}).to_string());
                    }else{
                        self.out.send(serde_json::json!({"Answer": "good"}).to_string());
                    }
                }else{
                    assert_eq!(serde_json::json!("GiveUp").to_string(),text);
                    assert_eq!(self.allow_give_up, true);

                    if !self.allow_give_up{
                        self.out.close(CloseCode::Normal);
                    }else{
                        if self.instance_index < self.instance[self.repo_index].len()-1{
                            self.instance_index += 1;
                            println!("server >>> {:?}", self.instance[self.repo_index][self.instance_index].clone());
                            self.out.send(self.instance[self.repo_index][self.instance_index].clone());
                        }else if self.repo_index < self.problem.len()-1{
                            self.repo_index = 0;
                            self.instance_index = 0;
                    
                            self.out.send(self.problem[self.repo_index].clone());
                            self.out.send(self.instance[self.repo_index][self.instance_index].clone());
                    
                        }else{
                            return self.out.close(CloseCode::Normal);
                        }
                    }
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

pub fn create_single_repo_server (
    host: String,
    bad: Vec<HashSet<String>>,
    problem: String,
    instances: Vec<String>,
    answer: Vec<String>,
    allow_give_up: bool
){
    println!("creating integeration test webserver");
    let repo : Vec<String> = vec![problem; 1];
    let repo_instances : Vec<Vec<String>> = vec![instances; 1];
    let bad_things : Vec<Vec<HashSet<String>>> = vec![bad; 1];
    // Run the WebSocket
    listen(host, |out| {
        Server {
            out: out,
            ping_timeout: None,
            expire_timeout: None,
            bad: bad_things.clone(),
            problem: repo.clone(),
            instance: repo_instances.clone(),
            answer: answer.clone(),
            allow_give_up: allow_give_up,
            instance_index: 0,
            repo_index: 0
        }
    }).unwrap();
}
