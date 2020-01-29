use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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
struct JsonMessageProblem {
    problem: JsonProblemDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonScore {
    score: String, // {pb0: 2} or null or {pb0: null}
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonAnswer {
    answer: String,
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

fn foo(
    good: &String,
    bad: String,
    commits: &HashMap<String, Vec<String>>,
    mut found: bool,
) -> (Vec<String>, bool) {
    let mut stack: Vec<String> = Vec::new();
    if let Some(parents) = commits.get(&bad) {
        for parent in parents {
            if !found && good == parent {
                found = true;
            }
            if !found {
                stack.push(parent.to_owned());
            }
            let (list, good_found) = foo(good, parent.to_owned(), commits, found);
            if good_found {
                stack.extend(list.iter().cloned());
                found = good_found;
            }
        }
    }

    return (stack.clone(), found);
}

fn parse_json(prob: JsonProblemDefinition) -> Vec<String> {
    let mut commits = HashMap::new();

    for commit in prob.dag {
        commits.insert(commit.commit, commit.parents);
    }

    let (list, _) = foo(&prob.good, prob.bad, &commits, false);

    // remove commits
    return list;
}

fn solve(prob: JsonProblemDefinition) {
    let mut commits = HashMap::new();

    for commit in prob.dag {
        commits.insert(commit.commit, commit.parents);
    }

    foo(&prob.good, prob.bad, &commits, false);
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
        assert_eq!(problem.problem.name, "pb0");
        Ok(())
    }

    #[test]
    fn test_parse_data_tree() -> Result<(), serde_json::Error> {
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.problem.dag.len(), 3);
        Ok(())
    }

    #[test]
    fn test_parse_data_tree_node() -> Result<(), serde_json::Error> {
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.problem.dag[0].commit, "a");
        Ok(())
    }
}

#[cfg(test)]
mod algorithm {
    use super::*;

    #[test]
    fn test_linear_tree() -> Result<(), serde_json::Error> {
        // a (good) <-- b <-- c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(parse_json(problem.problem).len(), 1);
        Ok(())
    }

    #[test]
    fn test_linear_tree_value() -> Result<(), serde_json::Error> {
        // a (good) <-- b <-- c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"c","dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(parse_json(problem.problem)[0], "b");
        Ok(())
    }

    #[test]
    fn test_linear_large() -> Result<(), serde_json::Error> {
        // a (good) <-- b <-- c (bad)
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"g","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["e",["d"]],["f",["e"]],["g",["f"]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let solution = parse_json(problem.problem);
        assert_eq!(solution, ["f", "e", "d", "c", "b"]);
        Ok(())
    }

    #[test]
    fn test_branching() -> Result<(), serde_json::Error> {
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let solution = parse_json(problem.problem);
        assert_eq!(solution, ["c", "b"]);
        Ok(())
    }

    #[test]
    fn test_branching_complex() -> Result<(), serde_json::Error> {
        // d has a child but we don't want to count that one
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["g",["d"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let solution = parse_json(problem.problem);
        assert_eq!(solution, ["c", "b"]);
        Ok(())
    }

    #[test]
    fn test_branching_complex_many_parents() -> Result<(), serde_json::Error> {
        // d has a child but we don't want to count that one
        let data = r#"{"Problem":{"name":"pb0","good":"a","bad":"d","dag":[["a",[]],["b",["a", "bb"]],["bb",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["g",["d"]],["f",[]]]}}"#;

        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        let solution = parse_json(problem.problem);
        assert_eq!(solution, ["c", "b", "bb"]);
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
// fine-grained control of the connection.
impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> ResultWS<()> {
        self.out.send("Hello WebSocket")
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
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    // Now, instead of a closure, the Factory returns a new instance of our Handler.
    connect("ws://127.0.0.1:3012", |out| Client {
        out: out,
        state: STATE::START,
    })
    .unwrap()
}
