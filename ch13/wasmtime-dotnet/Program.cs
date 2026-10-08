using System;
using Wasmtime;

namespace wasmtime_dotnet
{
    class Program
    {
        static void Main(string[] args)
        {
            using var engine = new Engine();
            using var module = Module.FromTextFile(engine, "hello.wat");
            using var linker = new Linker(engine);
            using var store = new Store(engine);

	    linker.Define(
	       "hello",
	       "world",
	       Function.FromCallback(store,
	           () => Console.WriteLine("I like soccer and shishkabobs."))
	    );

            var instance = linker.Instantiate(store, module);

	    // Newer versions of wasmtime-dotnet capture the `Store` in the
	    // `Instance`, so exported functions are retrieved and invoked
	    // without passing it again. `GetAction` is the accessor for an
	    // export that takes and returns nothing.
	    var exec = instance.GetAction("exec")
	        ?? throw new InvalidOperationException("module does not export 'exec'");
	    exec();
        }
    }
}
