mod algorithm;
pub mod json_types;

use crate::algorithm::*;
pub use crate::json_types::*;

use serde_json::Value;

use std::collections::HashMap;
use std::io;

use std::fs::File;
use std::io::prelude::*;

use ws::Result as ResultWS;
use ws::{connect, CloseCode, Handler, Handshake, Message, Sender};

// {
//     // let mut debug: String = "digraph G {\n".to_string();
//     // debug += &format!("node [shape = doublecircle, color=blue]; {}\n", instance.bad);
//     // debug += &format!("node [shape = doublecircle, color=green]; {}\n", instance.good);

//     // debug += "node [shape = circle, color=black];\n";
//     // for node in self.parents.keys() {
//     //     for parent in self.parents.get(node).unwrap() {
//     //         if self.parents.contains_key(parent) {
//     //             debug = debug + &format!("{} -> {}\n", node, parent);
//     //         }
//     //     }
//     // }
//     // debug = debug + &"}".to_string();
//     // let mut file = File::create("main.dot").unwrap();
//     // file.set_len(0).unwrap();
//     // file.write_all(debug.as_bytes()).unwrap();
// }


// note: look into a better way potentailly to do the header...
// could use a macro or something
// (https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)
fn _send_data(out: &Sender, header: &str, msg: String) -> () {
    // println!(">>> {} : {}", header, msg);
    out.send(serde_json::json!({ header: msg }).to_string());
}

fn send_question(out: &Sender, msg: String) {
    _send_data(out, "Question", msg)
}

fn send_solution(out: &Sender, msg: String) {
    _send_data(out, "Solution", msg)
}

fn pretty_print(parents: &HashMap<String, Vec<String>>, name: &String, good: bool) {
        // if parents.len() < 10 {
        //     println!("parents: {:?}", parents);
        // }
        // for key in parents.keys() {
        //     println!("{}", key);
        // }
        // println!("-----");
        // println!("parents: {:?}", parents);
        // println!("keys: {:?}", parents.keys());
        // let mut debug: String = "digraph G {\n".to_string();
        // if good {
        //     debug += &format!("node [shape = doublecircle, color=green]; {}\n", name);
        // } else {
        //     debug += &format!("node [shape = doublecircle, color=red]; {}\n", name);
        // }
        // debug += "node [shape = circle, color=black];\n";
        // for node in parents.keys() {
        //     for parent in parents.get(node).unwrap() {
        //         if parents.contains_key(parent) {
        //             debug = debug + &format!("{} -> {}\n", node, parent);
        //         }else{
        //             println!("{}", parent);
        //         }
        //     }
        // }
        // debug = debug + &"}".to_string();
        // println!("---------------------");
        // println!("{}", debug);
        // let mut file = File::create(name.to_string() + ".dot").unwrap();
        // file.set_len(0).unwrap();
        // file.write_all(debug.as_bytes()).unwrap();
}

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    questions: i32,
    question_commit: String,
    bad: String,
    name: String,
    instance_count: i32,
    parents: HashMap<String, Vec<String>>,
    parents_master: HashMap<String, Vec<String>>,
}

fn debug(a: &Client, msg: &str) {
    if true{
        println!(
            "[client][{}][{}][{}][bad {}][qc {}] {}",
            a.name,
            a.instance_count,
            a.parents.len(),
            a.bad,
            a.question_commit,
            msg
        );
    }
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
                self.instance_count = 0;
                debug(self, "------------------------");
                self.parents_master = HashMap::new();
                let prob: JsonProblemDefinition =
                    serde_json::from_value::<JsonMessageProblem>(data)
                        .unwrap()
                        .Repo;
                self.name = prob.name;
                for commit in prob.dag {
                    self.parents_master.insert(commit.commit, commit.parents);
                }
            } else if data["Score"] != Value::Null {
                println!(
                    "score: {}",
                    serde_json::from_value::<JsonScore>(data).unwrap()
                );
                self.out.close(CloseCode::Normal);
            } else if data["Instance"] != Value::Null {
                self.instance_count += 1;
                debug(&self, "new instance");
                let instance = serde_json::from_value::<JsonInstanceGoodBad>(data.clone())
                    .unwrap()
                    .Instance;
                self.questions = 0;
                self.question_commit = "".to_owned();
                self.bad = "".to_string();


                debug(
                    &self,
                    &format!("instance: {} {}", &instance.good, &instance.bad),
                );
                

                self.bad = instance.bad;
                self.parents = self.parents_master.clone();
                debug(&self, "init");
                remove_unecessary_good_commits(&instance.good, &mut self.parents);
                debug(&self, "removed goods");
                remove_from_bad(&self.bad, &mut self.parents);
                pretty_print(&self.parents, &self.bad, false);
                if self.parents.len() == 1 {
                    debug(&self, &format!("solution: {:?}", self.parents.keys()));
                    send_solution(&self.out, self.parents.keys().last().unwrap().to_owned());
                } else {
                    self.question_commit = get_next_guess_algo(&self.bad, &self.parents).unwrap();
                    debug(
                        &self,  
                        &format!("question {} {}", self.questions, self.question_commit),
                    );
                    send_question(&self.out, self.question_commit.to_string());
                }
            } else if self.questions >= 29 {
                debug(self, "GIVING UP");
                //TODO: check what the limits are here!? and to see if we can submit a solution maybe??
                self.out.send(serde_json::json!("GiveUp").to_string());
            } else if data["Answer"] != Value::Null {
                if self.parents.len() == 1 {
                    debug(&self, &format!("solution: {:?}", self.parents.keys()));
                    send_solution(&self.out, self.parents.keys().last().unwrap().to_owned());
                } else {
                    let answer: String = serde_json::from_value::<JsonAnswer>(data).unwrap().Answer;
                    debug(
                        &self,
                        &format!("answer: {}\t{}", answer, self.question_commit),
                    );

                    if answer.eq("Bad") {
                        debug(&self, "removing based on bad commit");
                        self.bad = self.question_commit.clone();
                        pretty_print(&self.parents, &self.question_commit, false);
                        remove_from_bad(&self.question_commit, &mut self.parents);
                        pretty_print(&self.parents, &self.bad, false);
                    } else {
                        debug(&self, "removing based on good commit");
                        pretty_print(&self.parents, &self.question_commit, true);
                        remove_unecessary_good_commits(&self.question_commit, &mut self.parents);
                        pretty_print(&self.parents, &self.bad, false);
                    }

                    if self.parents.len() == 1 {
                        debug(&self, &format!("solution: {:?}", self.parents.keys()));
                        send_solution(&self.out, self.parents.keys().last().unwrap().to_owned());
                    } else {
                        self.questions += 1;
                        self.question_commit = get_next_guess_algo(&self.bad, &self.parents).unwrap();

                        debug(
                            &self,
                            &format!("question {} {}", self.questions, self.question_commit),
                        );
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
        name: "".to_string(),
        instance_count: 0,
        question_commit: "".to_string(),
        parents: HashMap::new(),
        parents_master: HashMap::new(),
    })
    .unwrap();
}
