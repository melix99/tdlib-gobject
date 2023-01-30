#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "BoxedChatMemberStatus")]
pub struct ChatMemberStatus(tdlib::enums::ChatMemberStatus);
