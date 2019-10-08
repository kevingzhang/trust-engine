mod docker_api;
use docker_api::{DockerReqHandler};
use docker_api::docker_pb::server::{GetDockerServer};

use tokio::prelude::Future;
use futures::{future, Stream};
use log::error;
use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use tower_hyper::server::{Http, Server};

pub fn main() {
    let _ = ::env_logger::init();

    //image();


    let new_service = server::GetDockerServer::new(docker_req_handler);

    let mut server = Server::new(new_service);

    let http = Http::new().http2_only(true).clone();

    let addr = "[::1]:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve_with(sock, http.clone());
            tokio::spawn(serve.map_err(|e| error!("hyper error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve)
}
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//   let addr = "[::1]:50051".parse().unwrap();
//   let docker_req_handler = DockerReqHandler::default();

//   Server::builder()
//   .interceptor_fn(move |svc, req| {
//     let auth_header = req.headers().get("authorization").clone();

//     let authed = if let Some(auth_header) = auth_header {
//         auth_header == "Leo security agent with some-secret-token"
//     } else {
//         false
//     };

//     let fut = svc.call(req);

//     async move {
//       if authed {
//         fut.await
//       } else {
//         // Cancel the inner future since we never await it
//         // the IO never gets registered.
//         drop(fut);
//         let res = http::Response::builder()
//           .header("grpc-status", "16")
//           .body(BoxBody::empty())
//           .unwrap();
//         Ok(res)
//       }
//     }
//   })
//   .clone()
//   .serve(addr, GetDockerServer::new(docker_req_handler))
//   .await?;

//   Ok(())
// }





