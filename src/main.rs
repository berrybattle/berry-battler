use game_server::update_game_state_server::{UpdateGameState, UpdateGameStateServer};
use game_server::{
    UnitDirectionVector, UnitPosition, UnitState, UpdateStateRequest, UpdateStatus,
    UpdatedStateResponse,
};
use rand::Rng;
use std::thread;
use std::time::Instant;
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
        mut request: Request<UpdateStateRequest>,
    ) -> Result<Response<UpdatedStateResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let updated_state = battler_logic(request.get_mut());

        Ok(Response::new(updated_state))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    let update = UpdateGameStateService::default();
    Server::builder()
        .add_service(UpdateGameStateServer::new(update))
        .serve(addr)
        .await?;

    Ok(())
}

fn battler_logic(current_state: &UpdateStateRequest) -> UpdatedStateResponse {
    let mut rng = rand::thread_rng();
    let start_time = Instant::now();

    let updated_units: Vec<UnitState> = current_state
        .units
        .iter()
        .map(|unit| UnitState {
            id: unit.id,
            unit_type: unit.unit_type,
            position: Some(UnitPosition {
                x: rng.gen::<f32>(),
                y: rng.gen::<f32>(),
                layer: rng.gen::<u32>(),
                direction: Some(UnitDirectionVector {
                    x: rng.gen::<f32>(),
                    y: rng.gen::<f32>(),
                }),
            }),
            tag: "Updated Unit Tag".to_string(),
        })
        .collect();

    let parsing_elapsed = start_time.elapsed();
    // Simulate processing more expensive work
    thread::sleep(parsing_elapsed * current_state.multiplier);

    UpdatedStateResponse {
        updated_status: UpdateStatus::Finished as i32,
        update_id: current_state.update_id,
        units: updated_units,
        multiplier: current_state.multiplier,
        single_pass_elapsed_time: u64::try_from(parsing_elapsed.as_micros()).unwrap(),
    }
}
