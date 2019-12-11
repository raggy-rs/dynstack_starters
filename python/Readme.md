Template for a python based solver.
================================

Prerequisits:
-------------
[python]{https://www.python.org/}
[pip]{https://pypi.org/} or [conda]{https://www.anaconda.com/distribution/}
[protobuf]{https://pypi.org/project/protobuf/}
[pyzmq]{https://pypi.org/project/pyzmq/}
[protoc](https://developers.google.com/protocol-buffers/docs/downloads)

Building:
---------

* install protobuf and pyzmq with your package manager of choice
* compile .proto file with:
> protoc.exe .\data_model.proto --python_out=python

Running:
--------

> python dynstack.py tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 G26NT63A