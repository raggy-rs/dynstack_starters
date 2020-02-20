using System;
using DynStacking.DataModel;

namespace Dynstacking
{
    public class SchedulingRules
    {
        public static void MakeSchedule(World world, CraneSchedule schedule)
        {
            AnyHandoverMove(world, schedule);
            FreeProductionStack(world, schedule);
        }
        private static void AnyHandoverMove(World world, CraneSchedule schedule)
        {
            if (!world.Handover.Ready)
            {
                return;
            }
            foreach (var stack in world.Buffers)
            {
                var blocks = stack.BottomToTop.Count;
                if (blocks > 0)
                {
                    var top = stack.BottomToTop[blocks - 1];
                    if (top.Ready)
                    {
                        var move = new CraneMove();
                        move.BlockId = top.Id;
                        move.SourceId = stack.Id;
                        move.TargetId = world.Handover.Id;
                        schedule.Moves.Add(move);
                    }
                }
            }

        }
        private static void FreeProductionStack(World world, CraneSchedule schedule)
        {
            var blocks = world.Production.BottomToTop.Count;
            if (blocks > 0)
            {
                var top = world.Production.BottomToTop[blocks - 1];
                foreach (var stack in world.Buffers)
                {
                    if (stack.MaxHeight > stack.BottomToTop.Count)
                    {
                        var move = new CraneMove();
                        move.BlockId = top.Id;
                        move.SourceId = world.Production.Id;
                        move.TargetId = stack.Id;
                        schedule.Moves.Add(move);
                        return;
                    }
                }
            }
        }
    }
}