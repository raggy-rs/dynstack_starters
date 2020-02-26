mod brp;
mod data_model;
mod heuristics;
mod search;

use data_model::{CraneSchedule, World};
use protobuf::Message;
#[derive(Debug, Copy, Clone)]
enum OptimizerType {
    RuleBased,
    ModelBased,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let world_socket_addr = args.next().expect("Expected address of world socket");
    let crane_socket_addr = args.next().expect("Expected address of crane socket");
    let sim_id = args.next().expect("Expected simulation id");

    let opt_type = if args.next().is_some() {
        OptimizerType::ModelBased
    } else {
        OptimizerType::RuleBased
    };
    println!("{:?}", opt_type);

    let ctx = zmq::Context::new();
    let crane_socket = ctx.socket(zmq::DEALER)?;
    crane_socket.set_identity(sim_id.as_bytes())?;
    crane_socket.connect(&crane_socket_addr)?;
    println!("Connected crane");

    let world_socket = ctx.socket(zmq::DEALER)?;
    world_socket.set_identity(sim_id.as_bytes())?;
    world_socket.connect(&world_socket_addr)?;
    println!("Connected world");

    while let Ok(msg) = world_socket.recv_msg(0) {
        let world = protobuf::parse_from_bytes::<World>(&msg)?;
        let sequence = world.get_Crane().get_Schedule().get_SequenceNr();
        if let Some(mut new_schedule) = optimize_crane_schedule(&world, opt_type) {
            // set sequence number because the simulation only accepts Schedules with increasing sequence numbers.
            new_schedule.set_SequenceNr(sequence + 1);
            println!("send {:?}", new_schedule);
            let message = new_schedule.write_to_bytes()?;
            crane_socket.send(message, 0)?;
        }
    }
    Ok(())
}

fn optimize_crane_schedule(world: &World, opt: OptimizerType) -> Option<CraneSchedule> {
    if !world.get_Crane().get_Schedule().get_Moves().is_empty() {
        // Leave the existing schedule alone
        return None;
    }
    let schedule = match opt {
        OptimizerType::RuleBased => heuristics::calculate_schedule(world),
        OptimizerType::ModelBased => brp::calculate_schedule(world),
    };

    if schedule.get_Moves().is_empty() {
        // avoid sending empty schedules
        None
    } else {
        Some(schedule)
    }
}
