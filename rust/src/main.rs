mod data_model;
use protobuf::Message;

fn main() {
    let mut args = std::env::args().skip(1);
    let world_socket_addr = args.next().expect("Expected address of world socket");
    let crane_socket_addr = args.next().expect("Expected address of crane socket");
    let sim_id = args.next().expect("Expected simulation id");

    let ctx = zmq::Context::new();
    let crane_socket = ctx.socket(zmq::DEALER).unwrap();
    crane_socket.set_identity(sim_id.as_bytes());
    crane_socket.connect(&crane_socket_addr).unwrap();
    println!("Connected crane");

    let world_socket = ctx.socket(zmq::DEALER).unwrap();
    world_socket.set_identity(sim_id.as_bytes());
    world_socket.connect(&world_socket_addr).unwrap();
    println!("Connected world");

    while let Ok(msg) = world_socket.recv_msg(0) {
        if msg.get_more() {
            println!("{:?}", msg);
            continue;
        }
        match protobuf::parse_from_bytes::<data_model::World>(&msg) {
            Ok(world) => {
                println!("{:?}", world);

                if let Some(new_schedule) = optimize_crane_schedule(&world) {
                    let message = new_schedule.write_to_bytes().expect("Could not serialize!");
                    crane_socket.send(message, 0);
                }
            }
            Err(_) => panic!("could not parse world"),
        }
    }
}

fn optimize_crane_schedule(world: &data_model::World) -> Option<data_model::CraneSchedule> {
    // TODO

    None
}
