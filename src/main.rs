mod cpu;
use cpu::*;

fn main() {
    
    let mut processor: Processor8080 = Default::default();

    processor.load("");

    processor.enabled = true;

    loop {

        emulate(&mut processor);

    }

}