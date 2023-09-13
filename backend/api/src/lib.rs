pub mod network {
    pub mod routing;
    pub mod responses {
        pub mod get;
        pub mod post;
    }
}

pub mod users {
    pub mod auth {
        pub mod get;
        pub mod post;
    }
}

pub mod crypto {
    pub mod passwords;
}
