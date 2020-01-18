#[async_std::test]
async fn with_async_trait() {
    mod foo {
        use async_std::future::ready;
        use async_trait::async_trait;
        use inherent_pub::inherent_pub;

        #[async_trait]
        pub trait Bar {
            async fn bar(&self) -> usize;
        }

        pub struct Baz(pub usize);

        // async_trait before inherent_pub
        #[async_trait]
        #[inherent_pub]
        impl Bar for Baz {
            pub async fn bar(&self) -> usize {
                ready(self.0 * 2).await
            }
        }

        pub struct Qux(pub usize);

        // inherent_pub before async_trait
        #[async_trait]
        #[inherent_pub]
        impl Bar for Qux {
            pub async fn bar(&self) -> usize {
                ready(self.0 * 3).await
            }
        }
    }

    assert_eq!(foo::Bar::bar(&foo::Baz(1)).await, 2);
    assert_eq!(foo::Baz(2).bar().await, 4);

    assert_eq!(foo::Bar::bar(&foo::Qux(3)).await, 9);
    assert_eq!(foo::Qux(4).bar().await, 12);
}
