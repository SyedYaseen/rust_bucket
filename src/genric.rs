pub fn genr () {
    // enumgenr();
    mixup();
}

#[derive(Debug)]
struct Point<T,U>  {
    x: T,
    y: U,
}

impl<T: Copy, U> Point<T,U> { // Copy is a trait, at this point I dont know how they work
    fn get_x(&self) -> T {
        self.x
    }
}

impl<T> Point<T,f64> {
    fn get_floaty(&self) -> f64 {
        self.y
    }
}

fn enumgenr() {
    let flotPt = Point { x: 5, y: 3.5 };
    flotPt.get_floaty();
    flotPt.get_x();
    let intPt = Point { x: 5, y: 3 };
    // intPt.get_floaty(); // wont work
    intPt.get_x();
}

// Supports T with copy trait, e.g. int, char, static str (stored on stack, fixed size).
// &self is a ref. Always copies. If "self" instead of "&self" was used, it means that struct "a" is moved
// into the method. Which mean a is no longer usable after this fn call. This is not because of the copy
// impl <T: Copy, U> Point<T, U> {
//     fn mix<V, W>(&self, pt: Point<V, W>) -> Point<T, W> {
//         Point {x: self.x, y: pt.y }
//     }
// }

// Support T with clone trait. e.g. String, heap allocated, can grow.
// Here the value needs to be explicitly cloned since its not allowed to be moved out of a reference.
// Since &self is a shared
impl <T: Clone, U> Point<T, U> {
    fn mix<V, W>(&self, pt: Point<V, W>) -> Point<T, W> {
        Point {x: self.x.clone(), y: pt.y }
    }
}


// Support T with Clone trait, e.g. String (heap-allocated, not Copy).
// This uses `self` instead of `&self`, meaning the method takes ownership of the whole struct.
// Because `self` is owned (not a reference), we are allowed to move `x` out of it directly.
// No need to clone here, even though T isn't Copy, because the method owns the value.
// Any method trying to access the struct after passing to method will cause a compiler error
// impl <T: Clone, U> Point<T, U> {
//     fn mix<V, W>(self, pt: Point<V, W>) -> Point<T, W> {
//         Point {x: self.x, y: pt.y }
//     }
// }

fn mixup() {
    // Copy trait
    // let a = Point {x: 1, y: 3.0};
    // let b = Point {x: "Hello", y: 'c'};

    // let c = a.mix(b);
    // println!("{:?}", c);
    // println!("{:?}", a);

    // Clone trait
    let a = Point {x: String::from("tasd"), y: 3.0};
    let b = Point {x: String::from("Hello"), y: 'c'};

    let c = a.mix(b);
    println!("{:?}", c);
    println!("{:?}", a);
}