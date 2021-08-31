//import "gogoproto/gogo.proto";

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetTimelineRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub index: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetTimelineResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<PostView>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetFollowingsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetFollowingsResponse {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostResponse {
    #[prost(message, optional, tag = "1")]
    pub post: ::core::option::Option<PostView>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsByAccountRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub index: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsByAccountResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<PostView>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetProfileInfoRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub height: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetProfileInfoResponse {
    #[prost(map = "string, message", tag = "1")]
    pub account_info: ::std::collections::HashMap<::prost::alloc::string::String, AccountInfo>,
    #[prost(map = "string, message", tag = "2")]
    pub following_count:
        ::std::collections::HashMap<::prost::alloc::string::String, FollowingCount>,
    #[prost(map = "string, bytes", tag = "3")]
    pub proofs:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsByParentPostRequest {
    #[prost(string, tag = "1")]
    pub parent_post: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub index: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetPostsByParentPostResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<PostView>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub parent_post: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub block_no: i64,
    #[prost(string, tag = "6")]
    pub metadata: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FollowingCount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub count: u64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PostView {
    #[prost(message, optional, tag = "1")]
    pub creator: ::core::option::Option<AccountInfo>,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub parent_post: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub block_no: i64,
    #[prost(string, tag = "6")]
    pub metadata: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub photo: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AccountInfoView {
    #[prost(message, optional, tag = "1")]
    pub account_info: ::core::option::Option<AccountInfo>,
    #[prost(int64, tag = "2")]
    pub following_count: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Following {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub followings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePost {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub parent_post: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAccountInfo {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub photo: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
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
        pub async fn get_post(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostRequest>,
        ) -> Result<tonic::Response<super::GetPostResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/shravanshetty1.samachar.samachar.Query/GetPost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_posts_by_parent_post(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostsByParentPostRequest>,
        ) -> Result<tonic::Response<super::GetPostsByParentPostResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/shravanshetty1.samachar.samachar.Query/GetPostsByParentPost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_posts_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostsByAccountRequest>,
        ) -> Result<tonic::Response<super::GetPostsByAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/shravanshetty1.samachar.samachar.Query/GetPostsByAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_timeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTimelineRequest>,
        ) -> Result<tonic::Response<super::GetTimelineResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/shravanshetty1.samachar.samachar.Query/GetTimeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_profile_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProfileInfoRequest>,
        ) -> Result<tonic::Response<super::GetProfileInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/shravanshetty1.samachar.samachar.Query/GetProfileInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_followings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFollowingsRequest>,
        ) -> Result<tonic::Response<super::GetFollowingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/shravanshetty1.samachar.samachar.Query/GetFollowings",
            );
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
