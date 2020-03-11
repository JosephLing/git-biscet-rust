use gitbisectrust::json_types::*;
use std::fs::File;
use std::io::prelude::*;
pub fn vis(data: &str, data2: &str, name: &str) {
    let problem = serde_json::from_str::<JsonMessageProblem>(data).unwrap();
    let instance = serde_json::from_str::<JsonInstanceGoodBad>(data2).unwrap().Instance;
    let mut debug: String = "digraph G {\n".to_string();
    debug += &format!("node [shape = doublecircle, color=red]; {}\n", instance.bad);
    debug += &format!("node [shape = doublecircle, color=green]; {}\n", instance.good);
    debug += "node [shape = circle, color=black];\n";
    for node in problem.Repo.dag {
        for parent in node.parents {
            debug = debug + &format!("{} -> {}\n", node.commit, parent);
        }
    }
    debug = debug + &"}".to_string();
    let mut file = File::create(name).unwrap();
    file.set_len(0).unwrap();
    file.write_all(debug.as_bytes()).unwrap();
    println!("{}", debug.to_string());
}
