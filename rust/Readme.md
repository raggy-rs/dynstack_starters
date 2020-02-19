Templates for rust based solvers.
================================

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

> cargo run --release --bin rulebased tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85

or 

> cargo run --release --bin modelbased tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85