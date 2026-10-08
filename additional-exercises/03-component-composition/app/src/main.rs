//! The *consumer* half of the composition. It calls `how-old` through an
//! interface, with no idea which component -- or which language -- implements
//! it. On its own this component is unrunnable: it has an unsatisfied import.
//! `wac plug` wires the two together into a single self-contained component.

wit_bindgen::generate!({
    world: "app",
    path: "wit",
});

use wasmtdg::calculator::ages;

fn main() {
    for (now, born) in [(2026, 2000), (2026, 1980), (2026, 2030)] {
        let age = ages::how_old(now, born);
        if age < 0 {
            println!("born in {born}? not yet!");
        } else {
            println!("born in {born}, in {now} you are {age}");
        }
    }
}
