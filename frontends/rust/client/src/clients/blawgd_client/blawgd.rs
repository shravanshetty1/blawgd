#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(uint64, tag = "1")]
    pub max_post_count: u64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(string, repeated, tag = "2")]
    pub post_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetTimelineRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int64, tag = "2")]
    pub page: i64,
    #[prost(int64, tag = "3")]
    pub per_page: i64,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsByAccountRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int64, tag = "2")]
    pub page: i64,
    #[prost(int64, tag = "3")]
    pub per_page: i64,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsByParentPostRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int64, tag = "2")]
    pub page: i64,
    #[prost(int64, tag = "3")]
    pub per_page: i64,
    #[prost(string, tag = "4")]
    pub parent_post: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(map = "string, bytes", tag = "1")]
    pub data:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(map = "string, bytes", tag = "2")]
    pub proofs:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
// Views

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PostView {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub creator: ::core::option::Option<AccountInfo>,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub parent_post: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub comments_count: u64,
    #[prost(uint64, tag = "6")]
    pub like_count: u64,
    #[prost(uint64, tag = "7")]
    pub repost_count: u64,
    #[prost(message, optional, boxed, tag = "8")]
    pub repost_parent: ::core::option::Option<::prost::alloc::boxed::Box<PostView>>,
}
// Models

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub parent_post: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub comments_count: u64,
    #[prost(uint64, tag = "6")]
    pub like_count: u64,
    #[prost(uint64, tag = "7")]
    pub repost_count: u64,
    #[prost(message, optional, boxed, tag = "8")]
    pub repost_parent: ::core::option::Option<::prost::alloc::boxed::Box<Post>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub photo: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub following_count: u64,
    #[prost(uint64, tag = "6")]
    pub followers_count: u64,
    #[prost(uint64, tag = "7")]
    pub post_count: u64,
}
// Transactions

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgRepost {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub post_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePost {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub parent_post: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAccountInfo {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub photo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgFollow {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgStopFollow {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgLikePost {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub post_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub amount: u64,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blawgd.Query/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_posts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostsRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blawgd.Query/GetPosts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_posts_by_parent_post(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostsByParentPostRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blawgd.Query/GetPostsByParentPost");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_posts_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostsByAccountRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blawgd.Query/GetPostsByAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_timeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTimelineRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/blawgd.Query/GetTimeline");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QueryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QueryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryClient {{ ... }}")
        }
    }
}
