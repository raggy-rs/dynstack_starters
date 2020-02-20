Templates for a C# based solvers.
================================

Prerequisits:
-------------
* [dotnet core](https://dotnet.microsoft.com/download)

Building:
---------

* install Google.Protobuf and NetMQ nuget Packages with:
> dotnet restore
* compile .proto file with:
> protoc.exe .\data_model.proto --csharp_out=csharp
* build with:
> dotnet build

Running:
--------
Find simulation update address, crane control address and simulation GUID on the competition website.

Run the rule based solver with for example: 
> dotnet run -- tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85

Run the model based solver with for example: 
> dotnet run -- tcp://1.2.3.4:8080 tcp://1.2.3.4:8081 fbc6b6ab-9786-4068-986d-b0f5da49fa85 --modelbased