use glib::subclass::prelude::*;

use crate::ChatMemberStatus;

mod imp {
    use super::*;

    use glib::prelude::*;
    use glib::Properties;
    use std::cell::Cell;
    use std::cell::RefCell;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::BasicGroup)]
    pub struct BasicGroup {
        #[property(get)]
        pub(super) member_count: Cell<i32>,
        #[property(get)]
        pub(super) status: RefCell<Option<ChatMemberStatus>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BasicGroup {
        const NAME: &'static str = "TdlBasicGroup";
        type Type = super::BasicGroup;
    }

    impl ObjectImpl for BasicGroup {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, _id: usize, _value: &glib::Value, _pspec: &glib::ParamSpec) {
            Self::derived_set_property(self, _id, _value, _pspec)
        }
        fn property(&self, _id: usize, _pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(self, _id, _pspec)
        }
    }
}

glib::wrapper! {
    pub struct BasicGroup(ObjectSubclass<imp::BasicGroup>);
}

impl BasicGroup {
    pub fn new(td_basic_group: tdlib::types::BasicGroup) -> Self {
        let obj: Self = glib::Object::new();
        let imp = obj.imp();

        imp.member_count.set(td_basic_group.member_count);
        imp.status
            .replace(Some(ChatMemberStatus(td_basic_group.status)));

        obj
    }

    pub fn update(&self, td_basic_group: tdlib::types::BasicGroup) {
        self.set_member_count(td_basic_group.member_count);
        self.set_status(Some(ChatMemberStatus(td_basic_group.status)));
    }

    fn set_member_count(&self, member_count: i32) {
        if self.member_count() == member_count {
            return;
        }
        self.imp().member_count.set(member_count);
        self.notify_member_count();
    }

    fn set_status(&self, status: Option<ChatMemberStatus>) {
        if self.status() == status {
            return;
        }
        self.imp().status.replace(status);
        self.notify_status();
    }
}
