use tonic::{body::BoxBody, transport::Server, Request, Response, Status};
//use docktape::*;
use docktape::{Docker, Socket};
use tower::Service;
use serde_json;
use std::vec::Vec;
//use serde_json::Value;
pub mod docker_api {
    tonic::include_proto!("docker_pb");
}

use docker_api::{
    server::{GetDocker, GetDockerServer},
    DockerInfoReply, DockerInfoRequest, DockerImagesReply
};

#[derive(Default)]
pub struct MyGreeter {
    data: String,
}

#[tonic::async_trait]
impl GetDocker for MyGreeter {
    async fn get_docker_info(
        &self,
        request: Request<DockerInfoRequest>,
    ) -> Result<Response<DockerInfoReply>, Status> {
        println!("Got a request: {:?}", request);

        let string = &self.data;

        println!("My data: {:?}", string);
        println!("Into inner of request: {:?}", request.into_inner());

        // let reply = hello_world::HelloReply {
        //     message: "Zomg, it works!".into(),
        // };
        println!("Before wait");
        let info  = get_info();
        println!("After wait");
        let reply = docker_api::DockerInfoReply {
            info: serde_json::to_string_pretty(&info).unwrap(),
        };
        Ok(Response::new(reply))
    }

    async fn get_docker_images(
        &self,
        request: Request<
        docker_api::DockerImagesRequest>,
    ) -> Result<Response<DockerImagesReply>, Status> {
        println!("Got a request: {:?}", request);
        
        let images = get_images();

        let reply = docker_api::DockerImagesReply {
            images: images
        };
        Ok(Response::new(reply))
    }
}

fn get_info() -> serde_json::Value {
    let socket = Socket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());
    let _info : serde_json::Value = docker.get_info().unwrap();
    _info
}

fn get_images() -> Vec<docker_api::Image>{
    let mut images = Vec::new();
    let socket = Socket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());
    let _images = docker.get_images().unwrap();
    
    for image in &_images {
    	println!("{} -> repoTags: {:?}", image.id(), image.repo_tags());
        let mut tags = Vec::new();
        for tag in image.repo_tags().unwrap(){
            tags.push(tag);
        }
        let i = docker_api::Image{ id: image.id(), tags};
        images.push(i);
    }
    images
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
    .interceptor_fn(move |svc, req| {
            let auth_header = req.headers().get("authorization").clone();

            let authed = if let Some(auth_header) = auth_header {
                auth_header == "Leo security agent with some-secret-token"
            } else {
                false
            };

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
        .serve(addr, GetDockerServer::new(greeter))
        .await?;

    Ok(())
}
