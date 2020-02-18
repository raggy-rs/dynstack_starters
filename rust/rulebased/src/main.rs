mod data_model;
use data_model::{CraneSchedule, World};
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
        let world = protobuf::parse_from_bytes::<data_model::World>(&msg)?;
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

fn any_handover_move(world: &World, schedule: &mut CraneSchedule) {
    if !world.get_Handover().get_Ready() {
        return;
    }
    for stack in world.get_Buffers() {
        if let Some(top) = stack.get_BottomToTop().last() {
            if top.get_Ready() {
                let mut mov = data_model::CraneMove::new();
                mov.set_BlockId(top.get_Id());
                mov.set_SourceId(stack.get_Id());
                mov.set_TargetId(world.get_Handover().get_Id());
                schedule.mut_Moves().push(mov);
                return;
            }
        }
    }
}

fn free_production_stack(world: &World, schedule: &mut CraneSchedule) {
    if let Some(block) = world.get_Production().get_BottomToTop().last() {
        if let Some(free) = world
            .get_Buffers()
            .iter()
            .find(|b| (b.get_MaxHeight() as usize) > b.get_BottomToTop().len())
        {
            let mut mov = data_model::CraneMove::new();
            mov.set_BlockId(block.get_Id());
            mov.set_SourceId(world.get_Production().get_Id());
            mov.set_TargetId(free.get_Id());
            schedule.mut_Moves().push(mov);
        }
    }
}
