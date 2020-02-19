use dynstack::{
    data_model::{CraneSchedule, World},
    heuristics::{any_handover_move, free_production_stack},
};
use protobuf::Message;

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

    while let Ok(msg) = world_socket.recv_msg(0) {
        if msg.get_more() {
            println!("{:?}", msg);
            continue;
        }
        let world = protobuf::parse_from_bytes::<World>(&msg)?;
        if let Some(new_schedule) = optimize_crane_schedule(&world) {
            let message = new_schedule.write_to_bytes()?;
            crane_socket.send(message, 0)?;
        }
    }
    Ok(())
}

fn optimize_crane_schedule(world: &World) -> Option<CraneSchedule> {
    if !world.get_Crane().get_Schedule().get_Moves().is_empty() {
        return None;
    }
    let mut schedule = CraneSchedule::new();
    schedule.set_SequenceNr(1);

    any_handover_move(world, &mut schedule);
    free_production_stack(world, &mut schedule);

    if schedule.get_Moves().is_empty() {
        None
    } else {
        println!("{:?}", schedule);
        return Some(schedule);
    }
}
