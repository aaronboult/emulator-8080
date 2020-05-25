mod machine;
mod cpu;

use machine::*;

fn main() {

    let mut arcade_machine = Machine::new(1, false);

    arcade_machine.start();

}