use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;  
use std::collections::VecDeque;

use ws::Result as ResultWS;
use ws::{connect, CloseCode, Handler, Handshake, Message, Sender};

enum STATE {
    START,
    InProgress,
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
fn _send_data(out: Sender, header: &str, msg: String) -> Result<(), String> {
    out.send(serde_json::json!({ header: msg }).to_string());
    Ok(())
}

fn send_question(out: Sender, msg: String) -> Result<(), String> {
    _send_data(out, "Question", msg)
}

fn send_solution(out: Sender, msg: String) -> Result<(), String> {
    _send_data(out, "Solution", msg)
}

#[allow(dead_code)]
fn remove_unecessary_good_commits(
    good: &String,
    commits: &mut HashMap<String, Vec<String>>,
) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let temp: &Vec<String>  = commits.get(good).unwrap();
    for i in 0..temp.len() {
        queue.push_back(temp.get(i).unwrap().to_owned());
    }

    while !queue.is_empty(){
        let commit = queue.pop_front().unwrap();
        if let Some(cats) = commits.get(&commit){
            let temp: &Vec<String>  = cats;
            for i in 0..temp.len() {
                queue.push_back(temp.get(i).unwrap().to_owned());
            }
            commits.remove_entry(&commit);
        }

    }

    commits.remove_entry(good);
}





fn parse_json(prob: JsonProblemDefinition) -> Vec<String> {
    let mut commits = HashMap::new();

    for commit in prob.dag {
        commits.insert(commit.commit, commit.parents);
    }

    remove_unecessary_good_commits(&prob.good, &mut commits);
    let mut values: Vec<String> = Vec::new();
    for v in commits.keys() {
        values.push(v.into());
    }
    println!("{:?}", values);
    // remove commits
    return values;
}

fn solve(prob: JsonProblemDefinition) {
    let mut commits = HashMap::new();

    for commit in prob.dag {
        commits.insert(commit.commit, commit.parents);
    }

    remove_unecessary_good_commits(&prob.good, &mut commits);
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
        assert_eq!(temp.len(), 2);
        Ok(())
    }

    #[test]
    fn test_linear_tree_value() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(parse_json(problem.Problem), ["c", "b"]);
        Ok(())
    }

    #[test]
    fn test_linear_large() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"g","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["e",["d"]],["f",["e"]],["g",["f"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        assert_eq!(solution, ["b", "c", "d", "e", "f", "g"]);
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
        assert_eq!(solution, ["b", "c","d", "e", "f"]);
        Ok(())
    }

    #[test]
    fn test_commits_before_bad_commit() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c  --> d (bad) --> g
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        // currently just ignores 'g'
        // and doesn't remove 'f' which is a dead commit

        assert_eq!(solution, ["b", "c", "d", "f", "g"]);
        Ok(())
    }

    #[test]
    fn test_commits_after_good_commit() -> Result<(), serde_json::Error> {
        // a --> b (good) --> c --> d (bad) --> g
        let data = r#"{"Problem":{"name":"pb0","good":"b","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let mut solution = parse_json(problem.Problem);
        solution.sort();
        // currently doesn't remove 'g' as it just ignores it
        // and doesn't remove 'f' which is a dead commit
        assert_eq!(solution, ["c", "d","f", "g"]);
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
        assert_eq!(solution, ["b", "bb", "c", "d"]);
        Ok(())
    }
}

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    state: STATE,
}
// impl From<serde_json::Error> for io::Error{
// fn from(e: serde_json::Error) -> Self {ws::Error{kind: ws::ErrorKind::Internal, details: "cats"}}
// }

// We implement the Handler trait for Client so that we can get more
// fine-grained control of th   e connection.
impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> ResultWS<()> {
        println!("oepning");
        self.out.send(r#"{"User":"jl653"}"#)
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> ResultWS<()> {
        /*!
         * TODO:
         * state logic
         * types or representation for problem
         * errors and closing down when necessary
         * testing printing the score
         *
         */
        if let Ok(text) = msg.as_text() {
            println!("{}", text);
            if let Ok(data) = serde_json::from_str::<Value>(&text) {
                // https://docs.serde.rs/serde_json/fn.from_value.html
                if data["Problem"] != Value::Null {
                    let problem: JsonMessageProblem = serde_json::from_value(data).unwrap();
                } else if data["Answer"] != Value::Null {
                    println!("answers: {}", data["Answer"])
                } else if data["Score"] != Value::Null {
                    // just print here
                    println!("score: {}", data["Score"])
                } else {
                    // problem
                }
            }

            match self.state {
                STATE::START => 1,
                STATE::InProgress => 2,
            };
            // Close the connection when we get a response from the server
            println!("Got message: {}", msg);
        }
        self.out.close(CloseCode::Normal)
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
            _ => println!("The client encountered an error: {:?} {}",code, reason),
        }
    }
}

fn main() {
    println!("running");
    // Now, instead of a closure, the Factory returns a new instance of our Handler.
    connect("ws://129.12.44.229:1234", |out| Client {
        out: out,
        state: STATE::START,
    })
    .unwrap();
    println!("cats");
}
