use core::num;
use std::result;
mod varmut {

    pub fn varmut() {
        // ternary();
        // loop_type1();
        // while_loop();
        for_loop();
    }

    fn ternary() {
        let cond = true;
        let x = if cond { 5 } else { 6 };
        println!("{}", x);
    }

    fn loop_type1() {
        let mut counter = 0;
        let result = loop {
            if counter == 10 {
                break counter;
            }
            counter += 1;
        };

        println!("{}", result)
    }

    fn while_loop() {
        let mut num = 0;
        while num != 3 {
            num += 1;
            println!("{}", num);
        }
        println!("{}", "Loop complete");
    }

    fn for_loop() {
        let byte = [0; 8];
        for i in byte.iter() {
            println!("{}", i);
        }

        for i in 1..5 {
            println!("{}", i);
        }
    }
}
