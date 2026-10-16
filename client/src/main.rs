//! 可乐 · 桌面版 — 微信风格的 GPUI 演示客户端。
//!
//! 这是工作区里唯一的二进制包（`client/`，见仓库根 `Cargo.toml` 的 `[workspace]`）。
//!
//! 会话列表与通信录的数据来自 `client/src/data/im/*.json`（`include_str!` 编译期内嵌，
//! 见 `client/src/model.rs`）。头像是网络图片：`img("https://…")` 由 GPUI 的资源系统下载，
//! 所以启动时必须装一个真正的 HTTP 客户端（`client/src/net.rs`）—— GPUI 自带的空实现会让
//! 每个请求都以 “No HttpClient available” 失败。
//!
//! 仓库根目录的 `.env`（运行时读取，见 `client/src/config.rs`）里 `FPS_DISPLAY_MODE=1`
//! 会打开左上角的实时帧率表。
//!
//! 聊天输入与附件（照片 / 视频 / 文件）目前全部只留在内存里（`chats[..].messages`），
//! 两个「后期对接外部能力」的插槽先留好：
//!
//! - **消息 SDK**：气泡内容的构造集中在 `model.rs`（`Message::text` / `Message::media`），
//!   将来换成 SDK 下发的消息时，只改这里的解析即可；输入框也已经走 GPUI 的输入法处理器
//!   （`gpui-component` 的 `Input`），中文输入法、粘贴、选区开箱可用。
//! - **S3 上传**：`client/src/upload.rs` 的 `S3Uploader`（当前是 `StubUploader`，异步返回「未接入」）；
//!   缩略图的尺寸规则（边界 400x400、等比、不裁切）在 `client/src/media.rs`。
//!
//! 名片弹窗（`Messenger::card_overlay`）的开关动画：打开时缩放 0.9 → 1、不透明度 0 → 1，
//! 关闭反向，各 200ms（`CardView` / `advance_card_animation`）。

mod config;
mod media;
mod model;
mod net;
mod upload;

use gpui_kit::base::StyledExt;
use gpui_kit::component::input::{Input, InputEvent, InputState};
use gpui_kit::component::{h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use log::log;
use media::{Attachment, MediaKind};
use model::{CardInfo, Chat, Contact, ContactGroup, Message};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::info;
use upload::{S3Uploader, StubUploader, UploadRequest, UploadState};

//////// COLOR
const GREEN: u32 = 0x8bc34a;
const WHITE: u32 = 0xffffff;
const TEXT: u32 = 0x252525;
const MUTED: u32 = 0x999999;
const LINE: u32 = 0xe8e8e8;
/// 会话列表底色。
const PANEL: u32 = 0xf8f8f8;
/// 名片弹窗侧边「关闭」条底色 (#0a0a0a)。
const INK: u32 = 0x0a0a0a;

/// 会话列表头像的兜底底色，按下标循环使用。
const AVATAR_TINTS: [u32; 12] = [
    0xdabf9f, 0x9fb8c9, 0xb9abcf, 0xcaaa9d, 0xa2c5b5, 0xb1bd89, 0xcbaec0, 0x8dbab1, 0xd3a595,
    0xc7b28e, 0xa9b9d3, 0xc49d9f,
];
/// 消息气泡 / 名片头像的兜底色。
const TINT_CHAT: u32 = 0xd8c2a8;
const TINT_ME: u32 = 0xc6a68b;
const TINT_CARD: u32 = 0xdabf9f;

/// 名片弹窗尺寸：卡片 400x600；「关闭」长条 360x45，放在「发消息」按钮正下方。
const CARD_W: f32 = 400.;
const CARD_H: f32 = 600.;
const CARD_CLOSE_W: f32 = 360.;
const CARD_CLOSE_H: f32 = 45.;
//////// 时间常量：所有「多久」都集中在这里，方便后期统一调整

/// 名片弹窗的开 / 关动画时长：打开、关闭各 200ms。
const CARD_ANIM: Duration = Duration::from_millis(200);
/// 打开动画的起始缩放：0.9 → 1（关闭时反向 1 → 0.9）。
const CARD_ANIM_SCALE_FROM: f32 = 0.9;
/// 聊天气泡区「有新消息时自动滚到底部」的动画时长：300ms。
const CHAT_SCROLL_ANIM: Duration = Duration::from_millis(300);
/// 帧率表的统计窗口：每 500ms 重新数一次帧数（`.env` 打开 FPS 表后才用得到）。
const FPS_SAMPLE_WINDOW: Duration = Duration::from_millis(500);

//////// 聊天气泡区的自动滚动 / 位置记忆

/// 一次「有新消息 → 平滑滚到底部」动画的起点。
///
/// 滚动偏移沿用 GPUI 的约定：**越往下越负**（顶部是 `0`，底部是 `-max_offset`）。
#[derive(Clone, Copy, Debug)]
struct ScrollAnim {
    /// 动画开始时间，用来算进度。
    since: Instant,
    /// 动画开始时的滚动偏移 y（负值）。
    from_y: f32,
}

/// 每个会话各自记住自己的滚动位置：`key = Chat::id`，`value = 滚动偏移 y`。
///
/// 只在切走某个会话时写入一次（读的是当时的实时偏移），所以手动滚动、发消息
/// 之后的滚动都会被如实记下来；切回去时按记录恢复，而不是每次都滚到底部。
type ChatScrollOffsets = HashMap<usize, f32>;

/// 切到某个会话时要恢复的偏移。
///
/// 记录过就用记录值；**没记录过从顶部（`0`）开始**——不主动滚到底部（滚到底只由
/// 新消息触发，见 `Messenger::scroll_chat_to_bottom`）。
fn restored_scroll_offset(offsets: &ChatScrollOffsets, chat_id: usize) -> f32 {
    offsets.get(&chat_id).copied().unwrap_or(0.0)
}

/// 平滑滚动的插值：`eased` 从 0 走到 1 时，偏移从 `from_y` 走到 `target_y`。
///
/// 单独抽成纯函数是为了能单测（渲染里那段拿不到 `Window`，没法直接测）。
fn scroll_offset_at(from_y: f32, target_y: f32, eased: f32) -> f32 {
    from_y + (target_y - from_y) * eased
}

//////// 名片弹窗的开关动画

/// 名片弹窗的动画阶段。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CardPhase {
    /// 打开动画进行中，值是动画起点。
    Opening(Instant),
    /// 已经完全打开（动画停在终点）。
    Open,
    /// 关闭动画进行中，值是动画起点；走完才真正把弹窗移除。
    Closing(Instant),
}

/// 名片弹窗 = 名片内容 + 开关动画阶段。
///
/// - 打开：缩放 `0.9 → 1`、不透明度 `0 → 1`；
/// - 关闭：同一条缓动曲线**反向**（`1 → 0.9`、`1 → 0`）；
/// - 两者时长都是 [`CARD_ANIM`]（200ms），卡片始终居中，所以缩放围绕中心。
///
/// GPUI 这个版本的元素样式里没有 `transform: scale`（只有 `scale_factor` 那种
/// DPI 缩放），所以「zoom」是用**布局尺寸**做的：把卡片的宽高、内边距、字号、
/// 头像尺寸一起乘上缩放系数，视觉上等价于整体缩放。
#[derive(Clone)]
struct CardView {
    card: CardInfo,   // 名片内容
    phase: CardPhase, // 当前动画阶段
}

impl CardView {
    /// # [VIEW] - 开始打开动画
    /// * `desc`: 封面尺寸从 90% 弹到 100%，不透明度 0 → 1
    fn opening(card: CardInfo) -> Self {
        Self {
            card,
            phase: CardPhase::Opening(Instant::now()),
        }
    }

    /// # [VIEW] - 开始关闭动画
    /// * `desc`: 保留内容，把阶段切到 `Closing`，反向播一遍
    fn closing(&self) -> Self {
        Self {
            card: self.card.clone(),
            phase: CardPhase::Closing(Instant::now()),
        }
    }

    /// # [VIEW] - 取当前动画进度
    /// * `desc`: `1.0` = 完全展开、`0.0` = 完全收起；第二个返回值表示动画是否还在跑
    /// * `condition`: `关闭时时间轴倒过来跑（1 → 0），所以关闭就是打开的反向`
    fn progress(&self, now: Instant) -> (f32, bool) {
        let elapsed = |since: Instant| {
            now.saturating_duration_since(since).as_secs_f32() / CARD_ANIM.as_secs_f32()
        };
        match self.phase {
            CardPhase::Opening(since) => {
                let t = elapsed(since);
                (ease_in_out(t.clamp(0.0, 1.0)), t < 1.0)
            }
            CardPhase::Open => (1.0, false),
            CardPhase::Closing(since) => {
                let t = elapsed(since);
                (ease_in_out((1.0 - t).clamp(0.0, 1.0)), t < 1.0)
            }
        }
    }

    /// # [VIEW] - 当前帧的缩放与不透明度
    /// * `desc`: 缩放 `0.9 → 1`、不透明度 `0 → 1`，都跟着同一个进度走
    fn visual(&self, now: Instant) -> (f32, f32) {
        let (t, _) = self.progress(now);
        (
            CARD_ANIM_SCALE_FROM + (1.0 - CARD_ANIM_SCALE_FROM) * t,
            t,
        )
    }

    /// # [VIEW] - 正在播的动画是否已经走完
    /// * `desc`: `Open` 是静止状态，永远返回 `false`
    fn done(&self, now: Instant) -> bool {
        match self.phase {
            CardPhase::Opening(since) | CardPhase::Closing(since) => {
                now.saturating_duration_since(since) >= CARD_ANIM
            }
            CardPhase::Open => false,
        }
    }
}

/////////
struct Messenger {
    /// 会话列表（`chat_list.json`）。
    chats: Vec<Chat>,
    /// 通信录分组（`contacts_list.json`）。
    contacts: Vec<ContactGroup>,
    /// `chats` 中当前会话的下标。
    selected: usize,
    /// 通信录里高亮联系人的 `id`。
    selected_contact: Option<usize>,
    /// 左侧导航下标：0 消息、1 联系人、2 动态、3 视频、4 商城、5 我的、6 设置。
    section: usize,
    /// 消息输入框。用 `gpui-component` 的 `Input`，中文输入法 / 长文本 / 粘贴 / 选区都能用。
    input: Entity<InputState>,
    /// `InputEvent` 的订阅句柄：一丢掉订阅就失效，所以必须挂在结构体上。
    _input_events: Subscription,
    /// 聊天气泡区的滚动句柄：绑到消息容器上（`track_scroll`），用来读偏移 / 写偏移。
    chat_scroll: ScrollHandle,
    /// 每个会话各自的滚动位置（`key = Chat::id`）；切走时记录、切回时恢复。
    chat_scroll_offsets: ChatScrollOffsets,
    /// 切会话后要恢复的偏移：`Some(y)` = 下一帧把滚动位置设成 y（**一次性**，设完清空），
    /// `None` = 不干预滚动（不然每帧都写会把用户的手动滚动顶掉）。
    chat_scroll_restore: Option<f32>,
    /// 「有新消息自动滚到底部」的动画状态；`None` = 当前没有正在播的滚动动画。
    chat_scroll_anim: Option<ScrollAnim>,
    /// 预留的 S3 上传器（当前是 `StubUploader`，后期换成真实实现，界面代码不用改）。
    uploader: Arc<dyn S3Uploader>,
    toast: Option<String>,
    /// 500x600 名片弹窗 + 开关动画；`None` 表示没有弹窗。
    card: Option<CardView>,
    show_emoji: bool,
    // 帧率诊断，由 `.env` 的 FPS_DISPLAY_MODE 打开。
    fps_overlay: bool,
    fps_frames: u32,
    fps_since: Instant,
    fps: u32,
    window_active: bool,
}

////////
impl Messenger {
    /// 输入框的占位提示（和原来那行提示文字保持一致）。
    const INPUT_PLACEHOLDER: &'static str = "输入消息，按 Enter 发送";

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 真正的文本输入：走 GPUI 的 `InputHandler`，所以中文输入法、粘贴、
        // 选区、光标移动都是系统行为，不再是「一个字符一个按键地拼」。
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(Self::INPUT_PLACEHOLDER));

        // Enter 发送（单行输入不会插入换行，`PressEnter` 由输入框发出）。
        let _input_events = cx.subscribe_in(
            &input,
            window,
            |this, _input, event: &InputEvent, window, cx| {
                if matches!(event, InputEvent::PressEnter { .. }) {
                    this.send(window, cx);
                }
            },
        );

        // 打开窗口就能直接打字。
        input.update(cx, |state, cx| state.focus(window, cx));

        Self {
            chats: model::load_chats(),
            contacts: model::load_contacts(),
            selected: 0,
            selected_contact: None,
            section: 0,
            input,
            _input_events,
            chat_scroll: ScrollHandle::new(),
            chat_scroll_offsets: HashMap::new(),
            chat_scroll_restore: None,
            chat_scroll_anim: None,
            // 预留：S3 还没接入，所有附件都只留在内存里。
            uploader: Arc::new(StubUploader::new()),
            toast: None,
            card: None,
            show_emoji: false,
            fps_overlay: config::fps_display_enabled(),
            fps_frames: 0,
            fps_since: Instant::now(),
            fps: 0,
            window_active: false,
        }
    }

    // 发送消息：把输入框里的文字取出来，塞进当前会话的消息列表（内存），然后清空输入框。
    fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.input.read(cx).value().trim().to_string();
        if text.is_empty() {
            return;
        }
        if let Some(chat) = self.chats.get_mut(self.selected) {
            chat.messages.push(Message::text(text, true));
        }
        // 新消息进来了：300ms 平滑滚到底部（见 `advance_chat_scroll`）。
        self.scroll_chat_to_bottom();
        self.input
            .update(cx, |state, cx| state.set_value("", window, cx));
        cx.notify();
    }
    /// # [HANDLER] - 选择照片 / 视频 / 文件
    ///
    /// 点击工具栏的「照片 / 视频 / 文件」都会走到这里：调用操作系统的文件选择对话框
    /// （GPUI 的 `App::prompt_for_paths`，Windows 上就是原生 IFileOpenDialog），
    /// 选中的路径异步回到渲染线程，再按附件发送。
    ///
    /// `kind` 只是兜底：GPUI 的选择器没有后缀过滤，真正的类型按扩展名重新判断。
    fn pick_media(&mut self, kind: MediaKind, window: &mut Window, cx: &mut Context<Self>) {
        let options = PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some(format!("选择{}", kind.label()).into()),
        };
        let receiver = cx.prompt_for_paths(options);

        cx.spawn_in(window, async move |this, cx| {
            let paths = match receiver.await {
                Ok(Ok(Some(paths))) => paths,
                // 用户点了「取消」。
                Ok(Ok(None)) => return,
                Ok(Err(error)) => {
                    let _ = this.update_in(cx, |this, _window, cx| {
                        this.notice(&format!("打开文件选择器失败：{error}"), cx);
                    });
                    return;
                }
                Err(_) => return,
            };
            let _ = this.update_in(cx, |this, _window, cx| this.attach_files(kind, paths, cx));
        })
        .detach();
    }

    /// # [HANDLER] - 把选中的文件变成气泡
    ///
    /// 内容全部留在内存（`chats[..].messages`）里，气泡直接用本地路径渲染缩略图。
    /// 同时给每个附件挂一次预留的 S3 上传（`spawn_upload`），后期接上真实实现即可。
    fn attach_files(&mut self, kind: MediaKind, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let chat_index = self.selected;
        let uploader = self.uploader.clone();
        let mut pending: Vec<(usize, UploadRequest)> = Vec::new();

        for path in paths {
            // 按后缀重新归类（点「照片」却选了 .mp4 时以文件本身为准）。
            let kind = media::kind_of_path(&path).unwrap_or(kind);
            let attachment = Attachment::from_path(kind, path);
            let request = UploadRequest::for_path(&attachment.path);
            let Some(chat) = self.chats.get_mut(chat_index) else {
                continue;
            };
            let index = chat.messages.len();
            chat.messages.push(Message::media(attachment, true));
            if let Some(attachment) = chat
                .messages
                .get_mut(index)
                .and_then(|message| message.attachment.as_mut())
            {
                attachment.upload = UploadState::Uploading;
            }
            pending.push((index, request));
        }

        if pending.is_empty() {
            return;
        }

        for (index, request) in pending {
            let uploader = uploader.clone();
            cx.spawn(async move |this, cx| {
                // 预留的 S3 上传：当前 `StubUploader` 会立刻返回「未接入」，
                // 换成真实实现后这里就是真正的异步上传，界面不用改。
                let result = uploader.upload(request).await;
                let _ = this.update(cx, |this, cx| this.finish_upload(chat_index, index, result, cx));
            })
            .detach();
        }

        // 附件也是「新消息」：同样滚到底部。
        self.scroll_chat_to_bottom();
        cx.notify();
    }

    /// # [HANDLER] - 上传结束
    ///
    /// 把结果写回附件（内存）。当前界面不展示上传状态，等接上 S3 后
    /// 用 `UploadState::is_uploaded()` / `url()` 在气泡上画角标即可。
    fn finish_upload(
        &mut self,
        chat_index: usize,
        message_index: usize,
        result: Result<String, String>,
        cx: &mut Context<Self>,
    ) {
        let state = match result {
            Ok(url) => UploadState::Uploaded { url },
            Err(reason) => UploadState::Failed { reason },
        };
        let attachment = self
            .chats
            .get_mut(chat_index)
            .and_then(|chat| chat.messages.get_mut(message_index))
            .and_then(|message| message.attachment.as_mut());
        if let Some(attachment) = attachment {
            attachment.upload = state;
            cx.notify();
        }
    }



    /// # [HANDLER] - 通知
    fn notice(&mut self, text: &str, cx: &mut Context<Self>) {
        self.toast = Some(text.to_string());
        cx.notify();
    }

    /// # [HANDLER] - 打开名片
    /// * `desc`: 弹窗（点击会话标题或通信录联系人），从 90% 缩放 + 全透明淡入
    fn open_card(&mut self, card: CardInfo, cx: &mut Context<Self>) {
        self.card = Some(CardView::opening(card));
        cx.notify();
    }

    /// # [HANDLER] - 关闭名片
    /// * `desc`: 点遮罩、右上角 ✕、右侧贯穿关闭条都会调用它；反向播一遍动画后移除
    /// * `condition`: `已经在关闭中就不要再重启动画（几处入口可能连续触发）`
    fn close_card(&mut self, cx: &mut Context<Self>) {
        let Some(view) = self.card.as_ref() else {
            return;
        };
        if matches!(view.phase, CardPhase::Closing(_)) {
            return;
        }
        self.card = Some(view.closing());
        cx.notify();
    }

    /// # [VIEW] - 推进名片开关动画
    /// * `desc`: 每帧调用一次，返回 `(名片内容, 缩放, 不透明度)`；动画期间持续请求下一帧
    /// * `condition`: `关闭动画播完才把 self.card 置空；系统开了「减少动态效果」则直接跳到终态`
    fn advance_card_animation(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<(CardInfo, f32, f32)> {
        let now = Instant::now();
        let mut remove = false;

        if let Some(view) = self.card.as_mut() {
            // 系统级「减少动态效果」时不播动画：打开直接停在终点，关闭直接结束。
            if cx.reduce_motion() {
                match view.phase {
                    CardPhase::Opening(_) => view.phase = CardPhase::Open,
                    CardPhase::Closing(_) => remove = true,
                    CardPhase::Open => {}
                }
            } else if view.done(now) {
                match view.phase {
                    // 打开动画走完 → 停在终点。
                    CardPhase::Opening(_) => view.phase = CardPhase::Open,
                    // 关闭动画走完 → 真正移除。
                    CardPhase::Closing(_) => remove = true,
                    CardPhase::Open => {}
                }
            }
        }
        if remove {
            self.card = None;
        }

        let view = self.card.as_ref()?;
        let (scale, opacity) = view.visual(now);
        if view.progress(now).1 {
            // 动画还没走完：不请求下一帧就会停在半路。
            window.request_animation_frame();
        }
        let card = view.card.clone();
        Some((card, scale, opacity))
    }

    /// # [HANDLER] - 切换会话（点会话列表）
    /// * `desc`: 记住当前会话滚到哪儿了，切过去后恢复**目标会话自己**的位置
    /// * `condition`: `切会话不滚到底部！滚到底只由新消息触发；没记录过的会话从顶部开始`
    fn switch_chat(&mut self, index: usize, cx: &mut Context<Self>) {
        self.remember_chat_scroll();
        self.selected = index;
        self.section = 0;
        // 上一个会话没播完的「滚到底」动画要取消，不然它会继续把新会话拉到它的底部。
        self.chat_scroll_anim = None;
        // 恢复目标会话上次的位置（一次性，下一帧应用，见 render）。
        let chat_id = self.chats.get(index).map(|chat| chat.id);
        self.chat_scroll_restore = Some(match chat_id {
            Some(chat_id) => restored_scroll_offset(&self.chat_scroll_offsets, chat_id),
            None => 0.0,
        });
        cx.notify();
    }

    /// # [VIEW] - 记住当前会话的滚动位置
    /// * `desc`: 读的是**实时**偏移，所以手动滚动 / 发消息后的位置都会被如实记下来
    fn remember_chat_scroll(&mut self) {
        let Some(chat_id) = self.chats.get(self.selected).map(|chat| chat.id) else {
            return;
        };
        let y: f32 = self.chat_scroll.offset().y.into();
        self.chat_scroll_offsets.insert(chat_id, y);
    }

    /// # [VIEW] - 应用「切会话后要恢复的滚动位置」（每帧最多一次）
    /// * `desc`: 只写一帧就清空——每帧都写会把用户的手动滚动顶掉
    /// * `condition`: `内容比记录的短时由 GPUI 自己夹到边界内，所以这里不用先算 max_offset`
    fn apply_chat_scroll_restore(&mut self) {
        if let Some(y) = self.chat_scroll_restore.take() {
            self.chat_scroll.set_offset(point(px(0.), px(y)));
        }
    }

    /// # [VIEW] - 请求「聊天气泡区滚到底部」
    /// * `desc`: **只有新消息**（发出文字 / 发出附件）时调用；
    ///   这里只记下起点，真正的滚动放在 `render` 里逐帧推进（那一帧才拿得到 `Window`）
    /// * `condition`: `偏移要等新消息布局完才准，所以动画目标每帧都重新读一次`
    fn scroll_chat_to_bottom(&mut self) {
        // 滚到底优先于「恢复位置」：两者同时挂着的话以新消息为准。
        self.chat_scroll_restore = None;
        self.chat_scroll_anim = Some(ScrollAnim {
            since: Instant::now(),
            from_y: self.chat_scroll.offset().y.into(),
        });
    }

    /// # [VIEW] - 推进「滚到底部」动画（每帧一次）
    /// * `desc`: 把滚动偏移从起点插值到最新的底部，走满 `CHAT_SCROLL_ANIM`（300ms）
    /// * `condition`: `系统开了「减少动态效果」就直接跳到底部；布局又变高时目标会跟着变大`
    fn advance_chat_scroll(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(anim) = self.chat_scroll_anim else {
            return;
        };
        // 目标：最新的底部偏移。GPUI 的偏移「越往下越负」，所以取负的最大偏移。
        // 每帧重新读一次，消息在动画期间又被追加时也能准确落到新的底部。
        let target_y = -f32::from(self.chat_scroll.max_offset().y);

        // 不播动画：直接落到底部（无障碍 / 省电场景）。
        if cx.reduce_motion() {
            self.chat_scroll.set_offset(point(px(0.), px(target_y)));
            self.chat_scroll_anim = None;
            return;
        }

        let elapsed = Instant::now().saturating_duration_since(anim.since);
        let t = elapsed.as_secs_f32() / CHAT_SCROLL_ANIM.as_secs_f32();

        // 时间到：精确落到最新的底部（不是按插值算出来的近似值），然后收工。
        if t >= 1.0 {
            self.chat_scroll.set_offset(point(px(0.), px(target_y)));
            self.chat_scroll_anim = None;
            return;
        }

        let y = scroll_offset_at(anim.from_y, target_y, ease_in_out(t.clamp(0.0, 1.0)));
        self.chat_scroll.set_offset(point(px(0.), px(y)));
        // 还没走完：请求下一帧，否则动画会停在半路。
        window.request_animation_frame();
    }
}

/// 网络头像：`img()` 由 GPUI 的资源系统从 CDN 拉取（这就是本项目要自带 HTTP 客户端
/// 的原因）。加载中显示浅灰方块；失败（离线 / 404 / 域名不通）时回退到首字母色块，
/// 所以断网时界面也不会出现空洞。
fn remote_avatar(
    id: SharedString,
    url: &str,
    initial: String,
    size: f32,
    radius: f32,
    tint: u32,
) -> AnyElement {
    if url.is_empty() {
        return letter_avatar(initial, size, radius, tint);
    }
    let fallback_initial = initial.clone();
    img(url.to_string())
        .id(id)
        .w(px(size))
        .h(px(size))
        .rounded(px(radius))
        .object_fit(ObjectFit::Cover)
        .with_loading(move || {
            div()
                .w(px(size))
                .h(px(size))
                .rounded(px(radius))
                .bg(rgb(0xdddddd))
                .into_any_element()
        })
        .with_fallback(move || letter_avatar(fallback_initial.clone(), size, radius, tint))
        .into_any_element()
}

/// 首字母色块头像：网络头像的兜底，也用于「我」。
fn letter_avatar(initial: String, size: f32, radius: f32, tint: u32) -> AnyElement {
    div()
        .w(px(size))
        .h(px(size))
        .flex_shrink_0()
        .rounded(px(radius))
        .bg(rgb(tint))
        .items_center()
        .justify_center()
        .text_color(rgb(WHITE))
        .text_size(px(size * 0.42))
        .child(initial)
        .into_any_element()
}

/// 缩略图占位（照片 / 视频正在加载或读不出来时）：占住同样的边界，避免气泡跳动。
fn thumbnail_placeholder(width: f32, height: f32, label: &'static str) -> AnyElement {
    div()
        .w(px(width))
        .h(px(height))
        .items_center()
        .justify_center()
        .text_color(rgb(0x8a8a8a))
        .text_size(px(12.))
        .child(label)
        .into_any_element()
}

/// `.env` 打开帧率表后的分档配色：>100 绿、60-100 黄、40-60 橙、<40 红。
fn fps_color(fps: u32) -> u32 {
    if fps > 100 {
        0x00ff00
    } else if fps >= 60 {
        0xffff00
    } else if fps >= 40 {
        0xff6600
    } else {
        0xff0000
    }
}

impl Messenger {
    /// 一条消息气泡（右键打开操作菜单）。文字消息渲染文字，附件消息渲染缩略图 / 文件卡片。
    fn bubble(&self, message: &Message, index: usize, cx: &mut Context<Self>) -> Div {
        let mine = message.mine;
        let attachment = message.attachment.clone();
        let chat = self.chats.get(self.selected);
        let bubble_color = if mine { 0xe7e7e7 } else { GREEN };
        let foreground = if mine { TEXT } else { 0x23301b };
        // 附件气泡的内容：缩略图（照片 / 视频）或文件卡片。
        let content: AnyElement = match &attachment {
            Some(attachment) => self.attachment_content(attachment, index, cx),
            None => div().child(message.text.clone()).into_any_element(),
        };
        let avatar = if mine {
            letter_avatar("我".to_string(), 40., 8., TINT_ME)
        } else {
            let name = chat.map(|chat| chat.name.as_str()).unwrap_or_default();
            let url = chat.map(|chat| chat.avatar.as_str()).unwrap_or_default();
            remote_avatar(
                SharedString::from(format!("bubble-avatar-{}-{index}", self.selected)),
                url,
                model::initial_of(name),
                40.,
                8.,
                TINT_CHAT,
            )
        };
        let bubble = div()
            .id(SharedString::from(format!(
                "message-{}-{}",
                self.selected, index
            )))
            .max_w(px(440.))
            .min_w(px(40.))
            .rounded_lg()
            .bg(rgb(bubble_color))
            .text_color(rgb(foreground))
            .text_size(px(14.))
            // 缩略图自己带圆角和边界，所以附件气泡只留一点点内边距，
            // 文字气泡还是原来的 40 高 + 左右 12 上下 8。
            .when_else(
                attachment.is_some(),
                |el| el.p_1(),
                |el| el.min_h(px(40.)).px_3().py_2(),
            )
            .child(content)
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, _, window, cx| {
                    this.notice("消息操作：收藏 · 撤回 · 删除", cx);
                    let _ = window.prompt(
                        PromptLevel::Info,
                        "消息操作",
                        Some(&format!("消息 #{}：收藏 · 撤回 · 删除", index + 1)),
                        &["收藏", "撤回", "删除", "取消"],
                        cx,
                    );
                }),
            );

        let row = h_flex().w_full().mb_5().gap_3().items_start().when_else(
            mine,
            |el| el.justify_end(),
            |el| el.justify_start(),
        );
        // 自己发的消息：气泡在左、头像在右；对方的消息：头像在左、气泡在右。
        if mine {
            row.child(bubble).child(avatar)
        } else {
            row.child(avatar).child(bubble)
        }
    }
    /// 附件气泡的内容。
    ///
    /// - 照片 / 视频：本地文件的缩略图，边界 400x400、等比缩小、**不裁切**（`ObjectFit::Contain`）；
    /// - 其它文件：图标 + 文件名 + 大小 的横向卡片。
    ///
    /// 三种都能点一下：交给系统默认程序打开（`App::open_with_system`）。
    fn attachment_content(
        &self,
        attachment: &Attachment,
        index: usize,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id_prefix = format!("attachment-{}-{index}", self.selected);

        if matches!(attachment.kind, MediaKind::Image | MediaKind::Video) {
            let (width, height) = attachment.display_size();
            let path = attachment.path.clone();
            let listener_path = path.clone();
            let mut frame = div()
                .id(SharedString::from(format!("{id_prefix}-frame")))
                .relative()
                .flex_shrink_0()
                .w(px(width))
                .h(px(height))
                .overflow_hidden()
                .rounded(px(6.))
                .bg(rgba(0x00000012))
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| this.open_attachment(&listener_path, cx)),
                )
                .child(
                    img(path)
                        .id(SharedString::from(format!("{id_prefix}-thumb")))
                        .w(px(width))
                        .h(px(height))
                        // 不裁切：整张图等比缩进 400x400 的边界里。
                        .object_fit(ObjectFit::Contain)
                        .with_loading(move || thumbnail_placeholder(width, height, "加载中…"))
                        .with_fallback(move || thumbnail_placeholder(width, height, "无法预览")),
                );
            if attachment.kind == MediaKind::Video {
                frame = frame.child(
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .size_full()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(48.))
                                .h(px(48.))
                                .rounded_full()
                                .bg(rgba(0x00000099))
                                .items_center()
                                .justify_center()
                                .text_color(rgb(WHITE))
                                .text_size(px(18.))
                                .child("▷"),
                        ),
                );
            }
            return frame.into_any_element();
        }

        let name = attachment.name.clone();
        let size = media::human_size(attachment.size);
        let icon = attachment.kind.icon();
        let path = attachment.path.clone();
        h_flex()
            .id(SharedString::from(id_prefix))
            .min_w(px(220.))
            .max_w(px(380.))
            .px_3()
            .py_2()
            .gap_3()
            .items_center()
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| this.open_attachment(&path, cx)),
            )
            .child(
                div()
                    .w(px(40.))
                    .h(px(40.))
                    .flex_shrink_0()
                    .rounded_md()
                    .bg(rgba(0x00000014))
                    .items_center()
                    .justify_center()
                    .text_size(px(18.))
                    .text_color(rgb(0x777777))
                    .child(icon),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(
                        div()
                            .text_ellipsis()
                            .text_size(px(13.))
                            .text_color(rgb(TEXT))
                            .child(name),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(0x808080))
                            .child(size),
                    ),
            )
            .into_any_element()
    }

    /// # [HANDLER] - 用系统默认程序打开附件（点气泡里的缩略图 / 文件卡片）。
    fn open_attachment(&mut self, path: &Path, cx: &mut Context<Self>) {
        if !path.exists() {
            self.notice("文件已不在本机，无法打开", cx);
            return;
        }
        cx.open_with_system(path);
    }
}

impl Messenger {
    fn nav_item(
        &self,
        index: usize,
        icon: &'static str,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let selected = self.section == index;
        let color = if selected { GREEN } else { 0x777777 };
        div()
            .id(SharedString::from(format!("nav-{index}")))
            .w_full()
            .h_16()
            .mb_2()
            .v_flex()
            .items_center()
            .justify_center()
            .gap_1()
            .rounded_md()
            .cursor_pointer()
            .bg(rgb(if selected { 0xe8eee3 } else { 0xf5f5f5 }))
            .text_color(rgb(color))
            .child(div().text_size(px(22.)).child(icon))
            .child(div().text_size(px(11.)).child(label))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.section = index;
                cx.notify();
            }))
    }

    /// 工具栏按钮的「壳」：只画图标，点击行为交给调用方。
    ///
    /// 之所以拆开：一个元素上可以有多个 `on_click` 监听（它们会**全部**触发），
    /// 所以「照片 / 视频 / 文件」需要自己的点击行为时，不能在壳里再挂一个默认提示。
    fn toolbar_icon(&self, id: &'static str, icon: &'static str) -> Stateful<Div> {
        div()
            .id(id)
            .size_8()
            .rounded_md()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .text_color(rgb(0x777777))
            .text_size(px(19.))
            .child(icon)
    }

    /// 只弹个提示的工具栏按钮（位置、红包、名片等还没实现的功能）。
    fn toolbar_button(
        &self,
        id: &'static str,
        icon: &'static str,
        tip: &'static str,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        self.toolbar_icon(id, icon)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.notice(tip, cx);
            }))
    }

    /// 会话列表（数据来自 `chat_list.json`）。
    fn chat_list(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div().id("chat-list").flex_1().overflow_y_scroll().children(
            self.chats
                .iter()
                .enumerate()
                .map(|(index, chat)| self.chat_item(index, chat, cx)),
        )
    }

    /// 一行会话：左头像、右文字列，两者横向并排（`h_flex`），不再上下错位。
    fn chat_item(&self, index: usize, chat: &Chat, cx: &mut Context<Self>) -> Stateful<Div> {
        let active = index == self.selected;
        let bg = if active { 0xe9e9e9 } else { PANEL };
        let tint = AVATAR_TINTS[index % AVATAR_TINTS.len()];
        let unread = if chat.unread > 99 {
            "99+".to_string()
        } else {
            chat.unread.to_string()
        };
        let id = chat.id;
        h_flex()
            .id(SharedString::from(format!("chat-{id}")))
            .h(px(68.))
            .px_3()
            .gap_3()
            .items_center()
            .bg(rgb(bg))
            .cursor_pointer()
            .child(remote_avatar(
                SharedString::from(format!("chat-avatar-{id}")),
                &chat.avatar,
                model::initial_of(&chat.name),
                40.,
                8.,
                tint,
            ))
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(
                        h_flex()
                            .justify_between()
                            .gap_1()
                            .child(
                                div()
                                    .flex_1()
                                    .text_ellipsis()
                                    .text_size(px(14.))
                                    .text_color(rgb(TEXT))
                                    .child(chat.name.clone()),
                            )
                            .when(!chat.time.is_empty(), |el| {
                                el.child(
                                    div()
                                        .text_size(px(10.))
                                        .text_color(rgb(MUTED))
                                        .child(chat.time.clone()),
                                )
                            }),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .gap_1()
                            .child(
                                div()
                                    .flex_1()
                                    .text_ellipsis()
                                    .text_size(px(12.))
                                    .text_color(rgb(MUTED))
                                    .child(chat.last.clone()),
                            )
                            .when(chat.unread > 0, |el| {
                                el.child(
                                    div()
                                        .min_w_4()
                                        .h_4()
                                        .px_1()
                                        .rounded_full()
                                        .bg(rgb(0xff0000))
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(9.))
                                        .text_color(rgb(WHITE))
                                        .child(unread),
                                )
                            }),
                    ),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                // 切会话：记住旧会话的位置，恢复新会话自己的位置（不滚到底部）。
                this.switch_chat(index, cx);
            }))
    }
}

/// 通信录的分组标题（JSON 里的 `group` 字母）。`index` 参与元素 id，避免同名分组
/// （示例数据里有两个 `B`、两个 `W`）产生重复 id。
fn group_header(index: usize, letter: &str) -> Stateful<Div> {
    div()
        .id(SharedString::from(format!("contact-group-{index}")))
        .h(px(26.))
        .px_3()
        .flex_shrink_0()
        .items_center()
        .bg(rgb(0xf0f0f0))
        .text_size(px(12.))
        .text_color(rgb(MUTED))
        .child(letter.to_string())
}

impl Messenger {
    /// 通信录列表（数据来自 `contacts_list.json`），保留 JSON 里的分组顺序。
    fn contacts_list(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        let mut rows: Vec<AnyElement> = Vec::new();
        for (group_index, group) in self.contacts.iter().enumerate() {
            rows.push(group_header(group_index, &group.group).into_any_element());
            for contact in &group.contacts {
                rows.push(self.contact_row(contact, cx).into_any_element());
            }
        }
        div()
            .id("contacts-list")
            .flex_1()
            .overflow_y_scroll()
            .children(rows)
    }

    /// 一行联系人：圆形网络头像 + 名字 + 签名，点击弹出名片。
    fn contact_row(&self, contact: &Contact, cx: &mut Context<Self>) -> Stateful<Div> {
        let active = self.selected_contact == Some(contact.id);
        let bg = if active { 0xe9e9e9 } else { PANEL };
        let id = contact.id;
        let card = CardInfo::from_contact(contact);
        h_flex()
            .id(SharedString::from(format!("contact-{id}")))
            .h(px(62.))
            .px_3()
            .gap_3()
            .items_center()
            .bg(rgb(bg))
            .cursor_pointer()
            .child(remote_avatar(
                SharedString::from(format!("contact-avatar-{id}")),
                &contact.avatar,
                model::initial_of(&contact.name),
                36.,
                18.,
                0x9fb8c9,
            ))
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(
                        div()
                            .text_ellipsis()
                            .text_size(px(14.))
                            .text_color(rgb(TEXT))
                            .child(contact.name.clone()),
                    )
                    .child(
                        div()
                            .text_ellipsis()
                            .text_size(px(11.))
                            .text_color(rgb(MUTED))
                            .child(contact.signature.clone()),
                    ),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected_contact = Some(id);
                this.open_card(card.clone(), cx);
            }))
    }
}

impl Messenger {
    /// 名片弹窗：半透明遮罩（点击关闭）之上是 500x600 的卡片；卡片里最后是「发消息」
    /// 按钮，紧跟着一条 iOS 风格的黑色长条「关闭」（400x45，#0a0a0a 底、白字 18px 居中）。
    /// 卡片自身有鼠标监听，点击卡片内部不会穿透到遮罩，所以不会误关。
    ///
    /// `scale` / `opacity` 来自 [`Messenger::advance_card_animation`]：打开时
    /// `0.9 → 1` / `0 → 1`（关闭反向）。GPUI 没有元素级的 `scale` 变换，所以这里把
    /// 卡片的**每一处尺寸**（宽高、内边距、间距、字号、头像）都乘上 `scale`，
    /// 效果等同于把整张卡片等比缩放；卡片本身居中，于是缩放围绕中心点。
    fn card_overlay(
        &self,
        card: CardInfo,
        scale: f32,
        opacity: f32,
        cx: &mut Context<Self>,
    ) -> Div {
        let backdrop = div()
            .id("card-backdrop")
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .size_full()
            .bg(rgba(0x00000066))
            // 遮罩跟着一起淡入淡出，开关时不突兀。
            .opacity(opacity)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.close_card(cx)),
            );

        let close_button = div()
            .id("card-close")
            .w(px(32. * scale))
            .h(px(32. * scale))
            .flex_shrink_0()
            .rounded_full()
            .cursor_pointer()
            .items_center()
            .justify_center()
            .bg(rgb(0xf2f2f2))
            .text_color(rgb(0x555555))
            .text_size(px(16. * scale))
            .child("✕")
            .on_click(cx.listener(|this, _, _, cx| this.close_card(cx)));

        let rows = card.rows.iter().map(|(label, value)| {
            h_flex()
                .w_full()
                .justify_between()
                .gap(px(16. * scale))
                .text_size(px(13. * scale))
                .child(div().text_color(rgb(MUTED)).child(label.clone()))
                .child(
                    div()
                        .text_color(rgb(TEXT))
                        .text_ellipsis()
                        .child(value.clone()),
                )
        });

        // iOS 风格的黑色长条「关闭」：紧跟在「发消息」按钮下面，
        // 白字居中，点一下关掉名片。
        let close_bar = div()
            .id("card-close-bar")
            .w(px(CARD_CLOSE_W * scale))
            .h(px(CARD_CLOSE_H * scale))
            .flex_shrink_0()
            .rounded(px(8. * scale))
            .cursor_pointer()
            .items_center()
            .justify_center()
            .bg(rgb(INK))
            .text_color(rgb(WHITE))
            .text_size(px(18. * scale))
            .child("关闭")
            .on_click(cx.listener(|this, _, _, cx| this.close_card(cx)));

        let panel = v_flex()
            .id("contact-card")
            .w(px(CARD_W * scale))
            .h(px(CARD_H * scale))
            // 窗口只有 800x600，原生标题栏会再吃掉几十像素：名片保持 500x600 的名义尺寸，
            // 但用上限兜底，避免在小窗口里被裁掉 ✕ 按钮和底部按钮。
            .max_w(relative(0.92))
            .max_h(relative(0.94))
            .p(px(28. * scale))
            .gap(px(16. * scale))
            .rounded(px(14. * scale))
            .bg(rgb(WHITE))
            .shadow_lg()
            .opacity(opacity)
            .on_mouse_down(MouseButton::Left, |_, _, _| {})
            .child(h_flex().w_full().justify_end().child(close_button))
            .child(h_flex().w_full().justify_center().child(remote_avatar(
                SharedString::from("card-avatar"),
                &card.avatar,
                model::initial_of(&card.name),
                96. * scale,
                24. * scale,
                TINT_CARD,
            )))
            .child(
                h_flex()
                    .w_full()
                    .justify_center()
                    .text_size(px(22. * scale))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(TEXT))
                    .child(card.name.clone()),
            )
            .when(!card.subtitle.is_empty(), |el| {
                el.child(
                    h_flex()
                        .w_full()
                        .justify_center()
                        .text_size(px(12. * scale))
                        .text_color(rgb(GREEN))
                        .child(card.subtitle.clone()),
                )
            })
            .child(
                div()
                    .w_full()
                    .my(px(8. * scale))
                    .border_t_1()
                    .border_color(rgb(LINE)),
            )
            .children(rows)
            .child(div().flex_1())
            .child(
                h_flex().w_full().justify_center().child(
                    div()
                        .id("card-send")
                        .px(px(32. * scale))
                        .py(px(12. * scale))
                        .rounded(px(6. * scale))
                        .bg(rgb(GREEN))
                        .text_color(rgb(WHITE))
                        .text_size(px(13. * scale))
                        .cursor_pointer()
                        .child("发消息")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_card(cx);
                            this.section = 0;
                            cx.notify();
                        })),
                ),
            )
            .child(close_bar);

        div()
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .size_full()
            .child(backdrop)
            .child(
                h_flex()
                    .absolute()
                    .top(px(0.))
                    .left(px(0.))
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child(panel),
            )
    }
}

impl Render for Messenger {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let chat = self.chats.get(self.selected).cloned().unwrap_or_default();
        let title = match self.section {
            0 => "消息",
            1 => "联系人",
            2 => "动态",
            3 => "视频",
            4 => "商城",
            5 => "我的",
            _ => "设置",
        };
        let is_chat = self.section == 0;
        let is_contacts = self.section == 1;

        // `.env` 里 FPS_DISPLAY_MODE=1 时绘制左上角帧率表：每帧继续请求下一帧，等于
        // 满负载测量；同时记录窗口是否处于激活状态。
        if self.fps_overlay {
            self.fps_frames += 1;
            let elapsed = self.fps_since.elapsed();
            if elapsed >= FPS_SAMPLE_WINDOW {
                self.fps = (self.fps_frames as f32 / elapsed.as_secs_f32()).round() as u32;
                self.fps_frames = 0;
                self.fps_since = Instant::now();
            }
            self.window_active = window.is_window_active();
            window.request_animation_frame();
        }
        let fps = self.fps;
        let window_active = self.window_active;

        // 聊天气泡区的滚动：
        // 1) 切会话后先把位置恢复成目标会话上次的位置（一次性，不滚到底）；
        // 2) 有新消息时把偏移平滑推到最新的底部，走满 CHAT_SCROLL_ANIM（300ms），
        //    同样是这一帧里请求下一帧。
        self.apply_chat_scroll_restore();
        self.advance_chat_scroll(window, cx);

        // 中间列表：联系人页读通信录，其余页面保持原来的会话列表。
        let list = if is_contacts {
            self.contacts_list(cx)
        } else {
            self.chat_list(cx)
        };
        // 名片弹窗 + 开关动画：打开 0.9 → 1（缩放）/ 0 → 1（不透明度），关闭反向，
        // 各 200ms；动画期间由 `advance_card_animation` 持续请求下一帧。
        let card_overlay = self
            .advance_card_animation(window, cx)
            .map(|(card, scale, opacity)| self.card_overlay(card, scale, opacity, cx));

        v_flex()
            .size_full()
            .bg(rgb(WHITE))
            .child(
                h_flex()
                    .w_full()
                    .h_9()
                    .px_4()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_color(rgb(GREEN)).text_size(px(16.)).child("●"))
                            .child("可乐 · 桌面版"),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(MUTED))
                            .child("微信风格即时通讯"),
                    ),
            )
            .child(
                h_flex()
                    .size_full()
                    .overflow_hidden()
                    // 左侧导航
                    .child(
                        v_flex()
                            .w(px(60.))
                            .h_full()
                            .items_center()
                            .pt_4()
                            .px_1()
                            .bg(rgb(0xf5f5f5))
                            .border_r_1()
                            .border_color(rgb(LINE))
                            .child(
                                div()
                                    .size_10()
                                    .mb_5()
                                    .rounded_lg()
                                    .bg(rgb(GREEN))
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(WHITE))
                                    .text_size(px(22.))
                                    .child("可"),
                            )
                            .child(self.nav_item(0, "◉", "消息", cx))
                            .child(self.nav_item(1, "♙", "联系人", cx))
                            .child(self.nav_item(2, "◈", "动态", cx))
                            .child(self.nav_item(3, "▷", "视频", cx))
                            .child(self.nav_item(4, "▤", "商城", cx))
                            .child(self.nav_item(5, "☺", "我的", cx))
                            .child(div().flex_1())
                            .child(self.nav_item(6, "⚙", "设置", cx)),
                    )
                    // 中间列表（会话 / 通信录）
                    .child(
                        v_flex()
                            .w(px(240.))
                            .max_w(px(600.))
                            .h_full()
                            .bg(rgb(PANEL))
                            .border_r_1()
                            .border_color(rgb(LINE))
                            .child(
                                h_flex()
                                    .h(px(56.))
                                    .px_4()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_size(px(18.))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(TEXT))
                                            .child(if is_chat { "微信" } else { title }),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_3()
                                            .items_center()
                                            .text_size(px(18.))
                                            .text_color(rgb(0x666666))
                                            .child("⌕")
                                            .child(
                                                div()
                                                    .id("new-chat")
                                                    .cursor_pointer()
                                                    .text_color(rgb(GREEN))
                                                    .child("＋")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.notice("发起新聊天", cx)
                                                    })),
                                            ),
                                    ),
                            )
                            .child(list),
                    )
                    // 会话主区
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w(px(260.))
                            .h_full()
                            .bg(rgb(WHITE))
                            .child(
                                h_flex()
                                    .h(px(56.))
                                    .px_6()
                                    .items_center()
                                    .justify_between()
                                    .border_b_1()
                                    .border_color(rgb(LINE))
                                    // 标题 = 24x24 圆形头像 + 名称；点击弹出名片
                                    .child(
                                        h_flex()
                                            .id("conversation-title")
                                            .cursor_pointer()
                                            .items_center()
                                            .gap_2()
                                            .when(is_chat, |el| {
                                                el.child(remote_avatar(
                                                    SharedString::from(format!(
                                                        "header-avatar-{}",
                                                        chat.id
                                                    )),
                                                    &chat.avatar,
                                                    model::initial_of(&chat.name),
                                                    24.,
                                                    12.,
                                                    TINT_CHAT,
                                                ))
                                            })
                                            .child(
                                                div()
                                                    .text_size(px(16.))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(TEXT))
                                                    .child(if is_chat {
                                                        chat.name.clone()
                                                    } else {
                                                        title.to_string()
                                                    }),
                                            )
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let card = this
                                                    .chats
                                                    .get(this.selected)
                                                    .map(CardInfo::from_chat);
                                                if let Some(card) = card {
                                                    this.open_card(card, cx);
                                                }
                                            })),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_5()
                                            .text_size(px(18.))
                                            .text_color(rgb(0x777777))
                                            .child("⌕")
                                            .child("⋯"),
                                    ),
                            )
                            .child(if is_chat {
                                v_flex()
                                    .id("conversation-messages")
                                    .flex_1()
                                    .overflow_y_scroll()
                                    // 把滚动状态交给 `Messenger::chat_scroll`：
                                    // `advance_chat_scroll()` 读它的偏移、写它的偏移来实现平滑滚动。
                                    .track_scroll(&self.chat_scroll)
                                    .px_6()
                                    .py_6()
                                    .children(
                                        chat.messages.iter().enumerate().map(|(index, message)| {
                                            self.bubble(message, index, cx)
                                        }),
                                    )
                                    .into_any_element()
                            } else {
                                v_flex()
                                    .id("conversation-empty")
                                    .flex_1()
                                    .items_center()
                                    .justify_center()
                                    .gap_3()
                                    .text_color(rgb(MUTED))
                                    .child(div().text_size(px(28.)).text_color(rgb(GREEN)).child(
                                        match self.section {
                                            1 => "♙",
                                            2 => "◈",
                                            3 => "▷",
                                            4 => "▤",
                                            5 => "☺",
                                            _ => "⚙",
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_size(px(16.))
                                            .child(format!("{}功能正在开发中", title)),
                                    )
                                    .child(div().text_size(px(12.)).child("返回消息即可继续聊天"))
                                    .into_any_element()
                            })
                            .child(
                                v_flex()
                                    .h(px(145.))
                                    .px_5()
                                    .pb_3()
                                    .border_t_1()
                                    .border_color(rgb(LINE))
                                    .child(
                                        h_flex()
                                            .h_10()
                                            .items_center()
                                            .gap_4()
                                            .child(
                                                self.toolbar_button("emoji", "☺", "表情面板", cx)
                                                    .when(self.show_emoji, |el| {
                                                        el.bg(rgb(0xe8eee3))
                                                    })
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.show_emoji = !this.show_emoji;
                                                        this.notice("表情：😀  😄  😊  ❤️  👍", cx);
                                                    })),
                                            )
                                            .child(
                                                // 照片：打开系统文件对话框，选中的图片按附件发送。
                                                self.toolbar_icon("photo", "▧").on_click(
                                                    cx.listener(|this, _, window, cx| {
                                                        this.pick_media(MediaKind::Image, window, cx)
                                                    }),
                                                ),
                                            )
                                            .child(
                                                self.toolbar_icon("video", "▣").on_click(
                                                    cx.listener(|this, _, window, cx| {
                                                        this.pick_media(MediaKind::Video, window, cx)
                                                    }),
                                                ),
                                            )
                                            .child(
                                                self.toolbar_icon("file", "▤").on_click(
                                                    cx.listener(|this, _, window, cx| {
                                                        this.pick_media(MediaKind::File, window, cx)
                                                    }),
                                                ),
                                            )
                                            .child(self.toolbar_button(
                                                "location",
                                                "⌖",
                                                "发送位置",
                                                cx,
                                            ))
                                            .child(self.toolbar_button(
                                                "redpacket",
                                                "◇",
                                                "发红包",
                                                cx,
                                            ))
                                            .child(self.toolbar_button(
                                                "contact",
                                                "♙",
                                                "发送名片",
                                                cx,
                                            )),
                                    )
                                    .child(
                                        // 消息输入框：`gpui-component` 的 `Input`，中文输入法 / 粘贴 /
                                        // 选区 / 光标都由 GPUI 的输入法处理器接管；Enter 发送见
                                        // `Messenger::new()` 里对 `InputEvent::PressEnter` 的订阅。
                                        div()
                                            .id("message-input")
                                            .flex_1()
                                            .min_h(px(42.))
                                            .max_h(px(68.))
                                            .py_2()
                                            .child(
                                                Input::new(&self.input)
                                                    .appearance(false)
                                                    .bordered(false)
                                                    .focus_bordered(false)
                                                    .w_full(),
                                            ),
                                    )
                                    .child(
                                        h_flex().justify_end().child(
                                            div()
                                                .id("send-button")
                                                .px_5()
                                                .py_2()
                                                .rounded_md()
                                                .bg(rgb(GREEN))
                                                .text_color(rgb(WHITE))
                                                .text_size(px(13.))
                                                .cursor_pointer()
                                                .child("发送")
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.send(window, cx)
                                                })),
                                        ),
                                    ),
                            ),
                    )
                    .when(self.toast.is_some(), |el| {
                        el.child(
                            div()
                                .absolute()
                                .bottom(px(160.))
                                .left_1_2()
                                .px_4()
                                .py_2()
                                .rounded_md()
                                .bg(rgb(0x333333))
                                .text_color(rgb(WHITE))
                                .text_size(px(12.))
                                .child(self.toast.clone().unwrap_or_default()),
                        )
                    }),
            )
            .when_some(card_overlay, |el, overlay| el.child(overlay))
            .when(self.fps_overlay, move |el| {
                el.child(
                    h_flex()
                        .absolute()
                        .top(px(4.))
                        .left(px(8.))
                        .gap_2()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(rgb(0x000000))
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(rgb(fps_color(fps)))
                                .child(format!("{fps} FPS")),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(rgb(0x888888))
                                .child(format!("active={window_active}")),
                        ),
                )
            })
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);
    app.run(|cx| {
        gpui_kit::init(cx);
        // GPUI 自带的 HTTP 客户端是空实现（每个请求都以 "No HttpClient available"
        // 失败），而头像走的是 `img("https://…")`，所以必须先换上真正的客户端。
        cx.set_http_client(Arc::new(net::HttpClient::new()));
        // 使用操作系统的原生标题栏（server-side decorations）：`appears_transparent:
        // false` 会让 GPUI 保留系统标题栏，拖动交给 Windows 自己处理。
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            titlebar: Some(TitlebarOptions {
                title: Some("可乐IM · 桌面版".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        };

        //

        info!("[🗣️ MAIN] - ✅️ Cola IM 客户端启动");

        //
        gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| Messenger::new(window, cx))
        })
        .expect("[🗣️ MAIN] - ✅️  - ❌️ Failed to open Cola IM window");
    });
}

#[cfg(test)]
mod tests {
    use super::{
        CARD_ANIM, CARD_ANIM_SCALE_FROM, CardPhase, CardView, ChatScrollOffsets, fps_color,
        restored_scroll_offset, scroll_offset_at,
    };
    use crate::model::CardInfo;
    use gpui_kit::ease_in_out;
    use std::time::Instant;

    //////// 名片弹窗的开关动画

    fn card_view(phase: CardPhase) -> CardView {
        CardView {
            card: CardInfo::default(),
            phase,
        }
    }

    /// 打开：缩放 0.9 → 1、不透明度 0 → 1，正好 200ms。
    #[test]
    fn card_opens_from_90_percent_and_fades_in() {
        let since = Instant::now();
        let view = card_view(CardPhase::Opening(since));

        let (scale, opacity) = view.visual(since);
        assert!((scale - CARD_ANIM_SCALE_FROM).abs() < 0.0001, "起点缩放 {scale}");
        assert!(opacity.abs() < 0.0001, "起点不透明度 {opacity}");
        assert!(view.progress(since).1, "刚打开时应该是动画中");

        let (t, animating) = view.progress(since + CARD_ANIM / 2);
        assert!(animating);
        assert!((t - 0.5).abs() < 0.0001, "中点进度 {t}"); // ease_in_out(0.5) = 0.5
        let (scale, opacity) = view.visual(since + CARD_ANIM / 2);
        assert!((scale - 0.95).abs() < 0.0001, "中点缩放 {scale}");
        assert!((opacity - 0.5).abs() < 0.0001, "中点不透明度 {opacity}");

        // 200ms 之后停在终点，不再请求下一帧。
        let (scale, opacity) = view.visual(since + CARD_ANIM);
        assert!((scale - 1.0).abs() < 0.0001, "终点缩放 {scale}");
        assert!((opacity - 1.0).abs() < 0.0001, "终点不透明度 {opacity}");
        assert!(!view.progress(since + CARD_ANIM).1);
        assert!(view.done(since + CARD_ANIM));
    }

    /// 关闭就是打开的反向：进度从 1 走到 0。
    #[test]
    fn card_closes_as_the_reverse_of_opening() {
        let since = Instant::now();
        let view = card_view(CardPhase::Closing(since));

        let (scale, opacity) = view.visual(since);
        assert!((scale - 1.0).abs() < 0.0001, "关闭起点缩放 {scale}");
        assert!((opacity - 1.0).abs() < 0.0001, "关闭起点不透明度 {opacity}");

        let (t, animating) = view.progress(since + CARD_ANIM / 2);
        assert!(animating);
        assert!((t - 0.5).abs() < 0.0001, "关闭中点进度 {t}");

        let (scale, opacity) = view.visual(since + CARD_ANIM);
        assert!((scale - CARD_ANIM_SCALE_FROM).abs() < 0.0001, "关闭终点缩放 {scale}");
        assert!(opacity.abs() < 0.0001, "关闭终点不透明度 {opacity}");
        assert!(view.done(since + CARD_ANIM), "关闭动画走完应该被移除");
    }

    /// 缩放 / 不透明度始终落在 0.9..1 与 0..1 之间（缓动不会过冲）。
    #[test]
    fn card_animation_stays_inside_the_configured_range() {
        let since = Instant::now();
        for view in [
            card_view(CardPhase::Opening(since)),
            card_view(CardPhase::Closing(since)),
            card_view(CardPhase::Open),
        ] {
            for step in 0..=10 {
                let now = since + CARD_ANIM * step / 10;
                let (scale, opacity) = view.visual(now);
                assert!(
                    (CARD_ANIM_SCALE_FROM..=1.0).contains(&scale),
                    "缩放越界 {scale}"
                );
                assert!((0.0..=1.0).contains(&opacity), "不透明度越界 {opacity}");
            }
        }
    }

    /// 已打开的状态是静止的：没有「播完」一说，也不会再请求下一帧。
    #[test]
    fn an_open_card_is_at_rest() {
        let view = card_view(CardPhase::Open);
        let (t, animating) = view.progress(Instant::now());
        assert!((t - 1.0).abs() < 0.0001);
        assert!(!animating);
        assert!(!view.done(Instant::now()));
    }

    //////// 聊天气泡区的自动滚动

    /// 切会话恢复位置：记录过就用记录值；**没记录过从顶部开始**（不主动滚到底部）。
    #[test]
    fn switching_back_restores_the_remembered_position() {
        let mut offsets = ChatScrollOffsets::new();
        assert_eq!(restored_scroll_offset(&offsets, 3), 0.0);

        // 切走时记下的实时偏移。
        offsets.insert(3, -420.5);
        assert_eq!(restored_scroll_offset(&offsets, 3), -420.5);
        // 其它会话互不影响。
        assert_eq!(restored_scroll_offset(&offsets, 4), 0.0);

        // 再滚一次会覆盖旧记录（记住的是最后一次位置）。
        offsets.insert(3, -80.0);
        assert_eq!(restored_scroll_offset(&offsets, 3), -80.0);
    }

    /// 插值：进度 0 → 起点，1 → 底部，0.5 → 中间；从中途开始也能接上。
    #[test]
    fn scroll_offsets_interpolate_from_top_to_bottom() {
        let bottom = -1000.0;
        assert_eq!(scroll_offset_at(0.0, bottom, 0.0), 0.0);
        assert_eq!(scroll_offset_at(0.0, bottom, 0.5), -500.0);
        assert_eq!(scroll_offset_at(0.0, bottom, 1.0), bottom);
        // 上一次动画没播完就来了新消息：从当前偏移继续往底部走。
        assert_eq!(scroll_offset_at(-200.0, bottom, 0.5), -600.0);
    }

    /// 两端必须精确落在起点 / 底部（`ease_in_out` 的边界值不能有误差）。
    #[test]
    fn scroll_animation_endpoints_are_exact() {
        assert_eq!(scroll_offset_at(-42.0, -999.0, ease_in_out(0.0)), -42.0);
        assert_eq!(scroll_offset_at(-42.0, -999.0, ease_in_out(1.0)), -999.0);
    }

    /// 偏移只会一路往底部走（负得更多），不会越过目标，也不会往回弹。
    #[test]
    fn scroll_offsets_never_overshoot_the_bottom() {
        let (from, bottom) = (-100.0, -1000.0);
        let mut previous = from;
        for step in 0..=10 {
            let y = scroll_offset_at(from, bottom, ease_in_out(step as f32 / 10.0));
            assert!(y <= previous + 0.001, "第 {step} 步往回弹了：{previous} → {y}");
            assert!(y >= bottom - 0.001, "第 {step} 步越过底部：{y} < {bottom}");
            previous = y;
        }
    }

    //////// FPS 帧率表

    /// `.env` 里 FPS_DISPLAY_MODE=1 时的分档配色：>100 绿、60-100 黄、40-60 橙、<40 红。
    #[test]
    fn fps_colors_follow_the_requested_thresholds() {
        assert_eq!(fps_color(144), 0x00ff00);
        assert_eq!(fps_color(101), 0x00ff00);
        assert_eq!(fps_color(100), 0xffff00);
        assert_eq!(fps_color(60), 0xffff00);
        assert_eq!(fps_color(59), 0xff6600);
        assert_eq!(fps_color(40), 0xff6600);
        assert_eq!(fps_color(39), 0xff0000);
        assert_eq!(fps_color(0), 0xff0000);
    }
}

