Template for a rust based solver.
================================

Prerequisits:
-------------
[rust]{https://rustup.rs/}
[protoc](https://developers.google.com/protocol-buffers/docs/downloads) must be in the rust directory.

Building:
---------

Zeromq is automatically compiled during the build
Protobuf is automatically compiled during the build.

> cargo build

Running:
--------

> cargo run tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 G26NT63A