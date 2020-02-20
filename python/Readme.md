Templates for a python based solvers.
================================

Prerequisits:
-------------
* [python](https://www.python.org/)
* [pip](https://pypi.org/) or [conda](https://www.anaconda.com/distribution/)
* [protobuf](https://pypi.org/project/protobuf/)
* [pyzmq](https://pypi.org/project/pyzmq/)
* [protoc](https://developers.google.com/protocol-buffers/docs/downloads)

Building:
---------

* install protobuf and pyzmq with your package manager of choice
* compile .proto file with:
> protoc.exe .\data_model.proto --python_out=python

Running:
--------
Find simulation update address, crane control address and simulation GUID on the competition website.

Run the rule based solver with for example: 
> python stacking.py tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85

Run the model based solver with for example: 
> python stacking.py tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85 --modelbased