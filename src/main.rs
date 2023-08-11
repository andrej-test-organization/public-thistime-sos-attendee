mod tests;

mod secret {
    trait ComputeFns {
        fn addition(&self) -> f64;
        fn subtraction(&self) -> f64;
        fn multiplication(&self) -> f64;
        fn division(&self) -> f64;
        fn modulo(&self) -> f64;
    }
    pub struct Compute {
        x: f64,
        y: f64,
    }
    #[allow(dead_code)]
    impl Compute {
        pub fn new(arg1: &f64, arg2: &f64) -> Self {
            Self { x: *arg1, y: *arg2 }
        }
        pub fn do_adition(&self) -> f64 {
            // TODO
            0.0
        }
        pub fn do_subtraction(&self) -> f64 {
            // TODO
            0.0
        }
        pub fn do_multiplication(&self) -> f64 {
            // TODO
            0.0
        }
        pub fn do_division(&self) -> f64 {
            // TODO
            0.0
        }
        pub fn do_modulo(&self) -> f64 {
            // TODO
            0.0
        }
        pub fn change_x(&mut self, x: &f64) {
            self.x = *x;
        }
        pub fn change_y(&mut self, y: &f64) {
            self.y = *y;
        }
    }
    impl ComputeFns for Compute {
        fn addition(&self) -> f64 {
            // TODO
            0.0
        }
        fn subtraction(&self) -> f64 {
            // TODO
            0.0
        }
        fn multiplication(&self) -> f64 {
            // TODO
            0.0
        }
        fn division(&self) -> f64 {
            // TODO
            0.0
        }
        fn modulo(&self) -> f64 {
            // TODO
            0.0
        }
    }
}

fn main() {}
