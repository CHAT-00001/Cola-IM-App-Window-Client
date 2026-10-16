# COLA IM Client Infra - 可乐IM 客户端 SDK核心
## 2026.5.1 v0.1.0

## DESC 
` 本SDK使用RUST开发, 分为网络 / 数据 / 存储 / 接口 几个部分. `
1. 网络层: 提供HTTP/HTTPS/GRPC/WS/WT多种方式, 根据不同业务域自动调用不同的网关参数连接网络, 网络有状态, 多端口自动跳跃等功能
2. 数据层: 提供JSON / Protobuf 等格式正反序列化, 加解密.
3. 存储层: 把数据通过SQLite存储在本地, 热数据直接以.json / .protof 的文件形式存储, 让SDK可以在500ms内给外部视图层快速填充数据,实现APP秒开
4. 接口层: 暴露给外部程序调用, 支持Android / iOS / Windows / Mac / Linux / Wasm, 因为iOS特殊性, 存储层可能会使用iOS自带的,不使用本SDK
`
## 

## 项目结构

```text
cola-client-core/
├─ Cargo.toml
├─ src/
│  ├─ lib.rs                  # 核心 SDK 入口
│  ├─ api/                    # 面向 UI 层的业务接口
│  ├─ client/                 # 客户端生命周期与会话管理
│  ├─ endpoint/               # 接入点发现、健康检查、择优与切换
│  ├─ transport/              # HTTP、WebSocket、WebTransport、gRPC
│  ├─ codec/                  # JSON、Protobuf 与 E2EE 数据处理
│  ├─ storage/                # SQLite、本地缓存与存储仓储
│  ├─ domain/                 # 与平台和协议无关的领域模型
│  └─ error/                  # SDK 统一错误模型
└─ crates/
   ├─ cola-client-ffi/        # Android、iOS、桌面端使用的 C API
   └─ cola-client-wasm/       # Web/WASM 与 JavaScript/TypeScript 适配层
```

### 核心库入口

`src/lib.rs` 是 `cola-client-core` 核心 crate 的统一入口，负责导出稳定的 SDK API。核心库内部封装网络、接入点调度、数据转换、端到端加密、SQLite 存储和消息同步等通用能力。

### 平台适配 crate

- `crates/cola-client-ffi/src/lib.rs`：提供稳定的 C ABI，供 Android JNI、iOS Swift/Objective-C、桌面端或其他原生语言调用。
- `crates/cola-client-wasm/src/lib.rs`：提供 Web/WASM 适配，供 JavaScript/TypeScript UI 使用，并适配浏览器网络和存储能力。

平台适配层只负责类型转换、生命周期、回调和平台能力接入，不复制核心网络、存储或业务逻辑。

### 领域模块与平台适配

当前采用“通用基础设施 + 领域模块 + 平台适配”的结构：

```text
cola-client-core/
├─ Cargo.toml
├─ src/
│  ├─ lib.rs                  # 仅负责模块声明和稳定导出
│  ├─ common/                 # 规划中的通用错误、配置、类型和编解码门面
│  ├─ gateway/                # Global Gateway 探测、刷新和 Endpoint 调度
│  ├─ transport/              # HTTP / WebSocket / WebTransport / gRPC
│  ├─ codec/                  # JSON / Protobuf / E2EE 数据转换
│  ├─ storage/                # SQLite、FlatBuffers 和缓存抽象
│  ├─ im/                     # IM 领域门面和稳定 API
│  │  ├─ mod.rs
│  │  ├─ api/                 # ContactsApi、MessagesApi 等 UI 接口
│  │  ├─ client/              # IM 客户端生命周期和事件
│  │  ├─ domain/              # 账号、联系人、消息和同步模型
│  │  ├─ event/               # IM 领域事件
│  │  └─ storage/             # IM 专属 Repository（规划）
│  ├─ short_video/            # 短视频领域预留，不依赖 IM API
│  ├─ dynamic/                # 动态/Feed 领域预留，不依赖 IM API
│  └─ legacy_video/           # 旧视频代码的后续隔离位置
└─ crates/
   ├─ cola_client_ffi/
   │  ├─ src/
   │  │  ├─ lib.rs            # C ABI 导出门面
   │  │  ├─ handle.rs         # 句柄边界
   │  │  ├─ error.rs          # C 错误码
   │  │  ├─ memory.rs         # 字符串和数组释放
   │  │  ├─ im.rs             # IM ABI
   │  │  ├─ contacts.rs       # 联系人 ABI
   │  │  └─ messages.rs       # 消息 ABI
   │  └─ include/
   │     └─ cola_client.h
   ├─ cola-client-uniffi/     # Swift / Kotlin 自动绑定（规划）
   └─ cola-client-wasm/       # Web/WASM 适配层
```

`src/im/mod.rs` 当前是兼容门面：它重新导出已有的 IM 实现，因此本次目录调整不会破坏现有 Rust 或 C API。后续会逐步把联系人、消息、事件和 IM 存储从旧的顶层文件迁移到 `src/im/`，并保持对外 API 稳定。

领域依赖边界：

```text
im / short_video / dynamic
        ↓
gateway / transport / codec / storage
        ↓
平台适配：C ABI / UniFFI / WASM
```

通用基础设施不能反向依赖具体业务领域；短视频、动态和 IM 之间也不能直接复制业务逻辑。

### 默认配置与运行时配置

根目录的 `application.toml` 只保存非敏感的默认配置，并通过 `src/config/mod.rs` 在编译期嵌入 SDK。Android/iOS 宿主提供的运行时配置优先级更高，真实 token、API Key、数据库密钥和 E2EE 密钥不得写入或嵌入 `application.toml`。

当前配置优先级：

```text
平台运行时注入配置
    > 编译期嵌入的 application.toml
    > Rust 最小兜底配置
```

运行时配置应由 Android Keystore、iOS Keychain 或其他平台安全存储提供，不能依赖移动端当前工作目录读取配置文件。



> [!IMPORTANT]
> **Rust port is now in progress** on the [`dev/rust`](https://github.com/instructkr/claw-code/tree/dev/rust) branch and is expected to be merged into main today. The Rust implementation aims to deliver a faster, memory-safe harness runtime. Stay tuned — this will be the definitive version of the project.

> If you find this work useful, consider [sponsoring @instructkr on GitHub](https://github.com/sponsors/instructkr) to support continued open-source harness engineering research.

---

## Rust Port

The Rust workspace under `rust/` is the current systems-language port of the project.

It currently includes:

- `crates/api-client` — API client with provider abstraction, OAuth, and streaming support
- `crates/runtime` — session state, compaction, MCP orchestration, prompt construction
- `crates/tools` — tool manifest definitions and execution framework
- `crates/commands` — slash commands, skills discovery, and config inspection
- `crates/plugins` — plugin model, hook pipeline, and bundled plugins
- `crates/compat-harness` — compatibility layer for upstream editor integration
- `crates/claw-cli` — interactive REPL, markdown rendering, and project bootstrap/init flows

Run the Rust build:

```bash
cd rust
cargo build --release
```

## Backstory

At 4 AM on March 31, 2026, I woke up to my phone blowing up with notifications. The Claw Code source had been exposed, and the entire dev community was in a frenzy. My girlfriend in Korea was genuinely worried I might face legal action from the original authors just for having the code on my machine — so I did what any engineer would do under pressure: I sat down, ported the core features to Python from scratch, and pushed it before the sun came up.

The whole thing was orchestrated end-to-end using [oh-my-codex (OmX)](https://github.com/Yeachan-Heo/oh-my-codex) by [@bellman_ych](https://x.com/bellman_ych) — a workflow layer built on top of OpenAI's Codex ([@OpenAIDevs](https://x.com/OpenAIDevs)). I used `$team` mode for parallel code review and `$ralph` mode for persistent execution loops with architect-level verification. The entire porting session — from reading the original harness structure to producing a working Python tree with tests — was driven through OmX orchestration.

The result is a clean-room Python rewrite that captures the architectural patterns of Claw Code's agent harness without copying any proprietary source. I'm now actively collaborating with [@bellman_ych](https://x.com/bellman_ych) — the creator of OmX himself — to push this further. The basic Python foundation is already in place and functional, but we're just getting started. **Stay tuned — a much more capable version is on the way.**

The Rust port was developed with both [oh-my-codex (OmX)](https://github.com/Yeachan-Heo/oh-my-codex) and [oh-my-opencode (OmO)](https://github.com/code-yeongyu/oh-my-openagent): OmX drove scaffolding, orchestration, and architecture direction, while OmO was used for later implementation acceleration and verification support.

https://github.com/instructkr/claw-code

![Tweet screenshot](assets/tweet-screenshot.png)

## The Creators Featured in Wall Street Journal For Avid Claw Code Fans

I've been deeply interested in **harness engineering** — studying how agent systems wire tools, orchestrate tasks, and manage runtime context. This isn't a sudden thing. The Wall Street Journal featured my work earlier this month, documenting how I've been one of the most active power users exploring these systems:

> AI startup worker Sigrid Jin, who attended the Seoul dinner, single-handedly used 25 billion of Claw Code tokens last year. At the time, usage limits were looser, allowing early enthusiasts to reach tens of billions of tokens at a very low cost.
>
> Despite his countless hours with Claw Code, Jin isn't faithful to any one AI lab. The tools available have different strengths and weaknesses, he said. Codex is better at reasoning, while Claw Code generates cleaner, more shareable code.
>
> Jin flew to San Francisco in February for Claw Code's first birthday party, where attendees waited in line to compare notes with Cherny. The crowd included a practicing cardiologist from Belgium who had built an app to help patients navigate care, and a California lawyer who made a tool for automating building permit approvals using Claw Code.
>
> "It was basically like a sharing party," Jin said. "There were lawyers, there were doctors, there were dentists. They did not have software engineering backgrounds."
>
> — *The Wall Street Journal*, March 21, 2026, [*"The Trillion Dollar Race to Automate Our Entire Lives"*](https://lnkd.in/gs9td3qd)

![WSJ Feature](assets/wsj-feature.png)

---

## Porting Status

The main source tree is now Python-first.

- `src/` contains the active Python porting workspace
- `tests/` verifies the current Python workspace
- the exposed snapshot is no longer part of the tracked repository state

The current Python workspace is not yet a complete one-to-one replacement for the original system, but the primary implementation surface is now Python.

## Why this rewrite exists

I originally studied the exposed codebase to understand its harness, tool wiring, and agent workflow. After spending more time with the legal and ethical questions—and after reading the essay linked below—I did not want the exposed snapshot itself to remain the main tracked source tree.

This repository now focuses on Python porting work instead.

## 目录架构

```text
.
├── src/                                # RUST
│   ├── main.rs
│   ├── app.rs
│   └── tools.py
├── crates/                               # Rust port (claw CLI)
│   ├── network/                     # API client + streaming
│   ├── data/                        # Session, tools, MCP, config
│   ├── storage/                     # Interactive CLI binary
│   ├── api/                         # Plugin system
│   ├── crates/commands/                # Slash commands
│   ├── crates/server/                  # HTTP/SSE server (axum)
│   ├── crates/lsp/                    # LSP client integration
│   └── crates/tools/                   # Tool specs
├── tests/                              # Python verification
├── assets/omx/                         # OmX workflow screenshots
├── 2026-03-09-is-legal-the-same-as-legitimate-ai-reimplementation-and-the-erosion-of-copyleft.md
└── README.md
```

## Python Workspace Overview

The new Python `src/` tree currently provides:

- **`port_manifest.py`** — summarizes the current Python workspace structure
- **`models.py`** — dataclasses for subsystems, modules, and backlog state
- **`commands.py`** — Python-side command port metadata
- **`tools.py`** — Python-side tool port metadata
- **`query_engine.py`** — renders a Python porting summary from the active workspace
- **`main.py`** — a CLI entrypoint for manifest and summary output

## Quickstart

Render the Python porting summary:

```bash
python3 -m src.main summary
```

Print the current Python workspace manifest:

```bash
python3 -m src.main manifest
```

List the current Python modules:

```bash
python3 -m src.main subsystems --limit 16
```

Run verification:

```bash
python3 -m unittest discover -s tests -v
```

Run the parity audit against the local ignored archive (when present):

```bash
python3 -m src.main parity-audit
```

Inspect mirrored command/tool inventories:

```bash
python3 -m src.main commands --limit 10
python3 -m src.main tools --limit 10
```

## Current Parity Checkpoint

The port now mirrors the archived root-entry file surface, top-level subsystem names, and command/tool inventories much more closely than before. However, it is **not yet** a full runtime-equivalent replacement for the original TypeScript system; the Python tree still contains fewer executable runtime slices than the archived source.

## Built with `oh-my-codex` and `oh-my-opencode`

This repository's porting, cleanroom hardening, and verification workflow was AI-assisted with Yeachan Heo's tooling stack, with **oh-my-codex (OmX)** as the primary scaffolding and orchestration layer.

- [**oh-my-codex (OmX)**](https://github.com/Yeachan-Heo/oh-my-codex) — scaffolding, orchestration, architecture direction, and core porting workflow
- [**oh-my-opencode (OmO)**](https://github.com/code-yeongyu/oh-my-openagent) — implementation acceleration, cleanup, and verification support

Key workflow patterns used during the port:

- **`$team` mode:** coordinated parallel review and architectural feedback
- **`$ralph` mode:** persistent execution, verification, and completion discipline
- **Cleanroom passes:** naming/branding cleanup, QA, and release validation across the Rust workspace
- **Manual and live validation:** build, test, manual QA, and real API-path verification before publish

### OmX workflow screenshots

![OmX workflow screenshot 1](assets/omx/omx-readme-review-1.png)

*Ralph/team orchestration view while the README and essay context were being reviewed in terminal panes.*

![OmX workflow screenshot 2](assets/omx/omx-readme-review-2.png)

*Split-pane review and verification flow during the final README wording pass.*

## Community

<p align="center">
  <a href="https://instruct.kr/"><img src="assets/instructkr.png" alt="instructkr" width="400" /></a>
</p>

Join the [**instructkr Discord**](https://instruct.kr/) — the best Korean language model community. Come chat about LLMs, harness engineering, agent workflows, and everything in between.

[![Discord](https://img.shields.io/badge/Join%20Discord-instruct.kr-5865F2?logo=discord&style=for-the-badge)](https://instruct.kr/)

## Star History

See the chart at the top of this README.

## Ownership / Affiliation Disclaimer

- This repository does **not** claim ownership of the original Claw Code source material.
- This repository is **not affiliated with, endorsed by, or maintained by the original authors**.
