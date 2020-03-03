use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct JsonGoodAndBad {
    pub good: String,
    pub bad: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonInstanceGoodBad {
    pub Instance: JsonGoodAndBad,
}

#[derive(Serialize, Deserialize)]
pub struct JsonMessageProblem {
    pub Repo: JsonProblemDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JsonScore {
    pub Score: String, // {pb0: 2} or null or {pb0: null}
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JsonAnswer {
    pub Answer: String,
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
