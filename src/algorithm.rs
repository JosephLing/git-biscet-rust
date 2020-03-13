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
    // this is necessary for binary search but won't work well
    // for the "proper" way of counting
    parents.remove_entry(good);
}

pub fn remove_from_bad(bad: &String, parents: &mut HashMap<String, Vec<String>>) {
    let mut queue: VecDeque<String> = VecDeque::new();
    let parents_of_bad: &Vec<String> = parents.get(bad).unwrap();
    let mut results: HashSet<String> = HashSet::new();
    results.insert(bad.to_string());
    for i in 0..parents_of_bad.len() {
        let temp = parents_of_bad.get(i).unwrap();

        if !results.contains(temp) && parents.contains_key(temp) {
            queue.push_back(temp.to_owned());
            results.insert(temp.to_owned());
        }

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
    // println!("to delete: {:?}", results);
    // println!("from: {:?}", parent_keys);
    for removal in results.symmetric_difference(&parent_keys) {
        parents.remove(removal);
    }
}

/// it does a binary search by doing a breadth first searh through tree
/// this means that it will check the first parent, second parent, third....
/// and keep on going like that. Therefore even if we are going half way it might not
/// actually be fully half way down the tree.
pub fn get_next_guess(bad: &String, parents: &HashMap<String, Vec<String>>) -> Option<String> {
    println!("get_next_guess {} {}", bad, parents.len());
    let half_way = (parents.len() as f64 / 2 as f64).ceil() as usize;
    let mut count = 1;
    let mut queue: VecDeque<String> = VecDeque::new();
    let parents_of_bad: &Vec<String> = parents.get(bad).unwrap();
    let mut results: HashSet<String> = HashSet::new();
    results.insert(bad.to_string());
    for i in 0..parents_of_bad.len() {
        let temp = parents_of_bad.get(i).unwrap();

        if !results.contains(temp) && parents.contains_key(temp) {
            queue.push_back(temp.to_owned());
            results.insert(temp.to_owned());
            count += 1;
    
            if count >= half_way {
                return Some(temp.to_string());
            }
        }
        
    }
    while !queue.is_empty() {
        if let Some(cats) = parents.get(&queue.pop_front().unwrap()) {
            for i in 0..cats.len() {
                let temp = cats.get(i).unwrap();
                if !results.contains(temp) && parents.contains_key(temp) {

                    queue.push_back(temp.to_owned());
                    results.insert(temp.to_owned());
                    if count >= half_way {
                        return Some(temp.to_string());
                    }
                    count += 1;
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod removal {
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
        assert_eq!(temp.len(), 2);
        Ok(())
    }

    #[test]
    fn test_daimond() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#;
        let instance = r#"{"Instance":{"good":"s","bad":"y"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["t", "u", "v", "w", "x", "y"]);
        Ok(())
    }

    #[test]
    fn test_daimond2() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#;
        let instance = r#"{"Instance":{"good":"r","bad":"y"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["q", "s", "t", "u", "v", "w", "x", "y"]);
        Ok(())
    }

    #[test]
    fn test_daimond3() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#;
        let instance = r#"{"Instance":{"good":"r","bad":"y"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["q", "s", "t", "u", "v", "w", "x", "y"]);
        Ok(())
    }

    #[test]
    fn test_linear_tree_value() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"c"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "c"]);
        Ok(())
    }

    #[test]
    fn test_linear_large() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["e",["d"]],["f",["e"]],["g",["f"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"g"}}"#;
        let solution = helper(data, instance);
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
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "c", "d", "e", "f"]);
        Ok(())
    }

    #[test]
    fn test_commits_before_bad_commit() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c  --> d (bad) --> g
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "c", "d"]);
        Ok(())
    }

    #[test]
    fn test_commits_after_good_commit() -> Result<(), serde_json::Error> {
        // a <-- b (good) <-- c <-- d (bad) <-- g
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["g",["d"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"b","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["c", "d"]);
        Ok(())
    }

    #[test]
    fn test_branching_good() -> Result<(), serde_json::Error> {
        // a >-- b --> c --> d
        // v     |
        // |     ^
        // \---> bb
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a","bb"]],["bb",["a"]],["c",["b"]],["d",["c"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "bb", "c", "d"]);
        Ok(())
    }

    #[test]
    fn test_branching_rogue_commits() -> Result<(), serde_json::Error> {
        // a >-- b --> c --> d
        // v     |     \_ x
        // |     ^
        // \---> bb -- y
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["x",["c"]],["y",["bb"]],["b",["a","bb"]],["bb",["a"]],["c",["b"]],["d",["c"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "bb", "c", "d"]);
        Ok(())
    }


    /// this dag is so nested it is a pointless test
    #[ignore]
    #[test]
    fn test_tiny_complete() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"tiny-complete","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["b","a"]],["d",["c","b","a"]],["e",["d","c","b","a"]],["f",["e","d","c","b","a"]],["g",["f","e","d","c","b","a"]],["h",["g","f","e","d","c","b","a"]],["i",["h","g","f","e","d","c","b","a"]],["j",["i","h","g","f","e","d","c","b","a"]],["k",["j","i","h","g","f","e","d","c","b","a"]],["l",["k","j","i","h","g","f","e","d","c","b","a"]],["m",["l","k","j","i","h","g","f","e","d","c","b","a"]],["n",["m","l","k","j","i","h","g","f","e","d","c","b","a"]],["o",["n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["p",["o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["q",["p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["r",["q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["s",["r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["t",["s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["u",["t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["v",["u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["w",["v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["x",["w","v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["y",["x","w","v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["z",["y","x","w","v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]]]}}"#;
        let instance = r#"{"Instance":{"good":"b","bad":"y"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "bb", "c", "d"]);
        Ok(())
    }

    //{"d": ["c"], "w": ["v"], "x": ["w"], "e": ["d"], "h": ["g"], "m": ["l"], "y": ["x"], "z": ["y"], "r": ["q"], "l": ["k"], "a": [], "t": ["s"], "p": ["o"], "k": ["j"], "n": ["m"], "o": ["n"], "q": ["p"], "f": ["e"], "v": ["u"], "s": ["r"], "u": ["t"], "g": ["f"], "c": ["b"], "b": ["a"], "i": ["h"], "j": ["i"]}
    // b u

    #[test]
    fn test_branching_daimond() -> Result<(), serde_json::Error> {
        //
        //    b
        //   /  \
        //  a    d
        //   \ c /
        //     
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["c", "b"]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let solution = helper(data, instance);
        assert_eq!(solution, ["b", "c", "d"]);
        Ok(())
    }
}

#[cfg(test)]
mod counting {
    use super::*;
    use crate::json_types::*;

    fn counting_helper(data: &str, bad: String) -> Option<String> {
        let problem: JsonProblemDefinition = serde_json::from_str::<JsonMessageProblem>(data)
            .unwrap()
            .Repo;
        let mut commits = HashMap::new();
        for commit in problem.dag {
            commits.insert(commit.commit, commit.parents);
        }
        get_next_guess(&bad, &commits)
    }
    
    #[test]
    fn test_only_two() -> Result<(), serde_json::Error> {
        // b <-- c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":2,"dag":[["b",["a"]],["c",["b"]]]}}"#;
        let temp = counting_helper(data, "c".to_string());
        assert_eq!(temp, Some("b".to_string()));
        Ok(())
    }

    #[test]
    fn test_only_one() -> Result<(), serde_json::Error> {
        // b
        let data = r#"{"Repo":{"name":"pb0","instance_count":2,"dag":[["b",["a"]]]}}"#;
        let temp = counting_helper(data, "b".to_string());
        assert_eq!(temp, None);
        Ok(())
    }

    #[test]
    fn test_small_linear() -> Result<(), serde_json::Error> {
        // a (good) <-- b <-- c (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]]]}}"#;
        let temp = counting_helper(data, "c".to_string());
        assert_eq!(temp, Some("b".to_string()));
        Ok(())
    }

    #[test]
    fn test_meduim_linear() -> Result<(), serde_json::Error> {
        // a <-- b <-- c <-- d <-- e <-- f (bad)
        let data = r#"{"Repo":{"name":"pb0","instance_count":6,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c"]],["e",["d"]],["f",["e"]]]}}"#;
        let temp = counting_helper(data, "f".to_string());
        assert_eq!(temp, Some("c".to_string()));
        Ok(())
    }

    #[test]
    fn test_branching_good() -> Result<(), serde_json::Error> {
        // a >-- b --> c --> d
        // v     |
        // |     ^
        // \---> bb
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["bb",["b"]],["c",["b","bb"]],["d",["c"]]]}}"#;
        let temp = counting_helper(data, "d".to_string());
        assert_eq!(temp, Some("bb".to_string()));
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
        let temp = counting_helper(data, "d".to_string());
        assert_eq!(temp, Some("e".to_string()));
        Ok(())
    }

    #[test]
    fn test_trap() -> Result<(), serde_json::Error> {
        // a -> b -> c -> d
        //      \___ e
        //       \___ f
        //
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["b",["c", "e", "f"]],["c",["d"]],["d",[]],["e",[]],["f",[]]]}}"#;
        let temp = counting_helper(data, "b".to_string());
        assert_eq!(temp, Some("e".to_string()));
        Ok(())
    }
}
