pub mod docker_api {
    tonic::include_proto!("docker_api");
}
use http::header::HeaderValue;
use docker_api::{client::GetDockerClient, DockerInfoRequest};
use tonic::transport::Channel;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let channel = Channel::from_static("http://[::1]:50051")
        .intercept_headers(|headers| {
            headers.insert(
                "authorization",
                HeaderValue::from_static("Leo security agent with some-secret-token"),
            );
        })
        .channel();
    let mut client = GetDockerClient::new(channel);
    let request = tonic::Request::new(DockerInfoRequest {
        name: "hello".into(),
    });

    let response = client.get_docker_info(request).await?;

    println!("RESPONSE={}", response.into_inner().info);
    // let request = tonic::Request::new(DockerRequest {
    //     name: "hello".into(),
    // });
    // let response = client.get_docker_images(request).await?;

    // println!("RESPONSE={:?}", response);
    Ok(())
}
