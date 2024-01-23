use game_server::update_game_state_server::{UpdateGameState, UpdateGameStateServer};
use game_server::{UpdateStateRequest, UpdateStatus, UpdatedStateResponse};
use prost_types::Timestamp;
use tonic::{transport::Server, Request, Response, Status};

pub mod game_server {
    tonic::include_proto!("game_server");
}

#[derive(Default)]
pub struct UpdateGameStateService {}

#[tonic::async_trait]
impl UpdateGameState for UpdateGameStateService {
    async fn update(
        &self,
        request: Request<UpdateStateRequest>,
    ) -> Result<Response<UpdatedStateResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let mut processed_data = request.get_ref().data.clone();
        processed_data.push_str("_processed");
        let reply = UpdatedStateResponse {
            //message: format!("Hello {}!", request.get_ref().name),
            updated_status: UpdateStatus::Finished as i32,
            update_id: request.get_ref().update_id.clone(),
            timestamp: Some(Timestamp::default()),
            updated_data: processed_data,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    let say = UpdateGameStateService::default();
    Server::builder()
        .add_service(UpdateGameStateServer::new(say))
        .serve(addr)
        .await?;

    Ok(())
}
