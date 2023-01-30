mod imp {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Properties;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::Chat)]
    pub struct Chat {
        #[property(get)]
        pub(super) title: RefCell<String>,
        #[property(get)]
        pub(super) is_blocked: Cell<bool>,
        #[property(get)]
        pub(super) unread_count: Cell<i32>,
        #[property(get)]
        pub(super) unread_mention_count: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Chat {
        const NAME: &'static str = "TdlChat";
        type Type = super::Chat;
    }

    impl ObjectImpl for Chat {}
}

glib::wrapper! {
    pub struct Chat(ObjectSubclass<imp::Chat>);
}
