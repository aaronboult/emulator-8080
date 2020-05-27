mod machine;
mod cpu;

use machine::*;

fn main() {

    let mut arcade_machine = Machine::new(0, false, false);

    arcade_machine.start();

}