from data_model_pb2 import World, CraneSchedule, CraneMove
import zmq
from brp import prioritize_by_due_date, BrpState, depth_first_search

import sys

def optimize_crane_schedule(world):
    if len(world.Crane.Schedule.Moves) > 0:
        return None
    priorities = prioritize_by_due_date(world)
    initial = BrpState(world, priorities)
    moves = depth_first_search(initial)
    return create_schedule_from_solution(world, moves)

def create_schedule_from_solution(world, moves):
    schedule = CraneSchedule()
    handover = world.Handover
    is_ready = handover.Ready
    for opt_mov in moves[:3]:
        if not is_ready and opt_mov.tgt == handover.Id:
            break
        move = CraneMove()
        move.BlockId = opt_mov.block
        move.SourceId = opt_mov.src
        move.TargetId = opt_mov.tgt
        schedule.Moves.append(move)
    if any(schedule.Moves):
        return schedule
    else:
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
    print("Connected world")
    crane_socket = context.socket(zmq.DEALER)
    crane_socket.setsockopt_string(zmq.IDENTITY, id)
    crane_socket.connect(crane_addr)
    print("Connected crane")

    seq_nr = 0
    while True:
        msg = world_socket.recv()
        world = World()
        world.ParseFromString(msg)
        crane_schedule = optimize_crane_schedule(world)
        if crane_schedule:
            crane_schedule.SequenceNr = seq_nr
            seq_nr += 1
            print("send ", crane_schedule)
            msg = crane_schedule.SerializeToString()
            crane_socket.send(msg)
            

