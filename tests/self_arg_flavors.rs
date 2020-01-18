#[test]
fn no_self() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo() -> i32;
        }

        pub struct Bar;

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo() -> i32 {
                42
            }
        }
    }

    assert!(my_module::Bar::foo() == 42);
}

#[test]
fn move_self() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self) -> i32;
        }

        pub struct Bar(pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(self) -> i32 {
                self.0
            }
        }
    }

    assert!(my_module::Bar(42).foo() == 42);
}

#[test]
fn move_self_mut() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self) -> Self;
        }

        pub struct Bar(pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(mut self) -> Self {
                self.0 += 1;
                self
            }
        }
    }

    let bar = my_module::Bar(41);
    let bar = bar.foo();
    assert!(bar.0 == 42);
}

#[test]
fn borrow_self() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(&self) -> i32;
        }

        pub struct Bar(pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(&self) -> i32 {
                self.0
            }
        }
    }

    assert!(my_module::Bar(42).foo() == 42);
}

#[test]
fn borrow_self_mut() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(&mut self);
        }

        pub struct Bar(pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(&mut self) {
                self.0 += 1;
            }
        }
    }

    let mut bar = my_module::Bar(41);
    bar.foo();
    assert!(bar.0 == 42);
}

#[test]
fn box_self() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self: Box<Self>) -> i32;
        }

        pub struct Bar(pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(self: Box<Self>) -> i32 {
                self.0
            }
        }
    }

    assert!(Box::new(my_module::Bar(42)).foo() == 42);
}

#[test]
fn box_self_mut() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self: Box<Self>) -> i32;
        }

        pub struct Bar(pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(mut self: Box<Self>) -> i32 {
                self.0 += 1;
                self.0
            }
        }
    }

    assert!(Box::new(my_module::Bar(42)).foo() == 43);
}
