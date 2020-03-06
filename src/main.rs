use gitbisectrust::run;

fn main() {
    env_logger::init();
    println!("running");
    // Now, instead of a closure, the Factory returns a new instance of our Handler.
    run("ws://129.12.44.229:1234".to_string());
    println!("cats");
}
