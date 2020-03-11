mod server;
extern crate gitbisectrust;

use std::thread;

use std::collections::HashSet;

pub use crate::server::create_single_repo_server;
use gitbisectrust::run;

#[cfg(test)]
mod integeration {
    use super::*;

    /// creates the client and server threads and easily allows them to run on two different ports
    /// NOTE: if they are running on the same port bad things will happen!!!
    fn helper(
        host: &String,
        bad: Vec<HashSet<String>>,
        problem: String,
        instances: Vec<String>,
        answer: Vec<String>,
        allow_give_up: bool,
    ) {
        let server_host = "127.0.0.1:".to_string() + host;
        let client_host = "ws://".to_string() + &server_host;
        let server = thread::spawn(move || {
            println!("running server at {}", server_host);
            server::create_single_repo_server(
                server_host,
                bad,
                problem,
                instances,
                answer,
                allow_give_up,
            );
        });
        let client = thread::spawn(move || {
            println!("running client at {}", client_host);
            run(client_host);
        });

        server.join().unwrap();
        println!("server finished");

        client.join().unwrap();
        println!("client finished");

    }
    
    #[test]
    fn single_instance() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     
        //                      d (bad)
        //                      /
        //               f --> e
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;
        let instance = vec![r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string()];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("e".to_string());
        bad.insert("f".to_string());
        bad.insert("d".to_string());
        helper(
            &"3011".to_string(),
            vec![bad; 1],
            data.to_string(),
            instance,
            vec!["f".to_string()],
            false,
        );
        Ok(())
    }

    #[test]
    fn trap_bad_commit() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     
        //                      d (bad)
        //                      /
        //         g --> f --> e
        //          
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["c2",[]],["c1",["c2"]],["g",["c2"]],["b",["g","c1"]],["c3",["c1"]]]}}"#;
        let instance = vec![r#"{"Instance":{"good":"g","bad":"b"}}"#.to_string()];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("c1".to_string());
        helper(
            &"2000".to_string(),
            vec![bad; 1],
            data.to_string(),
            instance,
            vec!["c1".to_string()],
            false,
        );
        Ok(())
    }

    #[test]
    /// NOTE: leave me in as a test specailly if searching method changes
    fn single_instance_2() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     
        //                      d (bad)
        //                      /
        //               f --> e
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;
        let instance = vec![r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string()];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("c".to_string());
        bad.insert("d".to_string());
        helper(
            &"3012".to_string(),
            vec![bad; 1],
            data.to_string(),
            instance,
            vec!["c".to_string()],
            false,
        );
        Ok(())
    }
    #[test]
    fn mutliple_instances() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     
        //                      d (bad)
        //                      /
        //               f --> e
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Repo":{"name":"pb0","instance_count":3,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;
        let instance = vec![
            r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string(),
            r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string(),
        ];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("e".to_string());
        bad.insert("f".to_string());
        bad.insert("d".to_string());

        let mut bad2: HashSet<String> = HashSet::new();
        bad2.insert("c".to_string());
        bad2.insert("d".to_string());

        helper(
            &"3013".to_string(),
            vec![bad, bad2],
            data.to_string(),
            instance,
            vec!["f".to_string(), "c".to_string()],
            false,
        );
        Ok(())
    }

    #[test]
    fn mutliple_instances_circular() -> Result<(), serde_json::Error> {
        // a >-- b --> c --> d
        // v     |
        // |     ^
        // ---> bb
        let data = r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a", "bb"]],["bb",["a"]],["c",["b","bb"]],["d",["c"]]]}}"#;
        let instance = vec![
            r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string(),
            r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string(),
        ];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("d".to_string());
        bad.insert("c".to_string());
        bad.insert("b".to_string());
        bad.insert("bb".to_string());

        let mut bad2: HashSet<String> = HashSet::new();
        bad2.insert("c".to_string());
        bad2.insert("d".to_string());

        helper(
            &"3014".to_string(),
            vec![bad, bad2],
            data.to_string(),
            instance,
            vec!["bb".to_string(), "c".to_string()],
            false,
        );
        Ok(())
    }

    #[test]
    fn tiny_diamonds() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#;
        let instance = vec![
            r#"{"Instance":{"good":"r","bad":"y"}}"#.to_string(),
            ];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("w".to_string());
        bad.insert("t".to_string());
        bad.insert("x".to_string());
        bad.insert("v".to_string());
        bad.insert("u".to_string());
        bad.insert("s".to_string());
        bad.insert("r".to_string());
        bad.insert("q".to_string());

        helper(
            &"3021".to_string(),
            vec![bad],
            data.to_string(),
            instance,
            vec!["q".to_string()],
            false,
        );
        Ok(())
    }

    #[test]
    fn tiny_diamonds2() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#;
        let instance = vec![
            r#"{"Instance":{"good":"r","bad":"y"}}"#.to_string(),
            ];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("w".to_string());
        bad.insert("t".to_string());
        bad.insert("x".to_string());
        bad.insert("v".to_string());
        
        helper(
            &"3022".to_string(),
            vec![bad],
            data.to_string(),
            instance,
            vec!["t".to_string()],
            false,
        );
        Ok(())
    }


    #[test]
    fn tiny_diamonds3() -> Result<(), serde_json::Error> {
        let data = r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#;
        let instance = vec![
            r#"{"Instance":{"good":"c","bad":"t"}}"#.to_string(),
            ];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("s".to_string());
        bad.insert("r".to_string());
        
        helper(
            &"3023".to_string(),
            vec![bad],
            data.to_string(),
            instance,
            vec!["t".to_string()],
            false,
        );
        Ok(())
    }
}

//r#""#,
// r#"{"Instance":{"good":"r","bad":"y"}}"#,