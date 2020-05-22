mod cpu;
use cpu::*;

fn main() {
    
    let mut processor: Processor8080 = Default::default();

    processor.initialize();

    processor.enabled = true;

    // for _ in 0..20{

    //     emulate(&mut processor);

    // }

    while processor.enabled {

        emulate(&mut processor);
        
    }

}