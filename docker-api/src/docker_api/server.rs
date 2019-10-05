use tonic::{transport::Server, Request, Response, Status};
//use docktape::*;
use docktape::{Docker, Socket};

use serde_json;
//use serde_json::Value;
pub mod docker_api {
    tonic::include_proto!("docker_api");
}

use docker_api::{
    server::{GetDocker, GetDockerServer},
    DockerReply, DockerRequest,DockerInfoReply, DockerInfoRequest
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
        request: Request<DockerRequest>,
    ) -> Result<Response<DockerReply>, Status> {
        println!("Got a request: {:?}", request);

        let string = &self.data;

        println!("My data: {:?}", string);

        // let reply = hello_world::HelloReply {
        //     message: "Zomg, it works!".into(),
        // };
        println!("Before wait");
        get_images();
        println!("After wait ");
        let reply = docker_api::DockerReply {
            message: "imnages".to_string(),
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

fn get_images() {
    let socket = Socket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());
    let _images = docker.get_images().unwrap();
    
    for image in &_images {
    	println!("{} -> repoTags: {:?}", image.id(), image.repo_tags());
    }
    ;
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        .serve(addr, GetDockerServer::new(greeter))
        .await?;

    Ok(())
}
