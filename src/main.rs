use serde::{Deserialize, Serialize};
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
use serde_json::{Value};

enum STATE {
    START,
    IN_PROGRESS
}

#[derive(Serialize, Deserialize)]
struct JsonMessageProblem {
    name: String,
    good: String,
    bad: String,
    dag: String,
    // [["a",[]],["b",["a"]],["c",["b"]]]
    // [commit hash, [parent commit hashes]]
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonScore {
    Score: String, // {pb0: 2} or null or {pb0: null}
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonAnswer {
    Answer: String
}

// note: look into a better way potentailly to do the header... 
// could use a macro or something 
// (https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)
fn _send_data(out: Sender, header: &str,  msg: String) -> Result<()>{
    out.send(serde_json::json!({header: msg}).to_string())
}

fn send_question(out: Sender, msg: String) -> Result<()>{
    _send_data(out, "Question", msg)
}

fn send_solution(out: Sender, msg: String) -> Result<()>{
    _send_data(out, "Solution", msg)
}

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    state: STATE
}
// impl From<serde_json::Error> for io::Error{
    // fn from(e: serde_json::Error) -> Self {ws::Error{kind: ws::ErrorKind::Internal, details: "cats"}}
// }

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello WebSocket")
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        /*!
         * TODO: 
         * state logic 
         * types or representation for problem
         * errors and closing down when necessary
         * testing printing the score
         * 
         */
        if let Ok(text) = msg.as_text(){
            if let Ok(data) = serde_json::from_str::<Value>(&text){
                // https://docs.serde.rs/serde_json/fn.from_value.html
                if (data["Problem"] != Value::Null){
                    let problem : JsonMessageProblem = serde_json::from_value(data).unwrap();
                }else if (data["Answer"] != Value::Null){
                    println!("answers: {}", data["Answer"])
                }else if (data["Score"] != Value::Null){
                    // just print here
                    println!("score: {}", data["Score"])
                }else {
                    // problem
                }
            }

            match self.state {
                STATE::START => 1,
                STATE::IN_PROGRESS => 2, 
            };
            // Close the connection when we get a response from the server
            println!("Got message: {}", msg);
        }
        self.out.close(CloseCode::Normal)
        
        
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

}

fn main() {
  // Now, instead of a closure, the Factory returns a new instance of our Handler.
  connect("ws://127.0.0.1:3012", |out| Client { out: out, state: STATE::START } ).unwrap()
}