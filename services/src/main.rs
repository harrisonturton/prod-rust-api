use document_service::{DocumentServiceServer, DocumentServiceImpl};
use tonic::transport::Server;

pub mod document_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting...");
    let addr = "[::1]:5000".parse()?;
    let document_service = DocumentServiceImpl::default();
    println!("Serving...");
    Server::builder()
        .add_service(DocumentServiceServer::new(document_service))
        .serve(addr)
        .await?;
    println!("Stopped.");
    Ok(())
}
