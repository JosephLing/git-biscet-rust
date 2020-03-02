use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;
use std::collections::VecDeque;

use rand::distributions::{IndependentSample, Range};

use ws::Result as ResultWS;
use ws::{connect, CloseCode, Handler, Handshake, Message, Sender};

#[derive(Debug)]
enum STATE {
    START,
    InProgress,
    GiveUp,
    FINISHED,
}

#[derive(Serialize, Deserialize)]
struct JsonNode {
    commit: String,
    parents: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonProblemDefinition {
    name: String,
    good: String,
    bad: String,
    dag: Vec<JsonNode>,
    // [["a",[]],["b",["a"]],["c",["b"]]]
    // [commit hash, [parent commit hashes]]
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct JsonMessageProblem {
    Problem: JsonProblemDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct JsonScore {
    Score: String, // {pb0: 2} or null or {pb0: null}
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct JsonAnswer {
    Answer: String,
}

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

#[allow(dead_code)]
fn remove_unecessary_good_commits(
    good: &String,
    parents: &mut HashMap<String, Vec<String>>,
    children: &mut HashMap<String, Vec<String>>,
) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let temp: &Vec<String> = parents.get(good).unwrap();
    for i in 0..temp.len() {
        queue.push_back(temp.get(i).unwrap().to_owned());
    }

    while !queue.is_empty() {
        let commit = queue.pop_front().unwrap();
        if let Some(cats) = parents.get(&commit) {
            let temp: &Vec<String> = cats;
            for i in 0..temp.len() {
                queue.push_back(temp.get(i).unwrap().to_owned());
            }
            parents.remove_entry(&commit);
            children.remove_entry(&commit);
        }
    }
}

fn parse_json(prob: JsonProblemDefinition) -> Vec<String> {
    let mut commits = HashMap::new();
    let mut children = HashMap::new();

    for commit in prob.dag {
        commits.insert(commit.commit, commit.parents);
    }

    remove_unecessary_good_commits(&prob.good, &mut commits, &mut children);
    let mut values: Vec<String> = Vec::new();
    for v in commits.keys() {
        values.push(v.into());
    }
    println!("{:?}", values);
    create_children(&prob.bad, &commits, &mut children);

    let mut values: Vec<String> = Vec::new();
    for v in children.keys() {
        values.push(v.into());
    }
    println!("{:?}", values);
    // remove commits
    return values;
}

fn create_children(
    bad: &String,
    parents: &HashMap<String, Vec<String>>,
    children: &mut HashMap<String, Vec<String>>,
) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let temp: &Vec<String> = parents.get(bad).unwrap();
    children.insert(bad.to_owned(), vec![]);
    for i in 0..temp.len() {
        queue.push_back(temp.get(i).unwrap().to_owned());
        if parents.contains_key(temp.get(i).unwrap()) {
            let mut new_children: Vec<String> = Vec::new();
            if let Some(child) = children.get(bad) {
                new_children.clone_from(child);
            }
            new_children.push(bad.to_owned());
            children.insert(temp.get(i).unwrap().to_owned(), new_children);
        }
    }

    while !queue.is_empty() {
        let commit = &queue.pop_front().unwrap();
        if let Some(cats) = parents.get(commit) {
            let temp: &Vec<String> = cats;
            for i in 0..temp.len() {
                queue.push_front(temp.get(i).unwrap().to_owned());

                if parents.contains_key(temp.get(i).unwrap()) {
                    let mut new_children: Vec<String> = Vec::new();
                    if let Some(child) = children.get(commit) {
                        new_children.clone_from(child);
                    }
                    new_children.push(commit.to_owned());
                    children.insert(temp.get(i).unwrap().to_owned(), new_children);
                }
            }
        }
    }
}

fn get_next_guess(
    bad: &String,
    parents: &HashMap<String, Vec<String>>,
    children: &HashMap<String, Vec<String>>,
) -> String {
    let chance = Range::new(0, parents.len() - 1).ind_sample(&mut rand::thread_rng());
    let mut count = 0;
    for k in parents.keys() {
        if count >= chance {
            return k.to_owned();
        }
        count += 1;
    }
    return "".to_owned();
}

fn solve(prob: JsonProblemDefinition) {
    let mut parents = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    for commit in prob.dag {
        parents.insert(commit.commit, commit.parents);
    }

    remove_unecessary_good_commits(&prob.good, &mut parents, &mut children);
    create_children(&prob.bad, &parents, &mut children);
}

#[cfg(test)]
mod parsing {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), serde_json::Error> {
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        serde_json::from_str::<JsonMessageProblem>(data)?;
        Ok(())
    }

    #[test]
    fn test_parse_data() -> Result<(), serde_json::Error> {
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.Problem.name, "pb0");
        Ok(())
    }

    #[test]
    fn test_parse_data_tree() -> Result<(), serde_json::Error> {
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.Problem.dag.len(), 3);
        Ok(())
    }

    #[test]
    fn test_parse_data_tree_node() -> Result<(), serde_json::Error> {
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.Problem.dag[0].commit, "a");
        Ok(())
    }
}

#[cfg(test)]
mod algorithm {
    use super::*;

    #[test]
    fn test_linear_tree() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let temp = parse_json(problem.Problem);
        assert_eq!(temp.len(), 3);
        Ok(())
    }

    #[test]
    fn test_linear_tree_value() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        assert_eq!(solution, ["a", "b", "c"]);
        Ok(())
    }

    #[test]
    fn test_linear_large() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"g","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["e",["d"]],["f",["e"]],["g",["f"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        assert_eq!(solution, ["a", "b", "c", "d", "e", "f", "g"]);
        Ok(())
    }

    #[test]
    fn test_branching() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     \
        //                      d (bad)
        //                      /
        //               f --> e
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        assert_eq!(solution, ["a", "b", "c", "d", "e", "f"]);
        Ok(())
    }

    #[test]
    fn test_commits_before_bad_commit() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c  --> d (bad) --> g
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();

        assert_eq!(solution, ["a", "b", "c", "d"]);
        Ok(())
    }

    #[test]
    fn test_commits_after_good_commit() -> Result<(), serde_json::Error> {
        // a <-- b (good) <-- c <-- d (bad) <-- g
        let data = r#"{"Problem":{"name":"pb0","good":"b","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        assert_eq!(solution, ["b", "c", "d"]);
        Ok(())
    }

    #[test]
    fn test_branching_good() -> Result<(), serde_json::Error> {
        // a >-- b --> c --> d
        // v     |
        // |     ^
        // \---> bb
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a"]],["bb",["b"]],["c",["b","bb"]],["d",["c"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        assert_eq!(solution, ["a", "b", "bb", "c", "d"]);
        Ok(())
    }
}

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    state: STATE,
    good: String,
    bad: String,
    questions: i32,
    question_commit: String,
    parents: HashMap<String, Vec<String>>,
    children: HashMap<String, Vec<String>>,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> ResultWS<()> {
        println!("oepning");
        self.out.send(r#"{"User":"jl653"}"#)
    }

    fn on_message(&mut self, msg: Message) -> ResultWS<()> {
        if let Ok(text) = msg.as_text() {
            println!("got a msg {:?} {:?}", msg, self.state);
            if let Ok(data) = serde_json::from_str::<Value>(&text) {
                println!("got valid json");

                if data["Problem"] != Value::Null {
                    println!("be given another problem");
                    self.state = STATE::START;
                }

                if data["Score"] != Value::Null {
                    println!("score: {:?}", data);
                    self.state = STATE::FINISHED;
                }

                self.state = match &self.state {
                    STATE::START => {
                        self.parents = HashMap::new();
                        self.children = HashMap::new();
                        self.questions = 0;
                        self.question_commit = "".to_owned();

                        println!("starting");
                        let prob: JsonProblemDefinition =
                            serde_json::from_value::<JsonMessageProblem>(data)
                                .unwrap()
                                .Problem;
                        for commit in prob.dag {
                            self.parents.insert(commit.commit, commit.parents);
                        }
                        println!("problem size: {:?}", self.parents.len());

                        remove_unecessary_good_commits(
                            &prob.good,
                            &mut self.parents,
                            &mut self.children,
                        );
                        println!("good removal: {:?}", self.parents.len());
                        // create_children(&prob.bad, &self.parents, &mut self.children);
                        self.bad = prob.bad;
                        self.good = prob.good;

                        println!("problem reduced to:{:?}", self.parents.len());
                        if self.parents.len() == 1 {
                            send_solution(
                                &self.out,
                                self.parents.keys().last().unwrap().to_owned(),
                            );
                        } else {
                            self.question_commit =
                                get_next_guess(&self.bad, &self.parents, &self.children);
                            send_question(&self.out, self.question_commit.to_string());
                        }
                        STATE::InProgress
                    }
                    STATE::InProgress => {
                        println!("in progress");

                        if self.parents.len() < 5 {
                            println!("{} {} {:?}", self.good, self.bad, self.parents.keys());
                        }

                        if self.parents.len() == 1 {
                            println!("{:?}", self.parents.keys());
                            send_solution(
                                &self.out,
                                self.parents.keys().last().unwrap().to_owned(),
                            );
                        } else {
                            if data["Answer"] != Value::Null {
                                let answer: String =
                                    serde_json::from_value::<JsonAnswer>(data).unwrap().Answer;
                                if answer.eq("bad") {
                                    println!("found bad");
                                    self.bad = self.question_commit.clone();
                                //           create_children(&self.question_commit, &self.parents, &mut self.children);
                                } else {
                                    println!("found good");
                                    self.good = self.question_commit.clone();
                                    remove_unecessary_good_commits(
                                        &self.question_commit,
                                        &mut self.parents,
                                        &mut self.children,
                                    );
                                }
                                println!("problem size: {:?}", self.parents.len());
                            }
                            if self.parents.len() == 1 {
                                send_solution(
                                    &self.out,
                                    self.parents.keys().last().unwrap().to_owned(),
                                );
                            } else {
                                self.questions += 1;
                                self.question_commit =
                                    get_next_guess(&self.bad, &self.parents, &self.children);
                                send_question(&self.out, self.question_commit.to_string());
                                println!("problem size: {:?}", self.parents.len());
                            }
                        }

                        match self.questions {
                            29 => STATE::GiveUp,
                            _ => STATE::InProgress,
                        }
                    }
                    STATE::GiveUp => {
                        println!("GIVING UP - moving onto the next question");
                        self.out.send(serde_json::json!("GiveUp").to_string());
                        STATE::START
                    }
                    STATE::FINISHED => STATE::FINISHED,
                };
            }else{
                println!("invalid json {:?}", msg);
            }
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
        state: STATE::START,
        questions: 0,
        bad: "".to_string(),
        question_commit: "".to_string(),
        good: "".to_string(),
        parents: HashMap::new(),
        children: HashMap::new(),
    })
    .unwrap();
}
