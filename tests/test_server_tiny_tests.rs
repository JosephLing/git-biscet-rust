mod server;
extern crate gitbisectrust;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::fs;
use std::thread;
use gitbisectrust::json_types::*;

use serde_json::Value;
use serde_json::*;

use serde::{Deserialize, Serialize};

use gitbisectrust::run;
// use gitbisectrust::json_types::*;

pub use crate::server::create_single_repo_server;

#[derive(Serialize, Deserialize, Clone)]
struct JsonFileProblem {
    bads: HashSet<String>,
    good: String,
    bad: String,
    solution: String,
    name: String,
    statement: String
}

#[cfg(test)]
mod tinyJsonExamples {
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

    fn helper_file2(filename: &str, host: String, index: usize) -> Result<()> {
        println!("AAHHAHHSAHAHAHAHHAHAHAHAHHAHHAHAHAHH");
        println!("AAHHAHHSAHAHAHAHHAHAHAHAHHAHHAHAHAHH");
        println!("AAHHAHHSAHAHAHAHHAHAHAHAHHAHHAHAHAHH");
        println!("AAHHAHHSAHAHAHAHHAHAHAHAHHAHHAHAHAHH");
        let contents = fs::read_to_string("./tests/test_data/".to_string() + &filename).expect("Something went wrong reading the file");
        let list_data = &serde_json::from_str::<Value>(&contents).unwrap()[index];
        println!("{:?}", list_data);
        let data = serde_json::from_value::<JsonFileProblem>(list_data.clone()).unwrap();
        let single_instance = serde_json::json!(
            {"Instance":{"good": data.good, "bad": data.bad }}
        );
        let instance: Vec<String> = vec![single_instance.to_string()];



        
        let data2 = data.clone();
        {
            // let problem = serde_json::from_str::<JsonMessageProblem>(&data.statement).unwrap();
            // let instance_temp = serde_json::from_str::<JsonInstanceGoodBad>(&single_instance.to_string()).unwrap().Instance;
            // let mut debug: String = "digraph G {\n".to_string();
            // debug += &format!("node [shape = doublecircle, color=blue]; {}\n", instance_temp.bad);
            // debug += &format!("node [shape = doublecircle, color=green]; {}\n", instance_temp.good);
            // debug += &format!("node [shape = doublecircle, color=yellow]; {}\n", data.solution);

            // for b in data.bads{
            //     debug += &format!("node [shape = doublecircle, color=red]; {}\n", b);
            // }
            // debug += "node [shape = circle, color=black];\n";
            // for node in problem.Repo.dag {
            //     for parent in node.parents {
            //         debug = debug + &format!("{} -> {}\n", node.commit, parent);
            //     }
            // }
            // debug = debug + &"}".to_string();
            // let mut file = File::create("tiny chain.dot").unwrap();
            // file.set_len(0).unwrap();
            // file.write_all(debug.as_bytes()).unwrap();
        }
        helper(
            &host,
            vec![data2.bads],
            data2.statement,
            instance,
            vec![data2.solution],
            false,
        );

        
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_tiny_examples0() -> Result<()> {
        helper_file2("test_tiny_examples.json", "3000".to_string(), 0);
        Ok(())
    }

    #[test]
    fn test_tiny_examples1() -> Result<()> {
        helper_file2("test_tiny_examples.json", "3000".to_string(), 0);
        helper_file2("test_tiny_examples.json", "3001".to_string(), 1);
        helper_file2("test_tiny_examples.json", "3002".to_string(), 2);
        helper_file2("test_tiny_examples.json", "3003".to_string(), 3);
        helper_file2("test_tiny_examples.json", "3004".to_string(), 4);
        helper_file2("test_tiny_examples.json", "3005".to_string(), 5);
        helper_file2("test_tiny_examples.json", "3006".to_string(), 6);
        helper_file2("test_tiny_examples.json", "3007".to_string(), 7);
        helper_file2("test_tiny_examples.json", "3008".to_string(), 8);
        helper_file2("test_tiny_examples.json", "3009".to_string(), 9);
        helper_file2("test_tiny_examples.json", "3010".to_string(), 10);
        helper_file2("test_tiny_examples.json", "3011".to_string(), 11);
        helper_file2("test_tiny_examples.json", "3012".to_string(), 12);
        helper_file2("test_tiny_examples.json", "3013".to_string(), 13);
        helper_file2("test_tiny_examples.json", "3014".to_string(), 14);
        helper_file2("test_tiny_examples.json", "3015".to_string(), 15);
        helper_file2("test_tiny_examples.json", "3016".to_string(), 16);
        helper_file2("test_tiny_examples.json", "3017".to_string(), 17);
        helper_file2("test_tiny_examples.json", "3018".to_string(), 18);
        helper_file2("test_tiny_examples.json", "3019".to_string(), 19);
        helper_file2("test_tiny_examples.json", "3020".to_string(), 20);
        helper_file2("test_tiny_examples.json", "3021".to_string(), 21);
        helper_file2("test_tiny_examples.json", "3022".to_string(), 22);
        helper_file2("test_tiny_examples.json", "3023".to_string(), 23);
        helper_file2("test_tiny_examples.json", "3024".to_string(), 24);
        helper_file2("test_tiny_examples.json", "3025".to_string(), 25);
        helper_file2("test_tiny_examples.json", "3026".to_string(), 26);
        helper_file2("test_tiny_examples.json", "3027".to_string(), 27);
        helper_file2("test_tiny_examples.json", "3028".to_string(), 28);
        helper_file2("test_tiny_examples.json", "3029".to_string(), 29);
        helper_file2("test_tiny_examples.json", "3030".to_string(), 30);
        helper_file2("test_tiny_examples.json", "3031".to_string(), 31);
        helper_file2("test_tiny_examples.json", "3032".to_string(), 32);
        helper_file2("test_tiny_examples.json", "3033".to_string(), 33);
        helper_file2("test_tiny_examples.json", "3034".to_string(), 34);
        helper_file2("test_tiny_examples.json", "3035".to_string(), 35);
        helper_file2("test_tiny_examples.json", "3036".to_string(), 36);
        helper_file2("test_tiny_examples.json", "3037".to_string(), 37);
        helper_file2("test_tiny_examples.json", "3038".to_string(), 38);
        helper_file2("test_tiny_examples.json", "3039".to_string(), 39);
        helper_file2("test_tiny_examples.json", "3040".to_string(), 40);
        helper_file2("test_tiny_examples.json", "3041".to_string(), 41);
        helper_file2("test_tiny_examples.json", "3042".to_string(), 42);
        helper_file2("test_tiny_examples.json", "3043".to_string(), 43);
        helper_file2("test_tiny_examples.json", "3044".to_string(), 44);
        Ok(())
    }
}
