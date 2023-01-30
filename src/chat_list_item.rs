mod imp {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Properties;
    use std::cell::Cell;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::ChatListItem)]
    pub struct ChatListItem {
        #[property(get)]
        pub(super) is_pinned: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChatListItem {
        const NAME: &'static str = "TdlChatListItem";
        type Type = super::ChatListItem;
    }

    impl ObjectImpl for ChatListItem {}
}

glib::wrapper! {
    pub struct ChatListItem(ObjectSubclass<imp::ChatListItem>);
}
