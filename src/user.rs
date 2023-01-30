mod imp {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Properties;
    use std::cell::RefCell;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::User)]
    pub struct User {
        #[property(get)]
        pub(super) first_name: RefCell<String>,
        #[property(get)]
        pub(super) last_name: RefCell<String>,
        #[property(get)]
        pub(super) phone_number: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for User {
        const NAME: &'static str = "TdlUser";
        type Type = super::User;
    }

    impl ObjectImpl for User {}
}

glib::wrapper! {
    pub struct User(ObjectSubclass<imp::User>);
}
