use glib::subclass::prelude::*;

use crate::ChatMemberStatus;

mod imp {
    use super::*;

    use glib::prelude::*;
    use glib::Properties;
    use std::cell::Cell;
    use std::cell::RefCell;

    #[derive(Debug, Properties, Default)]
    #[properties(wrapper_type = super::Supergroup)]
    pub struct Supergroup {
        #[property(get)]
        pub(super) member_count: Cell<i32>,
        #[property(get)]
        pub(super) status: RefCell<Option<ChatMemberStatus>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Supergroup {
        const NAME: &'static str = "TdlSupergroup";
        type Type = super::Supergroup;
    }

    impl ObjectImpl for Supergroup {}
}

glib::wrapper! {
    pub struct Supergroup(ObjectSubclass<imp::Supergroup>);
}

impl Supergroup {
    pub(crate) fn new(td_supergroup: tdlib::types::Supergroup) -> Self {
        let obj: Self = glib::Object::new();
        let imp = obj.imp();

        imp.member_count.set(td_supergroup.member_count);
        imp.status
            .replace(Some(ChatMemberStatus(td_supergroup.status)));

        obj
    }

    pub(crate) fn update(&self, td_supergroup: tdlib::types::Supergroup) {
        self.set_member_count(td_supergroup.member_count);
        self.set_status(td_supergroup.status);
    }

    fn set_member_count(&self, member_count: i32) {
        if self.member_count() == member_count {
            return;
        }
        self.imp().member_count.set(member_count);
        self.notify_member_count();
    }

    fn set_status(&self, status: tdlib::enums::ChatMemberStatus) {
        if self.status().unwrap().0 == status {
            return;
        }
        self.imp().status.replace(Some(ChatMemberStatus(status)));
        self.notify_status();
    }
}
