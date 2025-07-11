#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods of struct
    fn area(&self) {
        self.width * self.height;
    }

    fn canHold(&self, rect: &Rectangle) -> bool {
        if self.width < rect.width || self.height < rect.height {
            false
        } else {
            true
        }
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        // Associative func
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn strcuts() {
    let sqr: Rectangle = Rectangle::square(5);
    println!("{:#?}", sqr);

    let r1: Rectangle = Rectangle {
        width: 20,
        height: 30,
    };
    let r2: Rectangle = Rectangle {
        width: 100,
        height: 70,
    };

    let rec: Rectangle = Rectangle {
        width: 25,
        height: 45,
    };

    println!("rec can hold r1: {}", rec.canHold(&r1));
    println!("rec can hold r2: {}", rec.canHold(&r2));
}
