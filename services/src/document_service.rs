use tonic::{Request, Response, Status};
pub use document_service::document_service_server::{DocumentService, DocumentServiceServer};
use document_service::{GetFileTreeRequest, GetFileTreeResponse};

pub mod document_service {
    tonic::include_proto!("document_service");
}

#[derive(Debug, Default)]
pub struct DocumentServiceImpl {}

#[tonic::async_trait]
impl DocumentService for DocumentServiceImpl {
    async fn get_file_tree(
        &self,
        req: Request<GetFileTreeRequest>
    ) -> Result<Response<GetFileTreeResponse>, Status> {
        println!("Got request: {:?}", req);
        let reply = GetFileTreeResponse{
            response: format!("Hello {:?}!", req.into_inner().user_id).into(),
        };
        Ok(Response::new(reply))
    }
}
