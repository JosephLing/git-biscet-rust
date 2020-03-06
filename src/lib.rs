mod algorithm;
pub mod json_types;

use crate::algorithm::*;
pub use crate::json_types::*;

use serde_json::Value;

use std::collections::HashMap;

use ws::Result as ResultWS;
use ws::{connect, CloseCode, Handler, Handshake, Message, Sender};

// note: look into a better way potentailly to do the header...
// could use a macro or something
// (https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)
fn _send_data(out: &Sender, header: &str, msg: String) -> () {
    println!(">>> {} : {}", header, msg);
    out.send(serde_json::json!({ header: msg }).to_string());
}

fn send_question(out: &Sender, msg: String) {
    _send_data(out, "Question", msg)
}

fn send_solution(out: &Sender, msg: String) {
    _send_data(out, "Solution", msg)
}

fn pretty_print(parents: &HashMap<String, Vec<String>>) {
    // println!("parents: {:?}", parents);
    // for key in parents.keys() {
        // println!("{}", key);
    // }
    // println!("-----");
}

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    questions: i32,
    question_commit: String,
    bad: String,
    parents: HashMap<String, Vec<String>>,
    parents_master: HashMap<String, Vec<String>>,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> ResultWS<()> {
        println!("oepning");
        self.out
            .send(serde_json::json!({"User":["jl653", "f6b598a8"]}).to_string())
    }
    fn on_message(&mut self, msg: Message) -> ResultWS<()> {
        if let Ok(data) = serde_json::from_str::<Value>(msg.as_text().unwrap()) {
            if data["Repo"] != Value::Null {
                println!("been given another problem");
                self.parents_master = HashMap::new();
                self.questions = 0;
                self.question_commit = "".to_owned();
                let prob: JsonProblemDefinition =
                    serde_json::from_value::<JsonMessageProblem>(data)
                        .unwrap()
                        .Repo;
                for commit in prob.dag {
                    self.parents_master.insert(commit.commit, commit.parents);
                }
                println!("problem size: {}", self.parents_master.len());

            } else if data["Score"] != Value::Null {
                println!("score: {}", serde_json::from_value::<JsonScore>(data).unwrap());
                self.out.close(CloseCode::Normal);

            } else if data["Instance"] != Value::Null {
                println!("instance");
                let instance = serde_json::from_value::<JsonInstanceGoodBad>(data.clone())
                    .unwrap()
                    .Instance;
                println!("instance: {} {}", &instance.good, &instance.bad);
                println!("instance: {:?} {:?}", self.parents_master.contains_key(&instance.good), self.parents_master.contains_key(&instance.bad));
                self.bad = instance.bad;
                self.questions = 0;
                self.parents = self.parents_master.clone();
                remove_unecessary_good_commits(&instance.good, &mut self.parents);
                println!("good removal: {:?}", self.parents.len());
                remove_from_bad(&self.bad, &mut self.parents);
                println!("problem reduced to:{:?}", self.parents.len());
                if self.parents.len() == 1 {
                    send_solution(&self.out, self.parents.keys().last().unwrap().to_owned());
                } else {
                    self.question_commit = get_next_guess(&self.bad, &self.parents).unwrap();
                    send_question(&self.out, self.question_commit.to_string());
                }

            } else if self.questions >= 29 {
                println!("GIVING UP - moving onto the next question");
                //TODO: check what the limits are here!? and to see if we can submit a solution maybe??
                self.out.send(serde_json::json!("GiveUp").to_string());

            } else if data["Answer"] != Value::Null {
                if self.parents.len() == 1 {
                    println!("{:?}", self.parents.keys());
                    send_solution(&self.out, self.parents.keys().last().unwrap().to_owned());
                } else {
                    println!("answer: {}", self.parents.len());
                    let answer: String = serde_json::from_value::<JsonAnswer>(data).unwrap().Answer;
                    if answer.eq("bad") {
                        self.bad = self.question_commit.clone();
                        pretty_print(&self.parents);
                        remove_from_bad(&self.question_commit, &mut self.parents);
                    } else {
                        pretty_print(&self.parents);
                        remove_unecessary_good_commits(&self.question_commit, &mut self.parents);
                    }
                    pretty_print(&self.parents);

                    if self.parents.len() == 1 {
                        send_solution(&self.out, self.parents.keys().last().unwrap().to_owned());
                    } else {
                        self.questions += 1;
                        println!("question {}", self.questions);
                        self.question_commit = get_next_guess(&self.bad, &self.parents).unwrap();
                        send_question(&self.out, self.question_commit.to_string());
                    }
                }
            } else {
                println!("uknown json {:?}", msg)
            }
        } else {
            println!("invalid json {:?}", msg)
        }
        Ok(())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {:?} {}", code, reason),
        }
    }
}

pub fn run(address: String) {
    connect(address, |out| Client {
        out: out,
        questions: 0,
        bad: "".to_string(),
        question_commit: "".to_string(),
        parents: HashMap::new(),
        parents_master: HashMap::new(),
    })
    .unwrap();
}
