use glib::subclass::prelude::*;
use std::collections::hash_map::Entry;

use crate::BasicGroup;
use crate::Chat;
use crate::ChatList;
use crate::SecretChat;
use crate::Supergroup;
use crate::User;

mod imp {
    use super::*;

    use glib::prelude::*;
    use glib::Properties;
    use once_cell::unsync::OnceCell;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::Client)]
    pub struct Client {
        pub(super) chats: RefCell<HashMap<i64, Chat>>,
        pub(super) users: RefCell<HashMap<i64, User>>,
        pub(super) basic_groups: RefCell<HashMap<i64, BasicGroup>>,
        pub(super) supergroups: RefCell<HashMap<i64, Supergroup>>,
        pub(super) secret_chats: RefCell<HashMap<i32, SecretChat>>,
        pub(super) main_chat_list: OnceCell<ChatList>,
        pub(super) archive_chat_list: OnceCell<ChatList>,
        pub(super) filter_chat_lists: RefCell<HashMap<i32, ChatList>>,
        #[property(get)]
        pub(super) me: OnceCell<User>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Client {
        const NAME: &'static str = "TdlClient";
        type Type = super::Client;
    }

    impl ObjectImpl for Client {}
}

glib::wrapper! {
    pub struct Client(ObjectSubclass<imp::Client>);
}

impl Client {
    /// Returns the `Chat` of the specified id from the cache.
    /// Panics if the chat is not present.
    ///
    /// Note that TDLib guarantees that types are always returned before their ids,
    /// so if you have an id returned by TDLib, it should be expected that the
    /// relative object is already present in the cache.
    pub fn chat(&self, id: i64) -> Chat {
        self.imp()
            .chats
            .borrow()
            .get(&id)
            .expect("Failed to get expected Chat")
            .clone()
    }

    /// Returns the `User` of the specified id from the cache.
    /// Panics if the user is not present.
    ///
    /// Note that TDLib guarantees that types are always returned before their ids,
    /// so if you have an id returned by TDLib, it should be expected that the
    /// relative object is already present in the cache.
    pub fn user(&self, id: i64) -> User {
        self.imp()
            .users
            .borrow()
            .get(&id)
            .expect("Failed to get expected User")
            .clone()
    }

    /// Returns the `BasicGroup` of the specified id from the cache.
    /// Panics if the basic group is not present.
    ///
    /// Note that TDLib guarantees that types are always returned before their ids,
    /// so if you have an id returned by TDLib, it should be expected that the
    /// relative object is already present in the cache.
    pub fn basic_group(&self, id: i64) -> BasicGroup {
        self.imp()
            .basic_groups
            .borrow()
            .get(&id)
            .expect("Failed to get expected BasicGroup")
            .clone()
    }

    /// Returns the `Supergroup` of the specified id from the cache.
    /// Panics if the supergroup is not present.
    ///
    /// Note that TDLib guarantees that types are always returned before their ids,
    /// so if you have an id returned by TDLib, it should be expected that the
    /// relative object is already present in the cache.
    pub fn supergroup(&self, id: i64) -> Supergroup {
        self.imp()
            .supergroups
            .borrow()
            .get(&id)
            .expect("Failed to get expected Supergroup")
            .clone()
    }

    /// Returns the `SecretChat` of the specified id from the cache.
    /// Panics if the secret chat is not present.
    ///
    /// Note that TDLib guarantees that types are always returned before their ids,
    /// so if you have an id returned by TDLib, it should be expected that the
    /// relative object is already present in the cache.
    pub fn secret_chat(&self, id: i32) -> SecretChat {
        self.imp()
            .secret_chats
            .borrow()
            .get(&id)
            .expect("Failed to get expected SecretChat")
            .clone()
    }

    pub fn main_chat_list(&self) -> ChatList {
        self.imp().main_chat_list.get().unwrap().clone()
    }

    pub fn archive_chat_list(&self) -> ChatList {
        self.imp().archive_chat_list.get().unwrap().clone()
    }

    pub fn filter_chat_list(&self, id: i32) -> ChatList {
        self.imp()
            .filter_chat_lists
            .borrow()
            .get(&id)
            .expect("Failed to get expected filter ChatList")
            .clone()
    }

    pub fn update(&self, update: tdlib::enums::Update) {
        use tdlib::enums::Update;

        match update {
            Update::BasicGroup(data) => {
                self.update_basic_group(data.basic_group);
            }
            Update::Supergroup(data) => {
                self.update_supergroup(data.supergroup);
            }
            _ => {}
        }
    }

    fn update_basic_group(&self, basic_group: tdlib::types::BasicGroup) {
        let mut basic_groups = self.imp().basic_groups.borrow_mut();
        match basic_groups.entry(basic_group.id) {
            Entry::Occupied(entry) => entry.get().update(basic_group),
            Entry::Vacant(entry) => {
                entry.insert(BasicGroup::new(basic_group));
            }
        }
    }

    fn update_supergroup(&self, supergroup: tdlib::types::Supergroup) {
        let mut supergroups = self.imp().supergroups.borrow_mut();
        match supergroups.entry(supergroup.id) {
            Entry::Occupied(entry) => entry.get().update(supergroup),
            Entry::Vacant(entry) => {
                entry.insert(Supergroup::new(supergroup));
            }
        }
    }
}
