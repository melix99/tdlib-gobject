mod basic_group;
mod chat;
mod chat_list;
mod chat_list_item;
mod client;
mod secret_chat;
mod supergroup;
mod user;

pub use basic_group::BasicGroup;
pub use chat::Chat;
pub use chat_list::ChatList;
pub use chat_list_item::ChatListItem;
pub use client::Client;
pub use secret_chat::SecretChat;
pub use supergroup::Supergroup;
pub use user::User;

#[derive(Clone, Debug, PartialEq, glib::Boxed)]
#[boxed_type(name = "TdlChatMemberStatus", nullable)]
pub struct ChatMemberStatus(tdlib::enums::ChatMemberStatus);
