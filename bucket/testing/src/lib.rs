#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_smol_rect() {
        let sm = Rectangle { width: 10, height: 20};
        let md = Rectangle { width: 20, height: 30 };
        let res = md.canHold(&sm);
        assert_eq!(true, res);
    }

    #[test]
    fn cant_hold_big_rect() {
        let sm = Rectangle { width: 10, height: 20};
        let md = Rectangle { width: 20, height: 30 };
        let res = sm.canHold(&md);
        assert_eq!(false, res);
    }
}
