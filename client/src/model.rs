//! Data models for the mock JSON shipped in `client/src/data/im/`.
//!
//! Both files are embedded with `include_str!`, so the binary always finds them
//! regardless of the working directory. Every field except `id`/`name`/`group`
//! is optional: the JSON can grow (`time`, `details`, …) without breaking the
//! app, and missing values degrade to an empty string instead of an error.

use serde::Deserialize;

use crate::media::Attachment;

const CHAT_LIST_JSON: &str = include_str!("data/im/chat_list.json");
const CONTACTS_LIST_JSON: &str = include_str!("data/im/contacts_list.json");

/// One conversation row from `chat_list.json`.
#[derive(Clone, Default, Deserialize)]
pub struct Chat {
    pub id: usize,
    pub name: String,
    /// JSON key `lastMessage`; shown as the grey preview line.
    #[serde(rename = "lastMessage", default)]
    pub last: String,
    #[serde(default)]
    pub unread: u32,
    /// Remote avatar URL (`img()` loads it over HTTP).
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub messages: Vec<Message>,
    /// Not present in the current mock file; rendered when supplied.
    #[serde(default)]
    pub time: String,
    /// Not present in the current mock file; rendered when supplied.
    #[serde(default)]
    pub details: String,
}

/// One chat bubble. `mine` marks messages written by the local user.
#[derive(Clone, Default, Deserialize)]
pub struct Message {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub mine: bool,
    /// 「照片 / 视频 / 文件」附件。
    ///
    /// `skip`：mock JSON 里暂时没有这个字段，`Message` 的附件全部由界面
    /// （文件选择器 → `Attachment::from_path`）在内存里创建；等以后消息由 SDK
    /// 下发时，把 `skip` 换成普通的 `default` 并给 `Attachment` 加上 `Deserialize` 即可。
    #[serde(skip)]
    pub attachment: Option<Attachment>,
}

impl Message {
    /// 只有文字的气泡。
    pub fn text(text: impl Into<String>, mine: bool) -> Self {
        Self {
            text: text.into(),
            mine,
            attachment: None,
        }
    }

    /// 只有附件（照片 / 视频 / 文件）的气泡，`text` 留空作说明位。
    pub fn media(attachment: Attachment, mine: bool) -> Self {
        Self {
            text: String::new(),
            mine,
            attachment: Some(attachment),
        }
    }
}

/// A letter section of the address book from `contacts_list.json`.
#[derive(Clone, Default, Deserialize)]
pub struct ContactGroup {
    pub group: String,
    #[serde(default)]
    pub contacts: Vec<Contact>,
}

/// One address-book entry.
#[derive(Clone, Default, Deserialize)]
pub struct Contact {
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub initial: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub signature: String,
}

/// Everything the 500x600 profile card needs, so the card can be opened for a
/// chat *or* an address-book entry.
#[derive(Clone, Default)]
pub struct CardInfo {
    pub name: String,
    pub avatar: String,
    /// Green line under the name.
    pub subtitle: String,
    /// `(label, value)` rows of the card body.
    pub rows: Vec<(String, String)>,
}

impl CardInfo {
    pub fn from_chat(chat: &Chat) -> Self {
        let subtitle = if chat.details.is_empty() {
            format!("{} · {} 条消息", chat.last, chat.messages.len())
        } else {
            chat.details.clone()
        };
        Self {
            name: chat.name.clone(),
            avatar: chat.avatar.clone(),
            subtitle,
            rows: vec![
                ("最后消息".to_string(), chat.last.clone()),
                ("会话消息".to_string(), format!("{} 条", chat.messages.len())),
                ("未读消息".to_string(), chat.unread.to_string()),
            ],
        }
    }

    pub fn from_contact(contact: &Contact) -> Self {
        Self {
            name: contact.name.clone(),
            avatar: contact.avatar.clone(),
            subtitle: contact.signature.clone(),
            rows: vec![
                ("手机号".to_string(), contact.phone.clone()),
                ("分组".to_string(), contact.initial.clone()),
                ("微信号".to_string(), format!("cola_{}", contact.id)),
            ],
        }
    }
}

/// The chat list, with a demo conversation seeded into the first two threads.
pub fn load_chats() -> Vec<Chat> {
    let mut chats: Vec<Chat> = serde_json::from_str(CHAT_LIST_JSON)
        .expect("client/src/data/im/chat_list.json must be valid JSON matching `Chat`");
    for (index, chat) in chats.iter_mut().enumerate() {
        if chat.messages.is_empty() {
            chat.messages = demo_messages(index);
        }
    }
    chats
}

/// The address book, in file order (group sections are rendered as given).
pub fn load_contacts() -> Vec<ContactGroup> {
    serde_json::from_str(CONTACTS_LIST_JSON)
        .expect("client/src/data/im/contacts_list.json must be valid JSON matching `ContactGroup`")
}

/// First character of a name, used as the avatar fallback when the remote image
/// cannot be loaded (e.g. offline).
pub fn initial_of(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default()
}

/// The mock file ships empty `messages` arrays, which would leave the
/// conversation pane blank; seed the first two threads with the demo dialogue
/// that used to be hard-coded. Add `messages` to the JSON to override it.
fn demo_messages(index: usize) -> Vec<Message> {
    let build = |text: &str, mine: bool| Message::text(text, mine);
    match index {
        0 => vec![
            build("早上好！今天有空一起喝咖啡吗？", false),
            build("当然可以，附近新开了一家店 ☕", true),
            build("好呀，那我们中午见～", false),
            build("没问题，十二点见！", true),
        ],
        1 => vec![
            build("Hey! 最近怎么样？", false),
            build("挺好的，最近在做一个新的项目。", true),
            build("How are you?", false),
        ],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{CardInfo, Contact, Message, initial_of, load_chats, load_contacts};
    use crate::media::{Attachment, MediaKind};

    #[test]
    fn mock_chat_list_loads() {
        let chats = load_chats();
        assert_eq!(chats.len(), 12);
        assert_eq!(chats[0].name, "Alice");
        assert_eq!(chats[0].last, "Hello!");
        assert_eq!(chats[0].unread, 2);
        assert!(chats[0].avatar.starts_with("https"));
        assert!(!chats[0].messages.is_empty());
    }

    #[test]
    fn mock_contacts_load() {
        let groups = load_contacts();
        let contacts: usize = groups.iter().map(|group| group.contacts.len()).sum();
        assert_eq!(contacts, 12);
        assert_eq!(groups[0].group, "A");
        assert_eq!(groups[0].contacts[0].phone, "13800138001");
    }

    #[test]
    fn contact_card_uses_the_mock_fields() {
        let card = CardInfo::from_contact(&Contact {
            id: 3,
            name: "Charlie".to_string(),
            initial: "C".to_string(),
            phone: "13800138003".to_string(),
            avatar: String::new(),
            signature: "See you later!".to_string(),
        });
        assert_eq!(card.subtitle, "See you later!");
        assert_eq!(card.rows[0].1, "13800138003");
    }

    #[test]
    fn initial_falls_back_to_empty() {
        assert_eq!(initial_of("Alice"), "A");
        assert_eq!(initial_of("哈基米"), "哈");
        assert_eq!(initial_of(""), "");
    }

    /// 附件消息：内容全部留在内存里，不依赖文件真实存在（读不到就给 0，不 panic）。
    #[test]
    fn media_messages_keep_the_attachment_in_memory() {
        let attachment = Attachment::from_path(MediaKind::File, "no-such-file.zip");
        let message = Message::media(attachment, true);
        assert!(message.mine);
        assert!(message.text.is_empty());
        let attachment = message.attachment.expect("附件应该被保留");
        assert_eq!(attachment.kind, MediaKind::File);
        assert_eq!(attachment.name, "no-such-file.zip");
        assert_eq!(attachment.size, 0);
        assert_eq!(attachment.width, 0);
        assert_eq!(attachment.height, 0);
    }

    /// 纯文字消息不带附件。
    #[test]
    fn text_messages_have_no_attachment() {
        let message = Message::text("hello", false);
        assert_eq!(message.text, "hello");
        assert!(!message.mine);
        assert!(message.attachment.is_none());
    }
}
