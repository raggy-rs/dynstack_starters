What is this?
=============

This repository contains starting kits for the dynamic stacking competition @ GECCO 2020.
Starting kits are avialable in multiple languages:
* Rust
* Python
* C#

The starting kit show how to:
* Connect to the competition server using zeromq.
* Serialize and deserialize the messages using protobuf
* Plan a crane schedule for a given world state

The starting kits do not implement good solutions to the problem, but they are fully functioning.
Simply checkout this repository choose a language run the starting kit and you should be able run your first training simulation and observe it working in the visualization page. For instructions on how to run a specific solver check out the readme in the solvers folder.

How to solve the dynamic stacking problem.
==========================================

There are a number of possible approaches to solving dynamic optimization problems.
The simplest one is using a set of rules / heuristics to implement an online solver what we call the rule based approach.
The process is illustrated in the following diagram:

       +-------------+   +-------------+   +-------------+
    .->| World state |-->| Rules       |-->| Crane       |->.
    |  +-------------+   +-------------+   +-------------+  |
    '-------------------------------------------------------'

A little bit more involved is the model based approach where we translate the world state into a model we than optimize the model and translate the result into like in the following diagram:

                         +---------------+
       +-------------+   | Model         |   +-------------+
    .->| World state |-->| Optimize      |-->| Crane       |->.
    |  +-------------+   | Into Schedule |   +-------------+  |
    |                    +---------------+                    |
    '---------------------------------------------------------'

All starter kits show the basics of how to implement both of this approaches.


A basic model based approach.
-----------------------
The dynamic stacking problem can be viewed as a series of offline block relocation problems (BRP).
This template demonstrates how to solve the problem in this way.
We translate both the buffer stacks as well as the production into stacks in the BRP.
The handover is not modelled instead we interpret removing a block from the BRP as putting it onto the handover stack.
In order to get a BRP from the simulation world state we need to assign priorities.
The code here simply uses the due date as the priority.
We can then solve the BRP which in this example is done by using a simple depth first search for sake of simplicity.
Finally we have to translate the solution to the model into a CraneSchedule for the simulation. 


How to improve the from here:
-----------------------------
Of course there is plenty of room for improvements for both approaches.

* Devise better rules.
    - By thinking really hard.
    - By using machine learning.
* Improve translation between offline model and online problem
    - Prevent the production stack from filling up.
    - Improve priority assignment.
* Make the model more accurate
    - Introduce time
    - Introduce uncertainties.
* Improve model solver.
    - Use heuristic for choosing promising moves.
    - Replace depth first search with a better search strategy (e.g. branch and bound).
    - Enable incremental updates instead of treating consecutive world states as completely seperate.