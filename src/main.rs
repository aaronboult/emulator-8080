mod machine;
mod io_handlers;
mod cpu;

use machine::*;

fn main() {

    let mut arcade_machine = Machine::new(1, false, false);

    arcade_machine.start();

}