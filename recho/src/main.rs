use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Deserialize)]
struct Request {
    message: Option<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

fn handle_echo() -> String {
    if rand::thread_rng().gen_bool(0.5) {
        "🌑 dark ECHO".to_string()
    } else {
        "💡 light ECHO".to_string()
    }
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let message = match event.payload.message {
        Some(msg) => msg.trim().to_lowercase(),
        None => return Err(Error::from("Missing 'message' field in request")),
    };

    let response = if message == "echo" {
        handle_echo()
    } else {
        format!("You said: {}. Say 'echo' to play the tunnel game!", message)
    };

    Ok(Response { message: response })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}