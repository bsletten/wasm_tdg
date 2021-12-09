## Chapter 13 : WebAssembly and .NET

As long as you have the [dotnet](https://dotnet.microsoft.com/en-us/)
toolchain installed, running the examples for this chapter are very
easy:

The `wasmtime-dotnet` directory has an example that uses the Wasmtime
library with C# and .NET:

```
> dotnet run
I like soccer and shishkabobs.
```

The `blazor-web` directory has an example that serves up the
WebAssembly-based Blazor distribution. It is executed the same way but
you will then point your browser at
[http://localhost:5000/](http://localhost:5000/) or
[https://localhost:5001/](https://localhost:5001/).

```
> dotnet run
Building...
info: Microsoft.Hosting.Lifetime[0]
      Now listening on: https://localhost:5001
info: Microsoft.Hosting.Lifetime[0]
      Now listening on: http://localhost:5000
info: Microsoft.Hosting.Lifetime[0]
      Application started. Press Ctrl+C to shut down.
info: Microsoft.Hosting.Lifetime[0]
      Hosting environment: Development
```
