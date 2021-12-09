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
	    var exec = instance.GetFunction(store, "exec");
	    exec.Invoke(store);
        }
    }
}
