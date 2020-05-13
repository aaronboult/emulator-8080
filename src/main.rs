mod cpu;
use cpu::*;

fn main() {
    
    let mut processor: Processor8080 = Default::default();

    processor.load("invaders/invaders".to_string());

    processor.enabled = true;

    loop {

        emulate(&mut processor);

    }

}