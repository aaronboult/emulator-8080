/*
    Intel 8080 Data Book: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
    Original Repository: https://github.com/aaronboult/emulator-8080
*/

mod machine;
mod cpu;

fn main() {

    let mut arcade_machine = machine::Machine::new(0, false, false); // Params: Game ID, Log To File, Test

    arcade_machine.start();

}