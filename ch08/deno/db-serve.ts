// `https://deno.land/x/sqlite` is no longer maintained; `@db/sqlite` on JSR is
// its successor. Deno's built-in `Deno.serve` replaces the old std `serve()`.
import { Database } from "jsr:@db/sqlite@0.12";

// Create the Database. This requires write access!

const db = new Database("pl.db");
db.exec("DROP TABLE IF EXISTS languages");
db.exec(
  "CREATE TABLE languages (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
);

const names = ["C", "C++", "Rust", "TypeScript"];

// Populate the database

const insert = db.prepare("INSERT INTO languages (name) VALUES (?)");
for (const name of names) {
  insert.run(name);
}

// Close out the connection

db.close();

Deno.serve({ hostname: "0.0.0.0", port: 9000 }, () => {
  // Re-open the Database
  const db = new Database("pl.db");
  let bodyContent = "Programming Languages that work with WebAssembly:\n\n";

  for (const [name] of db.prepare("SELECT name FROM languages").values()) {
    bodyContent += name + "\n";
  }

  bodyContent += "\n";

  // Close the Database
  db.close();

  return new Response(bodyContent);
});
