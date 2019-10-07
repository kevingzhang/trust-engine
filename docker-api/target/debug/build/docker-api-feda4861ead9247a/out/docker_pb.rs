// // The response message containing the greetings
// message HelloReply {
//   string message = 1;
// }

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerImagesReply {
    #[prost(message, repeated, tag = "1")]
    pub images: ::std::vec::Vec<Image>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerInfoRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerInfoReply {
    #[prost(string, tag = "1")]
    pub info: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerImagesRequest {
    #[prost(string, tag = "1")]
    pub req: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GetDockerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GetDockerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            tonic::transport::Endpoint::new(dst).map(|c| Self::new(c.channel()))
        }
    }
    impl<T> GetDockerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
        <T::ResponseBody as HttpBody>::Data: Into<bytes::Bytes> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        #[doc = r" Check if the service is ready."]
        pub async fn ready(&mut self) -> Result<(), tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })
        }
        pub async fn get_docker_info(
            &mut self,
            request: tonic::Request<super::DockerInfoRequest>,
        ) -> Result<tonic::Response<super::DockerInfoReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/docker_pb.GetDocker/GetDockerInfo");
            self.inner.unary(request, path, codec).await
        }
        pub async fn get_docker_images(
            &mut self,
            request: tonic::Request<super::DockerImagesRequest>,
        ) -> Result<tonic::Response<super::DockerImagesReply>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/docker_pb.GetDocker/GetDockerImages");
            self.inner.unary(request, path, codec).await
        }
    }
    impl<T: Clone> Clone for GetDockerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GetDockerServer."]
    #[async_trait]
    pub trait GetDocker: Send + Sync + 'static {
        async fn get_docker_info(
            &self,
            request: tonic::Request<super::DockerInfoRequest>,
        ) -> Result<tonic::Response<super::DockerInfoReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn get_docker_images(
            &self,
            request: tonic::Request<super::DockerImagesRequest>,
        ) -> Result<tonic::Response<super::DockerImagesReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetDockerServer<T: GetDocker> {
        inner: Arc<T>,
    }
    #[derive(Clone, Debug)]
    #[doc(hidden)]
    pub struct GetDockerServerSvc<T: GetDocker> {
        inner: Arc<T>,
    }
    impl<T: GetDocker> GetDockerServer<T> {
        #[doc = "Create a new GetDockerServer from a type that implements GetDocker."]
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self::from_shared(inner)
        }
        pub fn from_shared(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: GetDocker> GetDockerServerSvc<T> {
        pub fn new(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: GetDocker, R> Service<R> for GetDockerServer<T> {
        type Response = GetDockerServerSvc<T>;
        type Error = Never;
        type Future = Ready<Result<Self::Response, Self::Error>>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, _: R) -> Self::Future {
            ok(GetDockerServerSvc::new(self.inner.clone()))
        }
    }
    impl<T: GetDocker> Service<http::Request<HyperBody>> for GetDockerServerSvc<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/docker_pb.GetDocker/GetDockerInfo" => {
                    struct GetDockerInfo<T: GetDocker>(pub Arc<T>);
                    impl<T: GetDocker> tonic::server::UnaryService<super::DockerInfoRequest> for GetDockerInfo<T> {
                        type Response = super::DockerInfoReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DockerInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_docker_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetDockerInfo(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/docker_pb.GetDocker/GetDockerImages" => {
                    struct GetDockerImages<T: GetDocker>(pub Arc<T>);
                    impl<T: GetDocker> tonic::server::UnaryService<super::DockerImagesRequest> for GetDockerImages<T> {
                        type Response = super::DockerImagesReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DockerImagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_docker_images(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetDockerImages(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
}
