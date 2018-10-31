extern crate inherent_pub;

mod geometry {
    use inherent_pub::inherent_pub;

    pub trait Length {
        fn length(&self) -> f64;
    }

    pub struct Vector(pub f64, pub f64);

    #[inherent_pub]
    impl Length for Vector {
        pub fn length(&self) -> f64 {
            let Vector(x, y) = self;
            (x.powi(2) + y.powi(2)).sqrt()
        }
    }
}

fn main() {
    assert!(geometry::Vector(3.0, 4.0).length() == 5.0);
}
