from data_model_pb2 import World, CraneSchedule, CraneMove
import zmq
from brp import optimize_crane_schedule

import sys

def heuristic_crane_schedule(world):
    if len(world.Crane.Schedule.Moves) > 0:
        return None
    schedule = CraneSchedule()
    if len(world.Production.BottomToTop) > 0:
        block = world.Production.BottomToTop[-1]
        for buf in world.Buffers:
            if buf.MaxHeight > len(buf.BottomToTop):
                mov = schedule.Moves.add()
                mov.BlockId = block.Id
                mov.SourceId = world.Production.Id
                mov.TargetId = buf.Id
                return schedule

    return None

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("""USAGE:
        python dynstack WORLD CRANE ID""")
        exit(1)
    if len(sys.argv) == 4:
        print("rule based stacking")
        [_, world_addr, crane_addr, id] = sys.argv
        heuristic = False
    else:
        [_, world_addr, crane_addr, id, _] = sys.argv
        print("model based stacking")
        heuristic = True
    
    context = zmq.Context()
    world_socket = context.socket(zmq.DEALER)
    world_socket.setsockopt_string(zmq.IDENTITY, id)
    world_socket.connect(world_addr)
    print("Connected world")
    crane_socket = context.socket(zmq.DEALER)
    crane_socket.setsockopt_string(zmq.IDENTITY, id)
    crane_socket.connect(crane_addr)
    print("Connected crane")

    while True:
        msg = world_socket.recv()
        world = World()
        world.ParseFromString(msg)

        if heuristic:
            crane_schedule = heuristic_crane_schedule(world)
        else:
            crane_schedule = optimize_crane_schedule(world)
        if crane_schedule:
            crane_schedule.SequenceNr = world.Crane.Schedule.SequenceNr + 1
            print("send ", crane_schedule)
            msg = crane_schedule.SerializeToString()
            crane_socket.send(msg)
            

