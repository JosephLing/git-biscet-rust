mod server;

use std::thread;

extern crate gitbisectrust;
use std::collections::HashSet;

pub use crate::server::create_test_server;
use gitbisectrust::run;

#[cfg(test)]
mod integeration {
    use super::*;
    #[test]
    fn cats() -> Result<(), serde_json::Error> {
        // a (good) --> b --> c
        //                     \
        //                      d (bad)
        //                      /
        //               f --> e
        // d has two parents and we only want to get the ones that have a good commit
        // as their parent
        let data = r#"{"Repo":{"name":"pb0","dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#;
        let instance = r#"{"Instance":{"good":"a","bad":"d"}}"#;
        let mut bad: HashSet<String> = HashSet::new();
        bad.insert("e".to_string());
        bad.insert("f".to_string());
        let server = thread::spawn(move || {
            server::create_test_server(bad, data.to_string(), instance.to_string(), "f".to_string(), false);
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
