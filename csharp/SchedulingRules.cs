using System;
using DynStacking.DataModel;

namespace Dynstacking
{
    public class SchedulingRules
    {
        /// Use simple heuristics to come up with a crane schedule.
        public static void MakeSchedule(World world, CraneSchedule schedule)
        {
            AnyHandoverMove(world, schedule);
            ClearProductionStack(world, schedule);
        }

        /// If any block on top of a stack can be moved to the handover schedule this move.
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

        /// If the top block of the production stack can be put on a buffer schedule this move.
        private static void ClearProductionStack(World world, CraneSchedule schedule)
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