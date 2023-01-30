mod imp {
    use gio::subclass::prelude::*;
    use glib::prelude::*;
    use glib::Properties;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    use crate::ChatListItem;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::ChatList)]
    pub struct ChatList {
        pub(super) list: RefCell<BTreeMap<i64, ChatListItem>>,
        #[property(get)]
        pub(super) unread_chat_count: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChatList {
        const NAME: &'static str = "TdlChatList";
        type Type = super::ChatList;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for ChatList {}

    impl ListModelImpl for ChatList {
        fn item_type(&self) -> glib::Type {
            ChatListItem::static_type()
        }

        fn n_items(&self) -> u32 {
            self.list.borrow().len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.list
                .borrow()
                .iter()
                .nth(position as usize)
                .map(|(_, i)| i.upcast_ref())
                .cloned()
        }
    }
}

glib::wrapper! {
    pub struct ChatList(ObjectSubclass<imp::ChatList>)
        @implements gio::ListModel;
}
