#![deny(warnings, rust_2018_idioms)]
pub mod docker_api {
	include!(concat!(env!("OUT_DIR"), "/docker_pb.rs"));
}
use http::header::HeaderValue;
use docker_api::{client::GetDockerClient, DockerInfoRequest, DockerImagesRequest, DockerImagesReply};
use futures::Future;
use hyper::client::connect::{Destination, HttpConnector};
use tower_grpc::Request;
use tower_hyper::{client, util};
use tower_util::MakeService;

// async fn main() -> Result<(), Box<dyn std::error::Error>> {

// 	let channel = Channel::from_static("http://[::1]:50051")
// 		.intercept_headers(|headers| {
// 			headers.insert(
// 				"authorization",
// 				HeaderValue::from_static("Leo security agent with some-secret-token"),
// 			);
// 		})
// 		.channel();
// 	let mut client = GetDockerClient::new(channel);
// 	let request = tonic::Request::new(DockerInfoRequest {
// 		path: "".into(),
// 	});

// 	let response = client.get_docker_info(request).await?;

// 	println!("RESPONSE={}", response.into_inner().info);
// 	let request = tonic::Request::new(DockerImagesRequest {
// 		req: "images".into(),
// 	});
// 	let response = client.get_docker_images(request).await?;

// 	println!("RESPONSE={:#?}", response.into_inner().images);
// 	Ok(())
// }



pub fn main() {
    let _ = ::env_logger::init();

    let uri: http::Uri = format!("http://[::1]:50051").parse().unwrap();

    let dst = Destination::try_from_uri(uri.clone()).unwrap();
    let connector = util::Connector::new(HttpConnector::new(4));
    let settings = client::Builder::new().http2_only(true).clone();
    let mut make_client = client::Connect::with_builder(connector, settings);

    let call_for_docker_info = make_client
        .make_service(dst)
        .map_err(|e| panic!("connect error: {:?}", e))
        .and_then(move |conn| {
            
            let conn = tower_request_modifier::Builder::new()
                .set_origin(uri)
                .build(conn)
                .unwrap();

            // Wait until the client is ready...
            GetDockerClient::new(conn).ready()
        })
        .and_then(|mut client| {
            
            client.call_for_docker_info(Request::new(DockerInfoRequest {
                path: "".to_string(),
            }))
        })
        .and_then(|response| {
            println!("RESPONSE = {:?}", response);
            Ok(())
        })
        .map_err(|e| {
            println!("ERR = {:?}", e);
        });

    tokio::run(call_for_docker_info);
}
