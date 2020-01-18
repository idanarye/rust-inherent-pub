#[test]
fn tuple_destructure() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self, a: (i32, i32)) -> i32;
        }

        pub struct Bar;

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(self, (a, b): (i32, i32)) -> i32 {
                a + b
            }
        }
    }

    assert!(my_module::Bar.foo((40, 2)) == 42);
}

#[test]
fn tuple_struct_destructure() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub struct Pair(pub i32, pub i32);

        pub trait Foo {
            fn foo(self, a: Pair) -> i32;
        }

        pub struct Bar;

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(self, Pair(a, b): Pair) -> i32 {
                a + b
            }
        }
    }

    assert!(my_module::Bar.foo(my_module::Pair(40, 2)) == 42);
}

#[test]
fn struct_destructure() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub struct Pair {
            pub a: i32,
            pub b: i32,
        }

        pub trait Foo {
            fn foo(self, a: Pair) -> i32;
        }

        pub struct Bar;

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(self, Pair { a, b: c }: Pair) -> i32 {
                a + c
            }
        }
    }

    assert!(my_module::Bar.foo(my_module::Pair{ a: 40, b: 2}) == 42);
}
