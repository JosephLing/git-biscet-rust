mod server;

use std::thread;

extern crate gitbisectrust;
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
        //                     \
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
    /// NOTE: leave me in as a test specailly if searching method changes
    fn single_instance_2() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     \
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
        //                     \
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
        // \---> bb
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
}