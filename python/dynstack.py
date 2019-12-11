import data_model_pb2
import zmq


import sys

def optimize_crane_schedule(world):
    crane_schedule = data_model_pb2.CraneSchedule()
    print(world)
    return crane_schedule

if __name__ == "__main__":
    if len(sys.argv)!= 4:
        print("""USAGE:
        python dynstack WORLD CRANE ID""")
        exit(1)
    [_, world_addr, crane_addr, id] = sys.argv
    context = zmq.Context()
    world_socket = context.socket(zmq.DEALER)
    world_socket.setsockopt_string(zmq.IDENTITY, id)
    world_socket.connect(world_addr)
    crane_socket = context.socket(zmq.DEALER)
    crane_socket.setsockopt_string(zmq.IDENTITY, id)
    crane_socket.connect(crane_addr)

    seq_nr = 0
    while True:
        msg = world_socket.recv()
        
        world = data_model_pb2.World()
        world.ParseFromString(msg)
        
        crane_schedule = optimize_crane_schedule(world)
        if crane_schedule.Moves:
            crane_schedule.SequenceNr = seq_nr
            seq_nr += 1
            msg = crane_schedule.SerializeToString()
            crane_socket.send(msg)
            

