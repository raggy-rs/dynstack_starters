mod brp;
mod data_model;
mod search;

use brp::{prioritize_by_due_date, BrpState, Move};
use data_model::{CraneMove, CraneSchedule, World};
use protobuf::Message;
use search::depth_first_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let world_socket_addr = args.next().expect("Expected address of world socket");
    let crane_socket_addr = args.next().expect("Expected address of crane socket");
    let sim_id = args.next().expect("Expected simulation id");

    let ctx = zmq::Context::new();
    let crane_socket = ctx.socket(zmq::DEALER)?;
    crane_socket.set_identity(sim_id.as_bytes())?;
    crane_socket.connect(&crane_socket_addr)?;
    println!("Connected crane");

    let world_socket = ctx.socket(zmq::DEALER)?;
    world_socket.set_identity(sim_id.as_bytes())?;
    world_socket.connect(&world_socket_addr)?;
    println!("Connected world");

    let mut sequence = 0;

    while let Ok(msg) = world_socket.recv_msg(0) {
        if msg.get_more() {
            println!("{:?}", msg);
            continue;
        }
        let world = protobuf::parse_from_bytes::<World>(&msg)?;

        if let Some(mut new_schedule) = optimize_crane_schedule(&world) {
            // set sequence number because the simulation only accepts Schedules with increasing sequence numbers.
            new_schedule.set_SequenceNr(sequence);
            sequence += 1;
            println!("send {:?}", new_schedule);
            let message = new_schedule.write_to_bytes()?;
            crane_socket.send(message, 0)?;
        }
    }
    Ok(())
}

fn optimize_crane_schedule(world: &World) -> Option<CraneSchedule> {
    if !world.get_Crane().get_Schedule().get_Moves().is_empty() {
        // Leave the existing schedule alone
        return None;
    }

    let priorities = prioritize_by_due_date(world);
    let initial_state = BrpState::new(world, priorities);
    let solution = depth_first_search(initial_state);
    let schedule = create_schedule_from_solution(world, solution);

    if schedule.get_Moves().is_empty() {
        // avoid sending empty schedules
        None
    } else {
        Some(schedule)
    }
}

/// Translates the BRP solution into a CraneSchedule
fn create_schedule_from_solution(world: &World, moves: Vec<Move>) -> CraneSchedule {
    let mut schedule = CraneSchedule::new();
    let handover = world.get_Handover();
    let is_ready = handover.get_Ready();
    for opt_mov in moves.into_iter().take(3) {
        if !is_ready && opt_mov.tgt() == handover.get_Id() {
            break;
        }
        let mut mov = CraneMove::new();
        mov.set_BlockId(opt_mov.block());
        mov.set_SourceId(opt_mov.src());
        mov.set_TargetId(opt_mov.tgt());
        schedule.mut_Moves().push(mov);
    }
    schedule
}
