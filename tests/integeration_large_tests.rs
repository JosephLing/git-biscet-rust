mod server;
extern crate gitbisectrust;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::thread;

use serde_json::Value;
use serde_json::*;

use serde::{Deserialize, Serialize};

use gitbisectrust::run;
// use gitbisectrust::json_types::*;

pub use crate::server::create_single_repo_server;

#[derive(Serialize, Deserialize)]
struct JsonFileProblem {
    all_bad: HashSet<String>,
    bug: String,
}

#[cfg(test)]
mod bigJsonExamples {
    use super::*;
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

    fn helper_file(filename: String, host: String) -> Result<()> {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let data = serde_json::from_str::<Value>(&contents).unwrap();
        let bad = serde_json::from_value::<JsonFileProblem>(data[1].clone())
            .unwrap()
            .all_bad;
        let data2 =
            serde_json::json!({"Repo":{"name":"pb0", "instance_count":3, "dag": data[0]["dag"]}})
                .to_string();
        let single_instance = serde_json::json!(
            {"Instance":{"good": data[0]["good"], "bad": data[0]["bad"] }}
        );
        let instance: Vec<String> = vec![single_instance.to_string()];
        helper(
            &host,
            vec![bad],
            data2,
            instance,
            vec![data[1]["bug"].to_string().replace("\"", "")],
            false,
        );
        Ok(())
    }

    fn helper_loader(foo: &String, starting: String) {
        let mut count = 0;
        for entry in fs::read_dir("./tests/test_data/").unwrap() {
            let dir = entry.unwrap().path().to_str().unwrap().to_string();
            if dir.contains(foo) {
                println!("running test for dir {}", count);
                if count > 10 {
                    helper_file(dir, format!("{}{}", starting, count));
                } else {
                    helper_file(dir, format!("{}0{}", starting, count));
                }
                count += 1;
            }
        }
    }

    #[test]
    fn test_bootstrap() -> Result<()> {
        helper_loader(&"bootstrap".to_string(), "30".to_string());
        Ok(())
    }

    #[test]
    fn test_swift() -> Result<()> {
        helper_loader(&"swift".to_string(), "31".to_string());
        Ok(())
    }

    #[test]
    fn test_fb() -> Result<()> {
        helper_loader(&"fb".to_string(), "32".to_string());
        Ok(())
    }

    #[test]
    fn test_react() -> Result<()> {
        helper_loader(&"react".to_string(), "33".to_string());
        Ok(())
    }

    #[test]
    fn test_tensorflow() -> Result<()> {
        helper_loader(&"tensorflow".to_string(), "34".to_string());
        Ok(())
    }

    #[test]
    fn test_vscode() -> Result<()> {
        helper_loader(&"vscode".to_string(), "35".to_string());
        Ok(())
    }

    
    // --------------------------------------------------------
    // so these both fail with a give up giving up at getting to a dag 
    // of the size of 249 things
    // that means they are probably the same question
    #[test]
    fn test_test() -> Result<()> {
        // 249
        helper_loader(&"test_test".to_string(), "36".to_string());
        Ok(())  
    }

    #[test]
    // need to implement higher timeouts for these and
    // it's expected that these fails
    fn test_linux() -> Result<()> {
        // test_linux0 get's down to 249 repos in 29 questions
        helper_loader(&"linux".to_string(), "37".to_string());
        Ok(())

    }

    #[test]
    fn test_lewis() -> Result<()> {
        helper_loader(&"lewis".to_string(), "38".to_string());
        Ok(())
    }
}
