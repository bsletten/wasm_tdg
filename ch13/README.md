## Chapter 13 : WebAssembly and .NET

As long as you have the [dotnet](https://dotnet.microsoft.com/en-us/)
toolchain installed, running the examples for this chapter are very
easy.

Both projects targeted `net5.0`, which reached end of life in May 2022.
They now target `net10.0`, the current LTS. Along with that:

* `wasmtime` for .NET went from `0.28.0-preview1` to `48.0.2`. Exported
  functions are now retrieved from the `Instance` without passing the
  `Store` again, so `instance.GetFunction(store, "exec")` followed by
  `exec.Invoke(store)` becomes `instance.GetAction("exec")` and a plain
  call.
* Blazor went to `10.0.12`, and the `System.Net.Http.Json` package
  reference was dropped -- it is part of the shared framework now.
* `<Router PreferExactMatches="@true">` was removed in .NET 6 (exact
  matching is always on), so the attribute is gone from `App.razor`.
* `launchSettings.json` had `"dotnetRunMessages": "true"` as a string,
  which newer SDKs refuse to parse; it is a real boolean now.

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
