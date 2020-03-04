use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::fmt; // Import `fmt`
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct JsonNode {
    pub commit: String,
    pub parents: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonProblemDefinition {
    pub name: String,
    pub instance_count: usize,
    pub dag: Vec<JsonNode>,
    // {"name":"pb0","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonGoodAndBad {
    pub good: String,
    pub bad: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonInstanceGoodBad {
    pub Instance: JsonGoodAndBad,
}

#[derive(Serialize, Deserialize)]
pub struct JsonMessageProblem {
    pub Repo: JsonProblemDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JsonAnswer {
    pub Answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JsonScore {
    pub Score: HashMap<String, Value>,
}

impl fmt::Display for JsonScore {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut gaveup = 0;
        let mut correct = 0;
        let mut wrong = 0;
        writeln!(f, "scores: ");
        for k in self.Score.keys(){
            let temp = self.Score.get(k).unwrap();
            if temp["Correct"] != Value::Null{
                correct += 1;
                writeln!(f,"{} took {:?} questions", k, temp["Correct"].to_string());
            }else {
                let temp = temp.to_string();
                if temp.trim() == r#""Wrong""#.to_string(){
                    wrong += 1;
                }else if temp.trim() == r#""GiveUp""#.to_string(){
                    gaveup += 1;
                }
                writeln!(f,"{} {}", k, temp.trim());
            }
        }
        writeln!(f, "total correct: {}, wrong: {}, gaveup: {}", correct, wrong, gaveup);
        writeln!(f, "------")
    }

}

#[cfg(test)]
mod parsing {
    use super::*;
    #[test]
    fn test_parse() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        serde_json::from_str::<JsonMessageProblem>(data)?;
        Ok(())
    }
    #[test]
    fn test_parse_data() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.Repo.name, "pb0");
        Ok(())
    }
    #[test]
    fn test_parse_data_tree() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"c"}}"#;
        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.Repo.dag.len(), 3);
        Ok(())
    }
    #[test]
    fn test_parse_data_tree_node() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"c"}}"#;
        let problem = serde_json::from_str::<JsonMessageProblem>(data)?;
        assert_eq!(problem.Repo.dag[0].commit, "a");
        Ok(())
    }
}
