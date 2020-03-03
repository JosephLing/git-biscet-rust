mod server;

use std::thread;

extern crate gitbisectrust;
use std::collections::HashSet;

pub use crate::server::create_single_repo_server;
use gitbisectrust::run;

#[cfg(test)]
mod integeration {
    use super::*;
    
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
        let server = thread::spawn(move || {
            server::create_single_repo_server(vec![bad; 1], data.to_string(), instance, "f".to_string(), false);
        });
        println!("cats");
        let client = thread::spawn(move || {
            run("ws://127.0.0.1:3012".to_string());
        });
        println!("cats");

        server.join().unwrap();
        client.join().unwrap();
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
        let instance = vec![r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string(), r#"{"Instance":{"good":"a","bad":"d"}}"#.to_string()];
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("c".to_string());
        bad.insert("d".to_string());

        let mut bad2: HashSet<String> = HashSet::new();
        bad2.insert("c".to_string());
        bad2.insert("d".to_string());
        let server = thread::spawn(move || {
            server::create_single_repo_server(vec![bad, bad2], data.to_string(), instance, "f".to_string(), true);
        });
        println!("cats");
        let client = thread::spawn(move || {
            run("ws://127.0.0.1:3012".to_string());
        });
        println!("cats");

        server.join().unwrap();
        client.join().unwrap();
        Ok(())
    }
}
