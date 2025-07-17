use std::{collections::HashMap, thread, time::Duration};

struct CalcCache<T> {
    calculation: T,
    values: HashMap<u32, u32>
}

impl<T> CalcCache<T> 
    where T: Fn(u32) -> u32
{
    fn new(calc: T) -> CalcCache<T> {
        CalcCache {
            calculation: calc,
            values: HashMap::new()
        }
    }

    fn value(&mut self, input: u32) -> u32 {
        let out = self.values.get(&input);
        match out {
            Some(val) => {
                println!("Getting value for {input} from cache");
                *val
            } 
            None => {
                let v = (self.calculation)(input);
                self.values.insert(input, v);
                v
            }
        }
    }
}

fn generate_workout<T: Fn(u32) -> u32>(simulated_intensity: u32, intensity: u32, expensive_closure: &mut CalcCache<T>) {
    println!("Current intensity {}", simulated_intensity);


    if intensity < 5 {
        println!("Do 10 pull ups {}", expensive_closure.value(intensity));
        println!("Do 10 sit ups {}", expensive_closure.value(intensity));
    } else {
        println!("Do 200 pull ups {}", expensive_closure.value(intensity));
    }
}

fn main() {
    // let x =  5;
    // let c = |d: i32| println!("{}", x);
    // c(25);

    let mut expensive_closure = CalcCache::new(|intensity: u32| -> u32 {
        println!("Starting an expensive calc");
        thread::sleep(Duration::from_secs(3));
        println!("Completed expensive calc");
        intensity
    });


    generate_workout(5, 5, &mut expensive_closure);
    generate_workout(3, 3, &mut expensive_closure);
    generate_workout(10, 10, &mut expensive_closure);
    generate_workout(5, 5, &mut expensive_closure);
}
