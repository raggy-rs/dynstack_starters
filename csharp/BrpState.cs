using System;
using System.Linq;
using System.Collections.Generic;
using DynStacking.DataModel;

namespace Dynstacking
{
    public class ModelBasedOptimizer
    {
        World world;
        public ModelBasedOptimizer(World world)
        {
            this.world = world;
        }
        public void CalculateSchedule(CraneSchedule schedule)
        {
            var priorities = PrioritizeByDueDate();
            var initalState = new BrpState(world, priorities);
            var solution = DepthFirstSearch(initalState);
            FillScheduleFromSolution(solution, schedule);
        }

        public Dictionary<int, int> PrioritizeByDueDate()
        {
            return world.Production.BottomToTop
                .Concat(world.Buffers
                    .SelectMany(stack => stack.BottomToTop))
                .OrderBy(block => block.Due.MilliSeconds)
                .Select((block, Prio) => new { block.Id, Prio })
                .ToDictionary(x => x.Id, x => x.Prio);
        }

        public List<CraneMove> DepthFirstSearch(BrpState initial)
        {
            var budget = 10000;
            List<CraneMove> best = null;
            var stack = new Stack<BrpState>();
            stack.Push(initial);

            while (stack.Count > 0 && budget > 0)
            {
                budget -= 1;
                var state = stack.Pop();
                if (state.IsSolved)
                {
                    if (best == null || best.Count > state.Moves.Count)
                    {
                        best = state.Moves;
                    }
                }
                else
                {
                    foreach (var move in state.ForcedMoves())
                    {
                        stack.Push(state.Apply(move));
                    }
                }
            }

            return best;
        }
        public void FillScheduleFromSolution(List<CraneMove> solution, CraneSchedule schedule)
        {
            var handover = world.Handover;
            schedule.Moves.AddRange(solution
                .Take(3)
                .TakeWhile(move => handover.Ready || move.TargetId != handover.Id)
            );
        }
    }
    public class Stack
    {
        public int Id { get; }
        public int MaxHeight { get; }
        public Stack<Block> Blocks { get; }

        public Stack(DynStacking.DataModel.Stack stack, Dictionary<int, int> priorities)
        {
            Id = stack.Id;
            MaxHeight = stack.MaxHeight;
            Blocks = new Stack<Block>(
                stack.BottomToTop.Select(b => new Block(b.Id, priorities[b.Id]))
            );

        }
        public Stack(Stack source)
        {
            Id = source.Id;
            MaxHeight = source.MaxHeight;
            Blocks = new Stack<Block>(source.Blocks.Reverse());
        }

        public Block Top => Blocks.Peek();
        public Block MostUrgent => Blocks.MinBy(block => block.Prio);

    }
    public class Block
    {
        public Block(int id, int prio)
        {
            Id = id;
            Prio = prio;
        }
        public int Id { get; }
        public int Prio { get; }
    }

    public class BrpState
    {
        public List<CraneMove> Moves { get; }

        private Stack[] Stacks { get; }

        private int HandoverId { get; }
        private int ProductionId { get; }

        public BrpState(World world, Dictionary<int, int> priorities)
        {
            Moves = new List<CraneMove>();
            HandoverId = world.Handover.Id;
            ProductionId = world.Production.Id;
            Stacks = new[] { world.Production }
                .Concat(world.Buffers)
                .Select(s => new Stack(s, priorities))
                .ToArray();

        }
        public BrpState(BrpState source)
        {
            HandoverId = source.HandoverId;
            ProductionId = source.ProductionId;
            Moves = source.Moves.ToList();
            Stacks = source.Stacks.Select(s => new Stack(s)).ToArray();
        }

        public bool IsSolved => !NotEmptyStacks.Any();
        IEnumerable<Stack> NotFullStacks => Stacks.Where(x => x.Blocks.Count < x.MaxHeight);
        IEnumerable<Stack> NotEmptyStacks => Stacks.Where(x => x.Blocks.Count > 0);
        public List<CraneMove> ForcedMoves()
        {
            var possible = new List<CraneMove>();
            if (!NotEmptyStacks.Any())
            {
                return possible;
            }
            var src = NotEmptyStacks.MinBy(s => s.MostUrgent.Prio);
            var urgent = src.MostUrgent;
            var top = src.Top;
            if (urgent.Id == top.Id)
            {
                possible.Add(new CraneMove
                {
                    SourceId = src.Id,
                    TargetId = HandoverId,
                    BlockId = top.Id,
                });
            }
            else
            {
                foreach (var tgt in NotFullStacks)
                {
                    if (src.Id == tgt.Id || tgt.Id == ProductionId) continue;
                    possible.Add(new CraneMove
                    {
                        SourceId = src.Id,
                        TargetId = tgt.Id,
                        BlockId = top.Id,
                    });

                }
            }
            return possible;
        }

        public BrpState Apply(CraneMove move)
        {
            var result = new BrpState(this);
            var block = result.Stacks[move.SourceId].Blocks.Pop();
            if (move.TargetId != HandoverId)
            {
                result.Stacks[move.TargetId].Blocks.Push(block);
            }
            result.Moves.Add(move);
            return result;
        }
    }

    static class MinByExtension
    {
        public static T MinBy<T>(this IEnumerable<T> enumerable, Func<T, int> getKey) where T : class
        {
            int minKey = int.MaxValue;
            T minVal = null;
            foreach (var val in enumerable)
            {
                var curr = getKey(val);
                if (curr <= minKey)
                {
                    minKey = curr;
                    minVal = val;
                }
            }
            return minVal;
        }
    }
}