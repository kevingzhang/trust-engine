#![allow(non_snake_case)]
use tonic::{Request, Response, Status};
use docktape::{Docker, Socket};
use serde_json;
use std::vec::Vec;


pub mod docker_pb {
	tonic::include_proto!("docker_pb");
}

use docker_pb::{
	server::{GetDocker},
	DockerInfoReply, DockerInfoRequest, DockerImagesReply
};

#[derive(Default)]
pub struct DockerReqHandler {
//    data: String,
}

#[tonic::async_trait]
impl GetDocker for DockerReqHandler {
	async fn get_docker_info(
		&self,
		request: Request<DockerInfoRequest>,
	) -> Result<Response<DockerInfoReply>, Status> {
		//println!("Into inner of request: {:?}", &request.into_inner());

		let req_path = request.into_inner().path;
		let info  = get_info();
		
		let info_in_req_path = | req_path | {
			if req_path != "" {
				serde_json::to_string_pretty(&info[req_path]).unwrap()
			}else{
				serde_json::to_string_pretty(&info).unwrap()
			}
		};
		
		let reply = docker_pb::DockerInfoReply {
			info: info_in_req_path(req_path),
		};
		Ok(Response::new(reply))
	
	}

	async fn get_docker_images(
		&self,
		request: Request<
		docker_pb::DockerImagesRequest>,
	) -> Result<Response<DockerImagesReply>, Status> {
		//println!("Got a request: {:?}", request);
		
		let images = get_images();

		let reply = docker_pb::DockerImagesReply {
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

fn get_images() -> std::vec::Vec<docker_pb::Image>{
	let socket = Socket::new("/var/run/docker.sock");
	let mut docker = Docker::new(socket.clone());
	let images_value = docker.get_images_value();
	let arr_images: Vec<Image> = serde_json::from_value(images_value.unwrap()).unwrap();
	let mut ret = std::vec::Vec::new();
	for c in arr_images{
		//println!("c : {:#?}", c);
		let image : docker_pb::Image = c.to_docker_pb_image();
		//println!("c convert to image: {:#?}", image);
		ret.push(image);
	}

	ret
}

#[derive(Default, serde::Deserialize, Debug)]
pub struct Image{
	pub Id: String,
	pub Created: u64,
	pub ParentId: Option<String>,
	pub RepoDigests: Option<Vec<String>>,
	pub Size: u64,
	pub VirtualSize: u64,
	pub Labels: Option<std::collections::HashMap<String, String>>,
	pub RepoTags: Option<Vec<String>>
}
impl Image{
	pub fn to_docker_pb_image(&self) ->docker_pb::Image {
		docker_pb::Image{
			id: self.Id.to_string(),
			created: self.Created,
			parent_id: self.ParentId.clone().unwrap(),
			repo_digests: self.RepoDigests.clone().unwrap(),
			size: self.Size,
			virtual_size: self.VirtualSize,
			labels: self.Labels.clone().unwrap_or(std::collections::HashMap::new()),
			tags: self.RepoTags.clone().unwrap_or(Vec::new()),
			
		}
	}
}

/******
 * 
 * 
 * pub struct Image {
	#[prost(string, tag = "1")]
	pub id: std::string::String,
	#[prost(uint64, tag = "2")]
	pub created: u64,
	#[prost(string, tag = "3")]
	pub parent_id: std::string::String,
	#[prost(string, repeated, tag = "4")]
	pub repo_digests: ::std::vec::Vec<std::string::String>,
	#[prost(uint64, tag = "5")]
	pub size: u64,
	#[prost(uint64, tag = "6")]
	pub virtual_size: u64,
	#[prost(map = "string, string", tag = "7")]
	pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
	#[prost(string, repeated, tag = "8")]
	pub tags: ::std::vec::Vec<std::string::String>,
}
 */

