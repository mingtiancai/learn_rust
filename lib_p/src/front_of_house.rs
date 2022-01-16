pub mod front_if_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }

        pub fn seat_at_table() {
            println!("seat at table");
        }
    }

    mod serving {
        fn take_order() {
            println!("take order");
            // crate::front_if_house::hosting::add_to_waitlist();
        }

        fn serve_order() {
            println!("serve order");
        }

        fn take_payment() {
            println!("take payment");
        }
    }
}
