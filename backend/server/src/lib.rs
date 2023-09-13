pub mod api {
    pub mod network {
        pub mod routing;

        pub mod responses {
            pub mod get;
            pub mod post;
        }

        pub mod session;
    }

    pub mod users {
        pub mod auth {
            pub mod get;
            pub mod post;
        }

        pub mod user {
            pub mod get;
            pub mod post;
        }

        mod prelude;
    }

    pub mod crypto {
        pub mod passwords;
    }
}

pub mod args;
