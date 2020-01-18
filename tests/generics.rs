#[test]
fn generic_struct() {
    mod my_module {
        use std::fmt::Display;
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self) -> String;
        }

        pub struct Bar<T: Display>(pub T);

        #[inherent_pub]
        impl<T: Display> Foo for Bar<T> {
            pub fn foo(self) -> String {
                format!("{}", self.0)
            }
        }
    }

    assert!(my_module::Bar(42).foo() == "42");
    assert!(my_module::Bar(4.2).foo() == "4.2");
}

#[test]
fn generic_trait_and_struct() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo<T> {
            fn foo(self) -> T;
        }

        pub struct Bar<T>(pub T);

        #[inherent_pub]
        impl<T> Foo<T> for Bar<T> {
            pub fn foo(self) -> T {
                self.0
            }
        }
    }

    assert!(my_module::Bar(42).foo() == 42);
}

// #[test]
// fn generic_trait() {
    // mod my_module {
        // use inherent_pub::inherent_pub;

        // pub trait Foo<T: Default> {
            // fn foo(self) -> T;
        // }

        // pub struct Bar;

        // #[inherent_pub]
        // impl<T: Default> Foo<T> for Bar {
            // pub fn foo(self) -> T {
                // T::default()
            // }
        // }
    // }

    // assert!(my_module::Bar().foo() == 0);
// }

#[test]
fn generic_method() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo<T>(self, value: T) -> T;
        }

        pub struct Bar;

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo<T>(self, value: T) -> T {
                value
            }
        }
    }

    assert!(my_module::Bar.foo(42) == 42);
}
