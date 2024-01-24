use game_server::update_game_state_server::{UpdateGameState, UpdateGameStateServer};
use game_server::{
    UnitDirectionVector, UnitPosition, UnitState, UpdateStateRequest, UpdateStatus,
    UpdatedStateResponse,
};
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
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
        Ok(Response::new(simulate_game_update(request.get_mut())))
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

fn simulate_game_update(current_state: &UpdateStateRequest) -> UpdatedStateResponse {
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

    // Simulate running a pathing algorithm like A*
    let proc_duration = Duration::from_nanos(
        current_state.per_unit_proc_time_ns * current_state.units.len() as u64,
    );
    thread::sleep(proc_duration);
    let parsing_elapsed = start_time.elapsed();

    UpdatedStateResponse {
        updated_status: UpdateStatus::Finished as i32,
        update_id: current_state.update_id,
        units: updated_units,
        per_unit_proc_time_ns: current_state.per_unit_proc_time_ns,
        single_pass_elapsed_time_us: u64::try_from(parsing_elapsed.as_micros()).unwrap(),
    }
}
