extern crate inherent_pub;

mod foo {
    pub trait Foo {
        fn foo(self, x: i32, y: &str, z: (f32, u64));
    }
}

mod bar {
    use inherent_pub::inherent_pub;
    use foo::Foo;

    pub struct Bar;

    #[inherent_pub]
    impl Foo for Bar {
        pub fn foo(self, x: i32, y: &str, (z1, z2): (f32, u64)) {
            println!("Hello World {} {} {} {}", x, y, z1, z2);
        }
    }
}

fn main() {
    bar::Bar.foo(1, "2", (3.4, 5));
}
