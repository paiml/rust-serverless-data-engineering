use lambda_runtime::{Error, LambdaEvent};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
  pub message: Option<String>,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Response {
  pub message: String,
}

pub fn handle_echo<R: Rng>(rng: &mut R) -> String {
  if rng.gen_bool(0.5) {
    "🌑 dark ECHO".to_string()
  } else {
    "💡 light ECHO".to_string()
  }
}

pub async fn function_handler<R: Rng>(event: LambdaEvent<Request>, rng: &mut R) -> Result<Response, Error> {
  let message = match event.payload.message {
    Some(msg) => msg.trim().to_lowercase(),
    None => return Err(Error::from("Missing 'message' field in request")),
  };

  let response = if message == "echo" {
    handle_echo(rng)
  } else {
    format!("You said: {}. Say 'echo' to play the tunnel game!", message)
  };

  Ok(Response { message: response })
}

#[cfg(test)]
mod tests {
  use lambda_runtime::{Context, LambdaEvent};
  use rand::rngs::mock::StepRng;

  use super::*;

  fn event(message: Option<&str>) -> LambdaEvent<Request> {
    LambdaEvent::new(
      Request {
        message: message.map(str::to_string),
      },
      Context::default(),
    )
  }

  #[test]
  fn handle_echo_returns_dark_when_rng_low() {
    let mut rng = StepRng::new(0, 0);
    assert_eq!(handle_echo(&mut rng), "🌑 dark ECHO");
  }

  #[test]
  fn handle_echo_returns_light_when_rng_high() {
    let mut rng = StepRng::new(u64::MAX, 0);
    assert_eq!(handle_echo(&mut rng), "💡 light ECHO");
  }

  #[tokio::test]
  async fn function_handler_echo_keyword_returns_dark_with_low_rng() {
    let mut rng = StepRng::new(0, 0);
    let resp = function_handler(event(Some("echo")), &mut rng).await.unwrap();
    assert_eq!(resp.message, "🌑 dark ECHO");
  }

  #[tokio::test]
  async fn function_handler_echo_keyword_returns_light_with_high_rng() {
    let mut rng = StepRng::new(u64::MAX, 0);
    let resp = function_handler(event(Some("echo")), &mut rng).await.unwrap();
    assert_eq!(resp.message, "💡 light ECHO");
  }

  #[tokio::test]
  async fn function_handler_non_echo_message_is_normalized_and_quoted_back() {
    let mut rng = StepRng::new(0, 0);
    let resp = function_handler(event(Some("  Hello  ")), &mut rng).await.unwrap();
    assert_eq!(resp.message, "You said: hello. Say 'echo' to play the tunnel game!");
  }

  #[tokio::test]
  async fn function_handler_missing_message_field_is_an_error() {
    let mut rng = StepRng::new(0, 0);
    let err = function_handler(event(None), &mut rng).await.unwrap_err();
    assert!(err.to_string().contains("Missing 'message' field"));
  }

  #[test]
  fn request_deserializes_with_or_without_message_field() {
    let with: Request = serde_json::from_str(r#"{"message": "echo"}"#).unwrap();
    assert_eq!(with.message.as_deref(), Some("echo"));
    let without: Request = serde_json::from_str("{}").unwrap();
    assert_eq!(without.message, None);
  }

  #[test]
  fn response_serializes_message_field() {
    let resp = Response {
      message: "hi".to_string(),
    };
    let json = serde_json::to_string(&resp).unwrap();
    assert_eq!(json, r#"{"message":"hi"}"#);
  }
}
