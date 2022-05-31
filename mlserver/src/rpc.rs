#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(uint64, tag = "1")]
    pub clock: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyRequest {
    #[prost(bool, tag = "1")]
    pub empty: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelPull {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub clock: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleList {
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    #[prost(string, tag = "2")]
    pub model_name: ::prost::alloc::string::String,
    #[prost(double, repeated, tag = "3")]
    pub ws1: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag = "4")]
    pub bs1: ::prost::alloc::vec::Vec<f64>,
}
#[doc = r" Generated client implementations."]
pub mod parameter_server_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ParameterServerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParameterServerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ParameterServerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ParameterServerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ParameterServerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn pull(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelPull>,
        ) -> Result<tonic::Response<super::DoubleList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.ParameterServer/pull");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn push(
            &mut self,
            request: impl tonic::IntoRequest<super::DoubleList>,
        ) -> Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.ParameterServer/push");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clock(
            &mut self,
            request: impl tonic::IntoRequest<super::Clock>,
        ) -> Result<tonic::Response<super::Clock>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.ParameterServer/clock");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod parameter_server_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ParameterServerServer."]
    #[async_trait]
    pub trait ParameterServer: Send + Sync + 'static {
        async fn pull(
            &self,
            request: tonic::Request<super::ModelPull>,
        ) -> Result<tonic::Response<super::DoubleList>, tonic::Status>;
        async fn push(
            &self,
            request: tonic::Request<super::DoubleList>,
        ) -> Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn clock(
            &self,
            request: tonic::Request<super::Clock>,
        ) -> Result<tonic::Response<super::Clock>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ParameterServerServer<T: ParameterServer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ParameterServer> ParameterServerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ParameterServerServer<T>
    where
        T: ParameterServer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rpc.ParameterServer/pull" => {
                    #[allow(non_camel_case_types)]
                    struct pullSvc<T: ParameterServer>(pub Arc<T>);
                    impl<T: ParameterServer> tonic::server::UnaryService<super::ModelPull> for pullSvc<T> {
                        type Response = super::DoubleList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelPull>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pull(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = pullSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ParameterServer/push" => {
                    #[allow(non_camel_case_types)]
                    struct pushSvc<T: ParameterServer>(pub Arc<T>);
                    impl<T: ParameterServer> tonic::server::UnaryService<super::DoubleList> for pushSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DoubleList>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).push(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = pushSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ParameterServer/clock" => {
                    #[allow(non_camel_case_types)]
                    struct clockSvc<T: ParameterServer>(pub Arc<T>);
                    impl<T: ParameterServer> tonic::server::UnaryService<super::Clock> for clockSvc<T> {
                        type Response = super::Clock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Clock>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).clock(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = clockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ParameterServer> Clone for ParameterServerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ParameterServer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ParameterServer> tonic::transport::NamedService for ParameterServerServer<T> {
        const NAME: &'static str = "rpc.ParameterServer";
    }
}
