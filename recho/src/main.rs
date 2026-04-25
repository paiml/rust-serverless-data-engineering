use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rand::thread_rng;
use recho::{function_handler, Request, Response};

async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
  function_handler(event, &mut thread_rng()).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  run(service_fn(handler)).await
}
