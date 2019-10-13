
pub mod docker_api {
	tonic::include_proto!("docker_pb");
}
use std::{thread, time};
use http::header::HeaderValue;
use docker_api::{client::GetDockerClient, DockerInfoRequest, DockerImagesRequest, DockerImagesReply};
use tonic::transport::Channel;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	loop{
		let result = run().await;
		match(result){
			Ok(r)=>{
				
			},
			Err(e)=>{
				println!("Error from run function {:#?}", e);
			}
		}
		println!("Will try after 10 sec");
		thread::sleep(time::Duration::from_secs(10));
	}
	Ok(())
}

async fn run() ->Result<(), Box<dyn std::error::Error>>{
	
	let channel = Channel::from_static("http://[::1]:50051")
		.intercept_headers(|headers| {
			headers.insert(
				"authorization",
				HeaderValue::from_static("Leo security agent with some-secret-token"),
			);
		})
		.channel();
	println!("channel");
	let mut client = GetDockerClient::new(channel);
	let request = tonic::Request::new(DockerInfoRequest {
		path: "".into(),
	});
	println!("request");
	let response = client.get_docker_info(request).await?;
	println!("response");
	println!("RESPONSE={}", response.into_inner().info);
	let request = tonic::Request::new(DockerImagesRequest {
		req: "images".into(),
	});
	let response = client.get_docker_images(request).await?;

	println!("RESPONSE={:#?}", response.into_inner().images);
	Ok(())
}
