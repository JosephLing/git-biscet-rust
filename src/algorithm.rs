use crate::json_types::*;


use std::collections::VecDeque;
use std::collections::HashMap;

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

pub fn parse_json(prob: JsonProblemDefinition, goodAndBad: JsonGoodAndBad) -> Vec<String> {
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
    create_children(&goodAndBad.bad, &mut commits);
    let mut values: Vec<String> = Vec::new();
    for v in commits.keys() {
        values.push(v.into());
    }
    println!("{:?}", values);
    // remove commits
    return values;
}

pub fn create_children(bad: &String, parents: &mut HashMap<String, Vec<String>>) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let temp: &Vec<String> = parents.get(bad).unwrap();
    // children.insert(bad.to_owned(), vec![]);
    for i in 0..temp.len() {
        queue.push_back(temp.get(i).unwrap().to_owned());
        if parents.contains_key(temp.get(i).unwrap()) {
            // let mut new_children: Vec<String> = Vec::new();
            // if let Some(child) = children.get(bad) {
            //     new_children.clone_from(child);
            // }
            // new_children.push(bad.to_owned());
            // children.insert(temp.get(i).unwrap().to_owned(), new_children);
        }
    }
    while !queue.is_empty() {
        let commit = &queue.pop_front().unwrap();
        if let Some(cats) = parents.get(commit) {
            let temp: &Vec<String> = cats;
            for i in 0..temp.len() {
                queue.push_front(temp.get(i).unwrap().to_owned());
                if parents.contains_key(temp.get(i).unwrap()) {
                    // let mut new_children: Vec<String> = Vec::new();
                    // if let Some(child) = children.get(commit) {
                    //     new_children.clone_from(child);
                    // }
                    // new_children.push(commit.to_owned());
                    // children.insert(temp.get(i).unwrap().to_owned(), new_children);
                }
            }
        }
    }
}

pub fn get_next_guess(bad: &String, parents: &HashMap<String, Vec<String>>) -> String {
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

fn solve(prob: JsonProblemDefinition, goodAndBad: JsonGoodAndBad) {
    let mut parents = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    for commit in prob.dag {
        parents.insert(commit.commit, commit.parents);
    }
    remove_unecessary_good_commits(&goodAndBad.good, &mut parents);
    create_children(&goodAndBad.bad, &mut parents);
}


#[cfg(test)]
mod algorithm {
    use super::*;
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