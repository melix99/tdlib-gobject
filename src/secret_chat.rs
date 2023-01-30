mod imp {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Properties;
    use std::cell::Cell;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::SecretChat)]
    pub struct SecretChat {
        #[property(get)]
        pub(super) user_id: Cell<i64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SecretChat {
        const NAME: &'static str = "TdlSecretChat";
        type Type = super::SecretChat;
    }

    impl ObjectImpl for SecretChat {}
}

glib::wrapper! {
    pub struct SecretChat(ObjectSubclass<imp::SecretChat>);
}
