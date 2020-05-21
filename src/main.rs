mod cpu;
use cpu::*;

fn main() {
    
    let mut processor: Processor8080 = Default::default();

    processor.initialize();

    processor.enabled = true;

    while processor.enabled {

        emulate(&mut processor);
        
    }

}