# 开发文档 · Cola IM Desktop Client

一个基于 `gpui-kit`（`gpui-pre` / GPUI + `gpui-component`）的桌面 IM 客户端 Demo。

---

## 1. 环境与运行

| 项 | 值 |
| --- | --- |
| 平台 | Windows |
| 语言 | Rust，edition 2024 |
| 工程结构 | Cargo **工作区**：根目录只放 virtual manifest，二进制包在 `client/` |
| UI 依赖 | `gpui-kit = "0.7.3"` |
| 网络 | `reqwest`（`gpui-pre-reqwest` 分支）+ `tokio`，用于 `img("https://…")` 头像 |
| 数据 | `serde` / `serde_json`，读取 `client/src/data/im/*.json` |

### 1.1 目录结构

```text
cola-im-desktop-client/          # 仓库根 = Cargo 工作区根
├── Cargo.toml                   # [workspace] members = ["client"]（virtual manifest）
├── Cargo.lock                   # 工作区共享（唯一一份）
├── .env                         # 运行期配置，留在仓库根（查找顺序见第 5 节）
├── DEVELOPMENT.md
└── client/                      # 唯一的二进制包
    ├── Cargo.toml               # [package] name = "client"
    └── src/
        ├── main.rs              # 界面（Messenger / Render）
        ├── config.rs            # .env 解析
        ├── model.rs             # mock JSON 数据模型
        ├── media.rs             # 附件与缩略图尺寸
        ├── net.rs               # GPUI 用的 HTTP 客户端
        ├── upload.rs            # 预留的 S3 异步上传
        └── data/im/*.json       # mock 数据（include_str! 编译进二进制）
```

> 产物统一在仓库根的 `target/`：`target/debug/cola-im-desktop-client.exe`、
> `target/release/cola-im-desktop-client.exe`。
> （包名是 `client`，**二进制名保留历史名字 `cola-im-desktop-client`**，见 `client/Cargo.toml`
> 里的 `[[bin]]`——这样 IDE 的运行配置、旧脚本、产物路径都不用改。）

### 1.2 命令

```powershell
# 开发运行（注意：性能很差，见第 2 节）
cargo run -p client

# 运行（推荐）
cargo run --release -p client

# 只编译
cargo build -p client           # debug
cargo build --release -p client # release（首次约 15 分钟，之后增量很快）

# 单元测试（含一个默认忽略的真实网络请求，见第 4 节）
cargo test -p client
cargo test -p client -- --include-ignored

# 在 client/ 目录里也可以直接跑（工作区会自动向上找到根 Cargo.toml）
cd client; cargo run
```

> 工作区只有一个成员，所以 `-p client` 可以省略（`cargo run` 等价于 `cargo run -p client`）；
> 二进制名是 `cola-im-desktop-client`，所以 `cargo run --bin cola-im-desktop-client`、IDE 里
> 指向旧名字的运行配置也照样能跑。

运行时配置放在项目根目录的 `.env`（启动时读取，改完重启即可生效）：

```ini
# 1 = 打开左上角实时帧率表（配色见第 5 节），0 = 关闭
FPS_DISPLAY_MODE = 1
```


> **重要：日常调试 UI 逻辑可以用 debug，但任何涉及「流畅度 / 帧率 / 拖动窗口」的判断，必须用 `--release`。**
> debug 与 release 的帧率差距在本项目里非常大（实测 37fps vs 91fps，见下）。

---

## 2. Windows 拖动窗口掉帧（20fps）的调查结论

### 2.1 现象

拖动本应用窗口时大约只有 20fps，一帧一帧的、不跟手；而拖动系统里其它应用窗口都是 100+fps、很丝滑。

### 2.2 根因

**GPUI 在窗口每次移动时，会把整棵元素树整帧重绘一次。** 调用链：

```
WM_MOVE
  → gpui-pre-windows 触发 on_moved 回调
  → gpui-pre/src/window.rs  Window::bounds_changed()
  → gpui-pre/src/window.rs  Window::refresh()
  → window.invalidator.set_dirty(true)          // 整窗标记为脏
  → 下一帧重新执行 render() 构建整棵树 + 布局 + 绘制
```

所以拖动窗口时的帧率 ≈ **一次完整重绘的耗时**。而本项目 UI 较大（会话列表、消息气泡、圆角/边框、中文文本等），一次完整重绘在：

| 构建 | 一次完整重绘耗时 | 满负载整窗重绘 FPS（实测） |
| --- | --- | --- |
| `cargo build`（debug） | ≈ 27ms | **37 fps** |
| `cargo build --release` | ≈ 11ms | **91 fps** |

debug 下 27ms/帧 → 拖动时约 20fps；换成 release 后提升到 90fps 以上。

> 也就是说：**这不是代码 bug，而是拿 debug 构建跑出来的结果。**

### 2.3 已排除的其它可能

调查过程中逐一验证过、可以排除：

1. **“非激活窗口节流”**：GPUI 有非激活帧率限制（默认 `inactive_frame_interval = 33.3ms`），但触发条件是 `!active && !input_high_rate`。
   实测 FPS 面板显示 `active=true`，且拖动时输入频率很高 → **未触发节流**。
   相关代码：`gpui-pre/src/window.rs` 中 `min_frame_interval` 的计算与 `InputRateTracker`。
2. **DirectComposition / `WS_EX_NOREDIRECTIONBITMAP`**：用 `GPUI_DISABLE_DIRECT_COMPOSITION=1` 去掉该样式后帧率反而更低（22fps）→ **不是它**。
3. **wgpu `PresentMode`（`Fifo`/`AutoNoVsync`/`Mailbox`）**：**不适用**。当前后端是 GPUI 自带的 DirectX，不是 wgpu；平台层也不是 winit。
   - `gpui-pre-windows/src/directx_renderer.rs:251` → `swap_chain.Present(0, DXGI_PRESENT(0))`，**SyncInterval = 0，本来就不等垂直同步**。
   - 真正的垂直同步是另起 `VSyncProvider` 线程循环调用 `DwmFlush()`（`gpui-pre-windows/src/vsync.rs`），每帧再 `RedrawWindow(RDW_INVALIDATE)`。
   - 移动窗口期间走的是 `SetTimer(hwnd, SIZE_MOVE_LOOP_TIMER_ID, USER_TIMER_MINIMUM=10ms)` + `WM_TIMER`/`WM_PAINT`（`gpui-pre-windows/src/events.rs`）。
   - 想走 wgpu 路线只能切换到 `gpui-kit` 的 `gpui-fast` 特性（内含 `gpui-fast-wgpu`），那是另一套实现。

### 2.4 结论与建议

- **用 `cargo run --release`** 即可解决。
- 如果将来 release 下仍嫌不够流畅，唯一方向是**降低每帧重绘成本**（因为 GPUI 每次窗口移动都会重绘整棵树）：精简元素树、减少圆角/边框/阴影等重绘制元素、把不变的部分做成缓存组件等。

---

## 3. 数据来源：`client/src/data/im/*.json`

会话列表与通信录都读本地 mock JSON。`client/src/model.rs` 用 `include_str!` 把它们编译进二进制，
所以运行时不依赖启动目录：

| 文件 | 结构 | 用在哪 |
| --- | --- | --- |
| `chat_list.json` | `[{ id, name, lastMessage, unread, avatar, messages[] }]` | 左侧会话列表、会话标题、名片弹窗 |
| `contacts_list.json` | `[{ group, contacts: [{ id, name, initial, phone, avatar, signature }] }]` | 「联系人」页 |

- 除 `id` / `name` / `group` 外所有字段都是可选的：JSON 里补上 `time`、`details` 会自动显示，
  缺字段就留空，不会 panic。
- `messages` 目前是空数组，`model::demo_messages()` 给前两个会话补了演示对话，免得聊天区空白；
  把 `messages` 写进 JSON 就会覆盖它。
- 通信录按 JSON 顺序分组渲染，同名分组（示例数据里有两个 `B`、两个 `W`）也能正常显示。

## 4. 网络图片与 HTTP 客户端

头像是网络图片，走 GPUI 的资源系统：

```
img("https://…")  →  Resource::Uri  →  AssetLogger<ImageAssetLoader>
                  →  App::http_client().get(uri)   ← 默认是空实现！
```

GPUI 默认装的是 `NullHttpClient`（任何请求都返回 “No HttpClient available”），不换掉它，
头像永远是空白。`main()` 里在 `init(cx)` 之后立刻替换：

```rust
cx.set_http_client(std::sync::Arc::new(net::HttpClient::new()));
```

`client/src/net.rs` 的实现要点：

- 用 `reqwest`（GPUI 生态里的 `gpui-pre-reqwest` 分支）并只开 **`native-tls`**：Windows 上走
  SChannel，不需要 cmake / NASM / OpenSSL，构建里没有 `rustls` / `aws-lc-sys`
  （`gpui-pre-reqwest-client` 那套官方适配器默认启用 rustls，会要求 cmake+NASM，所以没用它）。
- 每个请求开一个短命名的 `cola-net` 工作线程，线程内建一个单线程 tokio 运行时，
  **`runtime.enter()` 之后再构造 reqwest Client**（否则 reqwest 会 panic
  “there is no reactor running”），请求结束后运行时与连接池一起销毁。
  → 不占主线程，也不会饿死 GPUI 的后台执行器（图像加载器正占用它）。
- 结果通过 `futures::oneshot` 送回渲染线程上的 future。
- 失败不崩：`img()` 的 `with_fallback()` 渲染首字母色块，`with_loading()` 显示灰底占位。

验证链路（会真的下载一张 CDN 头像并断言 HTTP 200）：

```powershell
cargo test --release -- --ignored   # net::tests::downloads_a_remote_avatar
```

## 5. FPS 面板（诊断工具）

帧率表由项目根目录的 `.env` 控制（读取逻辑在 `client/src/config.rs`），默认关闭：

```ini
FPS_DISPLAY_MODE = 1   # 1 / true / on / yes 打开，其余关闭
```

打开后窗口左上角会显示一个黑色小胶囊：

```
144 FPS · active=true
```

- `FPS`：最近 0.5 秒的实际帧数（每帧 `request_animation_frame()` 持续驱动，等于满负载测量）。
  颜色按档位变化：**>100 → `#00ff00`、60–100 → `#ffff00`、40–60 → `#ff6600`、<40 → `#ff0000`**
  （`fps_color()`，单元测试覆盖了边界值 101/100/60/59/40/39）。
- `active`：`Window::is_window_active()`，即 GPUI 是否认为窗口处于激活状态——用来排查“非激活节流”。

实现位置（`client/src/main.rs`）：字段 `Messenger::{fps_overlay, fps_frames, fps_since, fps, window_active}`、
`Messenger::new()` 里的 `config::fps_display_enabled()`、`render()` 开头的计数、整棵树末尾的
`.when(self.fps_overlay, …)` 绘制。

> **开关语义**：`FPS_DISPLAY_MODE=1` 显示、`=0`（或其它值）不显示，debug / release 完全一样。
> `.env` 查找顺序：当前工作目录 → 可执行文件所在目录 → `client/`（crate 根）→
> **仓库根（工作区根，编译期烧进去的 `CARGO_MANIFEST_DIR/..`）**
> （所以 `cargo run -p client`、双击 `target\release\cola-im-desktop-client.exe`、从任意目录启动，都能读到
> 仓库根目录的 `.env`）；同名环境变量优先于文件内容（兼容旧的 `COLA_FPS_OVERLAY=1`）。
> 启动时会把结论打到 stderr，例如：
>
> ```text
> [cola] FPS 帧率表：开（D:\rust\cola-im-desktop-client\.env 中的 FPS_DISPLAY_MODE=1）
> ```

---

## 6. 窗口标题栏：原生 vs 自绘

当前使用**操作系统原生标题栏**（server-side decorations）。关键点在 `client/src/main.rs` 的 `main()`：

```rust
titlebar: Some(TitlebarOptions {
    title: Some("可乐 · 桌面版".into()),
    appears_transparent: false, // ← 关键
    traffic_light_position: None,
}),
..Default::default()
```

- GPUI 在 Windows 上根据 `titlebar.appears_transparent` 决定 `hide_title_bar`：
  - `false` → `hide_title_bar = false` → `WM_NCCALCSIZE` 交还系统默认处理 → **系统绘制原生标题栏**；
  - `true`（以及 `titlebar: None`）→ `hide_title_bar = true` → 无边框窗口，需应用自绘标题栏。
- 因此不再使用 `gpui-component` 的 `TitleBar` 组件（它会注册 `WindowControlArea::Drag`，用于自绘标题栏的拖动）。原来的 `TitleBar::new().child(...)` 已降级为普通的一行 `h_flex()` 头部。
- 若想改回自绘标题栏（例如要更贴近微信的视觉），把这里改回 `..TitleBar::window_options()` 并恢复 `TitleBar` 组件即可；但注意自绘标题栏的拖动会走 Windows 系统模态移动循环，重绘路径与原生标题栏不同。

---

## 7. 界面要点

- 默认窗口 **800x600**：`main()` 里 `WindowBounds::centered(size(px(800.), px(600.)), cx)`。
- 会话列表的一行必须是 `h_flex()`：普通 `div()` 会让子元素纵向堆叠，这就是之前「名字没在头像
  右边、上下错位」的原因（见 `chat_item()`）。
- 会话标题栏（`conversation-title`）= **24x24 圆形头像**（`rounded(px(12.))`）+ 名称，点击弹出名片。
- 消息气泡（`bubble()`）：对方的消息是「头像 + 气泡」，自己发的反过来 —— 气泡在左、头像在右，
  整行用 `justify_end()` 靠右。
- 会话列表未读角标底色 `#ff0000`。
- 名片弹窗（`card_overlay()`）：整窗半透明遮罩 + **500x600** 卡片；卡片内自上而下为 ✕、头像、
  名称、签名、信息行、**「发消息」按钮**，以及紧跟其下的 iOS 风格黑色「关闭」长条
  （**400x45**、底色 `#0a0a0a`、白字 18px 居中）。三种关闭方式：点遮罩、右上角 ✕、点黑色长条。
  卡片自身带一个空的鼠标监听，避免点卡片内部时穿透到遮罩被误关；窗口高度不足 600 时卡片按
  `max_h(relative(0.94))` 收缩，保证 ✕ 与底部长条不被裁掉。
- **名片弹窗开关动画**（`CardView` / `Messenger::advance_card_animation`）：打开时缩放
  `0.9 → 1`、不透明度 `0 → 1`，关闭时同一条缓动曲线**反向**（`1 → 0.9` / `1 → 0`），
  两边都是 **200ms**，卡片始终居中（缩放围绕中心）。
  - GPUI 这一版没有元素级 `transform: scale`，所以「zoom」是把卡片的**每一处尺寸**
    （宽高 / 内边距 / 间距 / 字号 / 头像）都乘上缩放系数来实现的，见 `card_overlay()`。
  - 缓动用 GPUI 自带的 `ease_in_out`（`gpui::ease_in_out`），不手写曲线。
  - 状态机：`Opening(t) → Open`、`Closing(t) → 移除`；关闭动画走完才把 `self.card` 置空，
    所以关闭时不会「啪」地消失。动画期间每帧 `window.request_animation_frame()`。
  - 系统开了「减少动态效果」（`cx.reduce_motion()`）时直接跳到终态，不播动画。
- 通信录页（`contacts_list()`）按 JSON 的 `group` 分组渲染，点击联系人也弹名片（内容来自联系人
  的 `phone` / `initial` / `signature`）。
- 头像统一用 `remote_avatar()`：网络图 + 加载态 + 首字母兜底。

## 8. 聊天输入与附件（照片 / 视频 / 文件）

### 8.1 输入框

- 用的是 `gpui-component` 的 `Input` + `InputState`（不再是 `key.len() == 1` 的极简实现），
  所以**中文输入法、长文本、粘贴、选区、光标移动**都是 GPUI 输入法处理器的系统行为。
- `Messenger::new(window, cx)` 里创建 `InputState`，并用 `cx.subscribe_in(...)` 订阅
  `InputEvent::PressEnter` → `Messenger::send()`；订阅句柄存在 `_input_events`（丢掉订阅就失效）。
- 点「发送」按钮和按 Enter 走同一个 `send()`：读出 `InputState::value()` → `trim()` →
  推进 `chats[selected].messages`（**只在内存里**）→ `set_value("")` 清空。
- 渲染时 `Input::new(&self.input).appearance(false).bordered(false)`，外观还是原来那个 42–68 高的输入区。

### 8.2 附件

- 工具栏「照片 / 视频 / 文件」调用 `App::prompt_for_paths(PathPromptOptions { … })`
  （Windows 上是原生 `IFileOpenDialog`），选中的路径经 `cx.spawn_in` 异步回到渲染线程后
  交给 `Messenger::attach_files()`。GPUI 的选择器**不支持后缀过滤**，所以选完再按扩展名
  （`media::kind_of_path`）重新归类，点错按钮也能得到正确的类型。
- 气泡：照片 / 视频渲染本地文件缩略图，其它文件渲染「图标 + 文件名 + 大小」卡片；三种都能
  点一下用系统默认程序打开（`App::open_with_system`）。
- 消息内容（含附件路径、像素尺寸、上传状态）全部只存在内存里，重启即清空。

### 8.3 缩略图尺寸（`client/src/media.rs::thumbnail_size`）

规则：**边界 400x400、不裁切、等比缩小、只缩不放**。

```text
scale = min(1, 400/width, 400/height)
显示尺寸 = (width*scale, height*scale)   // 再 min(THUMB_MAX) 夹一次，消掉 f32 误差
```

- 例：`800x600 → 400x300`、`600x800 → 300x400`、`200x100 → 200x100`（不放大）；
- 渲染时用 `ObjectFit::Contain`，所以是「整张图缩进 400x400 的框」而不是裁切；
- 尺寸读不出来（视频没有抽帧能力 / 文件损坏）时回退成 400x400 的方块，气泡不会跳。
- 像素尺寸用 `image::image_dimensions()` 只读文件头，不解码整张图，放在渲染线程上也不卡。

### 8.4 预留：S3 异步上传（`client/src/upload.rs`）

```text
attach_files() → UploadRequest::for_path() → Arc<dyn S3Uploader>::upload()
                                                  ↓ 异步（cx.spawn，不阻塞渲染）
                                        finish_upload() 回写 UploadState
```

- `UploadState`：`Pending → Uploading → Uploaded { url } / Failed { reason }`，存在
  `Attachment::upload` 上；**当前界面不展示它**（接上 S3 后用 `is_uploaded()` / `url()` 加角标）。
- `StubUploader` 是占位实现，异步返回「尚未接入 S3」；接真机时新增一个实现并在
  `Messenger::new` 里换掉 `Arc<dyn S3Uploader>` 即可，界面代码不用动。
- 约定：`upload()` 必须纯异步（渲染在主线程上），`Ok(String)` 返回可直接写进消息体的对象 URL。

### 8.5 滚动：切会话记位置、新消息滚到底（300ms）

两条互不干扰的路径：

```text
点会话列表 → switch_chat(index)
   ├─ remember_chat_scroll()      记录旧会话的实时偏移（key = Chat::id）
   ├─ 取消旧会话没播完的「滚到底」动画
   └─ chat_scroll_restore = 目标会话记录过的偏移    ← 一次性恢复，不滚到底

send() / attach_files() → scroll_chat_to_bottom()
   └─ advance_chat_scroll()        300ms 平滑滚到底（下面这半段）
```

**位置记忆（切会话不滚到底）**

- `chat_scroll_offsets: ChatScrollOffsets`（= `HashMap<Chat::id, f32>`）：
  只在**切走**某个会话时写一次，读的是**实时**偏移，所以手动滚动、发消息后的位置都会被
  如实记住；切回来按记录恢复。
- **没记录过的会话从顶部（`0`）开始**，不主动滚到底 —— 滚到底只由新消息触发
  （`restored_scroll_offset()`，有单测）。
- 恢复是**一次性**的：`chat_scroll_restore` 在 `render()` 里 `take()` 掉、
  只写一帧。每帧都写会把用户的手动滚动顶掉。
- 恢复不用先算 `max_offset`：GPUI 在布局时会把偏移夹进 `[-max_offset, 0]`，
  记录的位置比新内容还长时自动落到底、也不会越界。
- 切会话时会顺手清掉 `chat_scroll_anim`，否则上一个会话没播完的动画会继续把新会话拉到底。

**新消息滚到底（300ms 动画）**

- 时间常量：`CHAT_SCROLL_ANIM = 300ms`（和 `CARD_ANIM = 200ms`、`FPS_SAMPLE_WINDOW = 500ms`
  一起放在 `main.rs` 顶部的「时间常量」段里，后期改时长只动这一处）。
- 缓动复用 GPUI 的 `ease_in_out`，插值在纯函数 `scroll_offset_at(from_y, target_y, eased)` 里，
  所以能单测（`scroll_offsets_*` / `scroll_animation_endpoints_are_exact`）。
- **目标每帧重新读一次** `ScrollHandle::max_offset()`：新消息是这一帧才进布局的，
  读一次的目标会偏小；每帧重读才能准确停在最新的底部（`t >= 1` 时直接吸附到底部）。
- GPUI 的滚动偏移约定是「**越往下越负**」（顶部 `0`、底部 `-max_offset`），所以目标取
  `-f32::from(handle.max_offset().y)`。
- `track_scroll` 会把 handle 的 offset `Rc` 与元素的滚动偏移**共享**（`div.rs:2274`），
  因此 `set_offset()` 写进去就是这一帧的滚动位置。
- 系统开了「减少动态效果」（`cx.reduce_motion()`）时不播动画，直接跳到底部。

---

## 9. 已知遗留 / 待办

- `window.prompt(...)` 返回的 `oneshot::Receiver` 直接被丢弃（`let _ =`），只用于演示系统弹窗。
- 会话列表 / 通信录没有虚拟滚动，条目多了会掉帧（GPUI 每次窗口移动都会整窗重绘，见第 2 节）。
- 拖动窗口的流畅度依赖「整帧重绘速度」，若元素树继续膨胀，release 下也可能再次变慢，届时需要做
  渲染裁剪/缓存。
- 视频缩略图没有抽帧能力（`image` 只认图片格式），现在显示的是带 ▶ 的兜底方块；
  需要真缩略图时要另外接 ffmpeg / Media Foundation。
- 附件目前只有本地路径，没有大小 / 数量上限，也没有「文件已删除」之外的失败处理。
