using System;
using System.Text;
using NetMQ;
using NetMQ.Sockets;
using DynStacking.DataModel;
using Google.Protobuf;

namespace Dynstacking
{
    enum OptimizerType
    {
        RuleBased,
        ModelBased
    }
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine(args.Length);
            if (args.Length < 3)
            {
                Console.WriteLine("Requeries 3 arguments: WORLD_SOCKET CRANE_SOCKET SIM_ID");
                return;
            }
            var worldSocketAddr = args[0];
            var craneSocketAddr = args[1];
            var identity = new UTF8Encoding().GetBytes(args[2]);
            OptimizerType optType;
            if (args.Length > 3)
            {
                optType = OptimizerType.ModelBased;
            }
            else
            {
                optType = OptimizerType.RuleBased;
            }
            Console.WriteLine(optType);

            using (var worldSocket = new DealerSocket()) // bind
            using (var craneSocket = new DealerSocket())
            {
                worldSocket.Options.Identity = identity;
                craneSocket.Options.Identity = identity;
                worldSocket.Connect(worldSocketAddr);
                Console.WriteLine("Connected world");
                craneSocket.Connect(craneSocketAddr);
                Console.WriteLine("Connected crane");


                while (true)
                {
                    var world = World.Parser.ParseFrom(worldSocket.ReceiveFrameBytes());

                    var schedule = OptimizeCraneShedule(world, optType);
                    if (schedule != null)
                    {
                        schedule.SequenceNr = world.Crane.Schedule.SequenceNr;
                        Console.WriteLine(schedule);
                        var data = schedule.ToByteArray();
                        Console.WriteLine(data.Length);
                        var msg = new Msg();
                        msg.InitGC(data, data.Length);
                        craneSocket.Send(ref msg, false);
                    }

                }

            }

        }
        private static CraneSchedule OptimizeCraneShedule(World world, OptimizerType opt)
        {
            if (world.Crane.Schedule.Moves.Count > 0)
            {
                return null;
            }
            var schedule = new CraneSchedule();
            schedule.SequenceNr = 1;
            switch (opt)
            {
                case OptimizerType.RuleBased: SchedulingRules.MakeSchedule(world, schedule); break;
                case OptimizerType.ModelBased: new ModelBasedOptimizer(world).CalculateSchedule(schedule); break;
            }
            if (schedule.Moves.Count > 0)
            {
                return schedule;
            }
            else
            {
                return null;
            }
        }


    }
}
