import { DB } from "https://deno.land/x/sqlite/mod.ts";
import { serve } from "https://deno.land/std@0.93.0/http/server.ts";

// Create the Database. This requires write access!

const db = new DB("pl.db");
db.query(
    "DROP TABLE IF EXISTS languages",
);

db.query(
   "CREATE TABLE languages (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
);

const names = ["C", "C++", "Rust", "TypeScript"]

// Populate the database

for (const name of names) {
  db.query("INSERT INTO languages (name) VALUES (?)", [name]);
}

// Close out the connection

db.close();

const server = serve({ hostname: "0.0.0.0", port: 9000 });
console.log(`HTTP webserver running.  Access it at:  http://localhost:9000/`);

for await (const request of server) {
    // Re-open the Database
    
    const db = new DB("pl.db");
    let bodyContent = "Programming Languages that work with WebAssembly:\n\n";
    
    for(const [name] of db.query("SELECT name FROM languages")) {
	bodyContent += name + "\n";
    }

    bodyContent += "\n";
    request.respond({ status: 200, body: bodyContent });

    // Close the Database
    db.close();
}
