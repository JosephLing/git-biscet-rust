use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use rand::distributions::{IndependentSample, Range};

pub fn remove_unecessary_good_commits(good: &String, parents: &mut HashMap<String, Vec<String>>) {
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
        }
    }
}

pub fn remove_from_bad(bad: &String, parents: &mut HashMap<String, Vec<String>>) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let parents_of_bad: &Vec<String> = parents.get(bad).unwrap();
    let mut results: HashSet<String> = HashSet::new();
    results.insert(bad.to_string());
    for i in 0..parents_of_bad.len() {
        let temp = parents_of_bad.get(i).unwrap();
        queue.push_back(temp.to_owned());
        results.insert(temp.to_owned());
    }
    while !queue.is_empty() {
        if let Some(cats) = parents.get(&queue.pop_front().unwrap()) {
            for i in 0..cats.len() {
                let temp = cats.get(i).unwrap();
                if !results.contains(temp) && parents.contains_key(temp) {
                    queue.push_back(temp.to_owned());
                    results.insert(temp.to_owned());
                }
            }
        }
    }
    let mut parent_keys: HashSet<String> = HashSet::with_capacity(parents.len());
    for key in parents.keys() {
        parent_keys.insert(key.to_owned());
    }
    for removal in results.symmetric_difference(&parent_keys) {
        parents.remove(removal);
    }
    println!("deleted");
}

pub fn get_next_guess(bad: &String, parents: &HashMap<String, Vec<String>>) -> String {
    let chance = parents.len() / 2;
    let mut count = 0;
    for k in parents.keys() {
        if count >= chance {
            return k.to_owned();
        }
        count += 1;
    }
    return "".to_owned();
}

#[cfg(test)]
mod algorithm {
    use super::*;
    use crate::json_types::*;


    fn parse_json(prob: JsonProblemDefinition, goodAndBad: JsonGoodAndBad) -> Vec<String> {
        let mut commits = HashMap::new();
        for commit in prob.dag {
            commits.insert(commit.commit, commit.parents);
        }
        remove_unecessary_good_commits(&goodAndBad.good, &mut commits);
        let mut values: Vec<String> = Vec::new();
        for v in commits.keys() {
            values.push(v.into());
        }
        println!("{:?}", values);
        remove_from_bad(&goodAndBad.bad, &mut commits);
        let mut values: Vec<String> = Vec::new();
        for v in commits.keys() {
            values.push(v.into());
        }
        println!("{:?}", values);
        // remove commits
        return values;
    }

    fn helper(data: &str, instance: &str) -> Vec<String> {
        let problem = serde_json::from_str::<JsonMessageProblem>(data).unwrap();
        let mut solution = parse_json(
            problem.Repo,
            serde_json::from_str::<JsonInstanceGoodBad>(instance)
                .unwrap()
                .Instance,
        );
        solution.sort();
        solution
    }

    #[test]
    fn test_linear_tree() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"c"}}"#;
        let temp = helper(data, instance);
        assert_eq!(temp.len(), 3);
        Ok(())
    }
    
    #[test]
    fn test_linear_tree_value() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"c"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["a", "b", "c"]);
        Ok(())
    }
    #[test]
    fn test_linear_large() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["e",["d"]],["f",["e"]],["g",["f"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"g"}}"#;
        let solution = helper(data, instance);
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
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["a", "b", "c", "d", "e", "f"]);
        Ok(())
    }
    #[test]
    fn test_commits_before_bad_commit() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c  --> d (bad) --> g
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["a", "b", "c", "d"]);
        Ok(())
    }
    #[test]
    fn test_commits_after_good_commit() -> Result<(), serde_json::Error> {
        // a <-- b (good) <-- c <-- d (bad) <-- g
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"b","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "c", "d"]);
        Ok(())
    }
    #[test]
    fn test_branching_good() -> Result<(), serde_json::Error> {
        // a >-- b --> c --> d
        // v     |
        // |     ^
        // \---> bb
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["bb",["b"]],["c",["b","bb"]],["d",["c"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["a", "b", "bb", "c", "d"]);
        Ok(())
    }
}
