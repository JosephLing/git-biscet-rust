use gitbisectrust::run;
mod visualise_dag;

use visualise_dag::vis;

fn main() {
    //#
    // vis(
    //     r#"{"Repo":{"name":"tiny-complete","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["b","a"]],["d",["c","b","a"]],["e",["d","c","b","a"]],["f",["e","d","c","b","a"]],["g",["f","e","d","c","b","a"]],["h",["g","f","e","d","c","b","a"]],["i",["h","g","f","e","d","c","b","a"]],["j",["i","h","g","f","e","d","c","b","a"]],["k",["j","i","h","g","f","e","d","c","b","a"]],["l",["k","j","i","h","g","f","e","d","c","b","a"]],["m",["l","k","j","i","h","g","f","e","d","c","b","a"]],["n",["m","l","k","j","i","h","g","f","e","d","c","b","a"]],["o",["n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["p",["o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["q",["p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["r",["q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["s",["r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["t",["s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["u",["t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["v",["u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["w",["v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["x",["w","v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["y",["x","w","v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]],["z",["y","x","w","v","u","t","s","r","q","p","o","n","m","l","k","j","i","h","g","f","e","d","c","b","a"]]]}}"#,
    //     r#"{"Instance":{"good":"b","bad":"y"}}"#,
    //     "basic1.txt",
    // );

    // vis(
    //     // "{\"Repo\":{\"name\":\"tiny-random\",\"instance_count\":10,\"dag\":[[\"a\",[]],[\"b\",[\"a\"]],[\"c\",[\"a\",\"b\"]],[\"d\",[\"c\"]],[\"e\",[\"b\",\"d\",\"a\"]],[\"f\",[\"a\"]],[\"g\",[]],[\"h\",[\"b\"]],[\"i\",[]],[\"j\",[]],[\"k\",[\"g\",\"a\",\"e\",\"b\",\"h\",\"f\",\"j\"]],[\"l\",[\"d\",\"k\",\"h\",\"g\",\"a\"]],[\"m\",[\"c\",\"i\",\"j\",\"f\",\"g\"]],[\"n\",[\"k\",\"b\",\"j\",\"f\",\"c\",\"a\",\"d\",\"e\",\"i\",\"m\",\"g\"]],[\"o\",[\"k\",\"a\",\"i\",\"g\"]],[\"p\",[\"k\",\"i\",\"g\",\"m\",\"a\",\"c\",\"b\",\"h\",\"n\",\"e\"]],[\"q\",[\"e\",\"k\",\"a\",\"l\",\"d\",\"h\",\"m\",\"p\",\"n\"]],[\"r\",[\"k\",\"n\",\"i\",\"j\",\"l\",\"d\",\"c\",\"p\"]],[\"s\",[\"d\"]],[\"t\",[\"e\",\"p\",\"h\",\"b\",\"c\",\"a\",\"i\",\"d\",\"n\",\"k\",\"m\",\"f\",\"l\",\"q\",\"o\"]],[\"u\",[\"f\",\"m\",\"l\",\"g\",\"e\",\"k\",\"b\",\"i\",\"a\",\"t\",\"s\",\"p\",\"n\",\"c\",\"h\"]],[\"v\",[\"t\",\"b\",\"f\"]],[\"w\",[\"e\",\"b\",\"h\",\"q\",\"l\",\"m\",\"n\",\"j\",\"p\"]],[\"x\",[\"d\",\"l\",\"u\",\"n\",\"b\",\"j\",\"k\",\"r\",\"s\",\"o\",\"q\",\"w\",\"c\",\"v\"]],[\"y\",[\"l\",\"o\",\"t\",\"b\"]],[\"z\",[\"c\",\"p\",\"e\",\"a\",\"b\",\"j\",\"r\",\"h\",\"y\"]]]}}",
    //     r#"{"Repo":{"name":"tiny-diamonds","instance_count":10,"dag":[["a",[]],["b",["a"]],["c",["a"]],["d",["b","c"]],["e",["d"]],["f",["d"]],["g",["e","f"]],["h",["g"]],["i",["g"]],["j",["h","i"]],["k",["j"]],["l",["j"]],["m",["k","l"]],["n",["m"]],["o",["m"]],["p",["n","o"]],["q",["p"]],["r",["p"]],["s",["q","r"]],["t",["s"]],["u",["s"]],["v",["t","u"]],["w",["v"]],["x",["v"]],["y",["w","x"]],["z",["y"]]]}}"#,
    //     r#"{"Instance":{"good":"r","bad":"y"}}"#,
    //     "basic.txt"
    // );
    // vis(
    //     r#"{"Repo":{"name":"pb0","instance_count":7,"dag":[["a",[]],["b",["a"]],["c",["b"]],["d",["c","e"]],["e",["f"]],["f",[]]]}}"#,
    //     r#"{"Instance":{"good":"a","bad":"d"}}"#,
    //     "basic2.txt",
    // );
    println!("running");
    // Now, instead of a closure, the Factory returns a new instance of our Handler.
    // run("ws://129.12.44.229:1234".to_string());
    run("ws://129.12.44.246:1234".to_string());

    
    println!("finished");
}
