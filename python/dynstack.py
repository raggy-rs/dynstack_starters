import data_model_pb2
import zmq


import sys

def optimize_crane_schedule(world):
    
    if len(world.Crane.Schedule.Moves) > 0:
        print("existing")
        return None
    schedule = data_model_pb2.CraneSchedule()
    if len(world.Production.BottomToTop) > 0:
        block = world.Production.BottomToTop[-1]
        print("production", block)
        for buf in world.Buffers:
            if buf.MaxHeight > len(buf.BottomToTop):
                print("free", buf)
                mov = schedule.Moves.add()
                mov.BlockId = block.Id
                mov.SourceId = world.Production.Id
                mov.TargetId = buf.Id
                return schedule

    return None

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
        if crane_schedule:
            crane_schedule.SequenceNr = seq_nr
            seq_nr += 1
            msg = crane_schedule.SerializeToString()
            crane_socket.send(msg)
            

