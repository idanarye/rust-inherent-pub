extern crate inherent_pub;

mod foo {
    pub trait Foo {
        fn foo(self);
    }
}

mod bar {
    use inherent_pub::inherent_pub;
    use foo::Foo;

    pub struct Bar;

    #[inherent_pub]
    impl Foo for Bar {
        pub fn foo(self) {
            println!("Hello World");
        }
    }
}

fn main() {
    use foo::Foo;

    bar::Bar.foo();
}
