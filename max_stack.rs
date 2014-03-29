#[ crate_type = "bin" ];

mod max_stack {
    use std::vec::Vec;

    struct MaxStack {
        data: Vec<i64>,
        largests: Vec<i64>
    }

    pub fn new() -> MaxStack {
        return MaxStack { data: Vec::new(), largests: Vec::new() };
    }

    impl MaxStack {
        pub fn pop(&mut self) -> Option<i64> {
            let head = self.data.pop();
            let largest = self.get_largest();

            match (head, largest) {
                (None, _) => { None },
                (_, None) => { fail!("something went wrong") },
                (Some(popped_val), Some(largest_val)) => {
                    if popped_val == largest_val {
                        self.largests.pop();
                    }
                    head
                }
            }
        }

        pub fn push(&mut self, val: i64) {
            let largest = self.get_largest();

            match largest {
                None => {
                    self.largests.push(val); }
                Some(largest_val) => {
                    if val >= largest_val {
                        self.largests.push(val);
                    }
                }
            }
            self.data.push(val);
        }

        pub fn get_largest(&self) -> Option<i64> {
            self.largests.last().map(|&x| x)
        }
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_increasing() {
            let mut s = super::new();
            for i in range(0i64, 4) { s.push(i); }
            assert_eq!(s.get_largest(), Some(3)); s.pop();
            assert_eq!(s.get_largest(), Some(2)); s.pop();
            assert_eq!(s.get_largest(), Some(1)); s.pop();
            assert_eq!(s.get_largest(), Some(0)); s.pop();
            assert_eq!(s.get_largest(), None);
        }

        #[test]
        fn test_decreasing() {
            let mut s = super::new();
            for i in range(0i64, 4) { s.push(3 - i) }
            assert_eq!(s.get_largest(), Some(3));
        }

        #[test]
        fn test_all_negative() {
            let xs = [-34i64, -123i64, -7i64];
            let mut s = super::new();
            for x in xs.iter() { s.push(*x) };

            assert_eq!(s.get_largest(), Some(-7));
        }

        #[test]
        fn test_assorted() {
            use std::cmp::max;

            let xs = [1i64, 0, 34, 123, 1, 1232344, 234, 12323, 134322423423, -12334, -123];
            let max = xs.iter().fold(0, |a, &b| max(a, b));

            let mut s = super::new();
            for x in xs.iter() { s.push(*x) };
            assert_eq!(s.get_largest(), Some(max));
        }
    }
}

pub fn main() {
    let mut s = max_stack::new();
    s.push(1);
    s.push(59);
    s.push(32);
    s.push(17);
    println!("largest {}", s.get_largest());
    s.pop();
    println!("largest {}", s.get_largest());
    s.pop();
    println!("largest {}", s.get_largest());
    s.pop();
    println!("largest {}", s.get_largest());
    s.pop();
    println!("largest {}", s.get_largest());
}
