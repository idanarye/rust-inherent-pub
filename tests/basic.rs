#[test]
fn it_works() {
    mod my_module {
        use inherent_pub::inherent_pub;

        pub trait Foo {
            fn foo(self, a: i32, b: i32) -> i32;
        }

        pub struct Bar(pub i32, pub i32);

        #[inherent_pub]
        impl Foo for Bar {
            pub fn foo(self, a: i32, b: i32) -> i32 {
                a + b + self.0 + self.1
            }
        }
    }

    assert!(my_module::Bar(1, 2).foo(3, 4) == 1 + 2 + 3 + 4);
}
