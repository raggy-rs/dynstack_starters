Template for a model based solver written in rust.
================================

The dynamic stacking problem can be viewed as a series of offline block relocation problems (BRP).
This template demonstrates how to solve the problem in this way.
We translate both the buffer stacks as well as the production into stacks in the BRP.
The handover is not modelled instead we interpret removing a block from the BRP as putting it onto the handover stack.
In order to get a BRP from the simulation world state we need to assign priorities.
The code here simply uses the due date as the priority.
We can then solve the BRP which in this example is done by using a simple depth first search for sake of simplicity.
Finally we have to translate the solution to the model into a CraneSchedule for the simulation.


How to improve the from here:

* Improve translation from offline model to online problem
    - Prevent the production stack from filling up.
    - Improve priority assignment.
* Improve model
    - Introduce time
    - Introduce 
* Improve model solver.
    - Use heuristic for choosing promising moves.
    - Replace depth first search with branch and bound.

Prerequisits:
-------------
* [rust](https://rustup.rs/)
* [protoc](https://developers.google.com/protocol-buffers/docs/downloads) must be in the rust directory.

Building:
---------

Zeromq is automatically compiled during the build
Protobuf is automatically compiled during the build.

> cargo build

Running:
--------

> cargo run tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85