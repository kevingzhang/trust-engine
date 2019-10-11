use tower::Service;
use tonic::{body::BoxBody, transport::Server};

mod docker_api;

use docker_api::{DockerReqHandler};
use docker_api::docker_pb::server::{GetDockerServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse().unwrap();
  let docker_req_handler = DockerReqHandler::default();

  Server::builder()
  .interceptor_fn(move |svc, req| {
    let auth_header = req.headers().get("authorization").clone();

    // let authed = if let Some(auth_header) = auth_header {
    //     auth_header == "Leo security agent with some-secret-token"
    // } else {
    //     false
    // };
    let authed = true;

    let fut = svc.call(req);

    async move {
      if authed {
        fut.await
      } else {
        // Cancel the inner future since we never await it
        // the IO never gets registered.
        drop(fut);
        let res = http::Response::builder()
          .header("grpc-status", "16")
          .body(BoxBody::empty())
          .unwrap();
        Ok(res)
      }
    }
  })
  .clone()
  .serve(addr, GetDockerServer::new(docker_req_handler))
  .await?;

  Ok(())
}
