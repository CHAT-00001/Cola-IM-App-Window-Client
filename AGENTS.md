# 🤖 AGENTS 编码规范约束

> 本项目所有 Rust 代码必须严格遵守以下规范，AI 助手与开发者共同遵循。🕒🕒

---

## 1. 🔤 代码块分隔符（强制）

- **必须使用 8 个斜杠 `////////`** 作为代码块分隔符，
- 文件内各逻辑段之间使用 `////////` 分隔
- 文件末尾使用 `//////// END` 标记结束（同样 8 个斜杠)

**正确：**
```rust
////////

use xxx::yyy;

////////

pub fn foo() {}

//////// END
```

**错误（禁止）：**
```rust
//////

use xxx::yyy;

//////
```

---

## 2. 🗂️ 文件头部注释（强制）

每个源文件顶部必须有注释头，格式：
```rust
// 完整路径
// 简短说明
// 日期 时间 Created.
```

示例：
```rust
// cola_data/src/auth/port/service.rs  -- 数据 - AUTH - port - 会话接口
// 2026/8/1 10:25 Created.

////////

```

---

## 3. 🔤 编码与字符（强制）

- 所有中文注释必须使用 **UTF-8** 编码，**禁止 GBK**
- 注释中使用 emoji 标注模块职责（如 `🚧` 开发中、`✅` 完成、`🌟` 重点）

---

## 4. 🏗️ 分层架构（强制）

请求自上而下的层级，各层职责不可混淆：

| 层级 | 职责 |
|------|------|
| `GATEWAY` | web 框架（Actix），解析/合并参数、前置身份校验、按 service 分发 |
| `API HANDLER` | 应用权限编排、统一响应壳 `AppData<T>` |
| `CASE` | 逻辑流程编排，调用 Port 接口 |
| `PORT` | 定义 trait 接口（如 `SessionPort`、`TypePort`） |
| `ADAPTER` | 实现 Port trait，包装 repository 静态服务 |
| `REPOSITORY` | 仓储层（pg / redis / grpc / mock） |

**分层原则：**
- 网关层负责：URL + Body(JSON) 双重解析（Body 为主）、`verify_session` 会话验证、分发
- 转发器层（dispatcher）**不得自带会话验证**，session 由 gateway 传入
- 底层函数需要什么参数自己从链路中取，不层层透传整包

---

## 5. 🔌 Port / Adapter 规范（强制）

- `cola_data` 中定义 `trait`（如 `SessionPort`），配 `Arc<dyn Trait + Send + Sync + 'static>` 注入结构体
- `repo_adapter` 层实现该 trait，并通过 `build_app_context()` 统一装配
- trait 方法用 `#[async_trait::async_trait]` 标注

### Redis 客户端挂载约定（强制）

- Redis 客户端由 `app::DbService` 统一初始化：`crate::db::redis::redis_init(&config.redis)`
- 初始化后通过 `redis_client.get_multiplexed_async_connection().await` 得到 `redis_conn`
- `DbService` 持有字段 `pub redis_conn: MultiplexedConnection`
- 全局通过 `app_config::GLOBAL_DB` 挂载，业务代码优先使用 `app_config::GLOBAL_DB.get()` 获取 `DbService`
- 需要 Redis 的 `repository` / `repo_adapter` 代码，统一复用 `db.redis_conn.clone()` 获取可并发使用的连接
- 计划阶段如已确认以上链路存在，默认视为“Redis 客户端已挂载完成”，无需重复追查初始化流程

---

## 6. 🌐 网关参数规范（强制）

- 统一使用 `cola_data::app::query::ApiGatewayRequest` 作为网关请求体
- 禁止在网关层自建私有解析参数结构体（如 `GatewayQuery`）
- URL 与 Body(JSON) 双重命中，**Body 为主**：`url_req.merge(body_req)` 字段级合并

---

## 7. 🎯 命名规范（建议）

- 动作码常量集中在 `dispatcher/xxx.rs` 的 `pub mod action` 中
- 转发器命名 `xxx_dispatch`，handler 命名 `xxx_xxx`
- Port 结构体命名 `XxxPorts`（复数，含多个 Arc 端口）
- Adapter 命名 `XxxPortAdapter`

---

## 8. ✅ 其他约定

- 分页字段：请求用 `page` / `qty`，后端计算 `limit` / `offset`（由 `ApiGatewayRequest::build()` 处理）
- 统一响应壳：`AppData<T>`（`AppData::ok` / `AppData::err` / `AppData::empty`）
- 会话上下文：`SessionContext`（请求侧可信会话），`AuthContext`（业务侧登录态），两者职责区分

---

## 9. 📢 分层日志规范（强制）

每层日志必须使用 emoji 标注层名和状态，方便在控制台快速定位问题所在层。

### 日志格式

```
[<层标识> <模块名>] - <状态图标> <描述>
```

### 层标识（带 emoji）

| 层 | 标识前缀 | 示例 |
|----|---------|------|
| GATEWAY | `[🌐 GATEWAY]` | `[🌐 GATEWAY]: ⚠️ Unknown service: xxx` |
| DISPATCHER | `[🚧 DISPATCH]` | `[🚧 DISPATCH]: Unknown dispatch action` |
| API | `[🗣️ API]` / `[🤐 API]` | `[🗣️ API] - ✅️ 开通会员成功` |
| CASE | `[🗣️ CASE]` / `[🤐 CASE]` | `[🤐 CASE]: ❌️ 充值会员失败` |
| ADAPTER | `[🔌 ADAPTER]` | `[🔌 ADAPTER]: ✅ session 查询成功` |
| SERVICE | `[🗣️ XXX SERVICE]` / `[🤐 XXX SERVICE]` | `[🤐 FOLLOW SERVICE]: ❌️ 查询关注失败` |
| REPOSITORY | (tracing::info!/error! 由上层调用，不独立打日志) |

### 状态图标（强制）

| 场景 | 图标 | 用途 |
|------|------|------|
| 成功 | `✅️` | 操作成功（info 级别） |
| 失败 | `❌️` | 操作失败（error/warn 级别） |
| 发音 | `🗣️` | 成功日志的前缀（代表"正常说话"） |
| 禁言 | `🤐` | 失败日志的前缀（代表"被禁言/出错了"） |

### 正确示例

```rust
// ✅ 成功日志
info!("[🗣️ FOLLOW CASE]: ✅️ 添加关注成功: uid={}, target_id={}", uid, target_id);
info!("[🗣️ FOLLOW SERVICE]: ✅️ 关注列表查询成功, uid={}, count={}", uid, infos.len());
info!("[🗣️ API] - ✅️ 用户充值贵宾卡成功!");

// ❌ 错误日志
error!("[🤐 API] - ❌️ 用户充值贵宾卡失败!");
warn!("[🤐 FOLLOW SERVICE]: ❌️ 查询关注IDs失败: {}", e);
warn!("[🤐 FOLLOW CASE]: ❌️ 获取关注列表失败: {}", e);
```

### 错误代码规范

- 用户输入错误/业务校验失败 → `error!(...)` + `[🤐 LAYER] - ❌️`
- 系统内部异常 → `error!(...)` + `[🤐 LAYER] - ❌️`
- 预期外的降级/兜底 → `warn!(...)` + `[🤐 LAYER] - ❌️`

> **注意**: 仓储层（REPOSITORY）不独立打日志，由调用方 SERVICE 层统一记录。

---

## 10. 📝 注释与文档规范（强制）

### 函数/方法注释

函数注释使用 `/// #` 格式,只包含标题和描述,参数用行内注释:

**正确:**
```rust
/// # 1. [CASE] - 获取用户资料
/// * `desc`: 根据用户ID查询资料名片
pub async fn case_get_profile(
    user_id: i64, // 目标用户ID
    ctx: &AppContext, // 全局上下文
) -> Result<Option<ProfileInfo>, anyhow::Error> {
```

**错误(禁止):**
```rust
/// # 1. [CASE] - 获取用户资料
/// * `desc`: 根据用户ID查询资料名片
/// * `user_id`: 目标用户ID          ← 参数不要写在 /// 注释里!
/// * `ctx`: 全局上下文              ← 参数不要写在 /// 注释里!
pub async fn case_get_profile(
    user_id: i64,
    ctx: &AppContext,
) -> Result<Option<ProfileInfo>, anyhow::Error> {
```

### 特殊情况

只有需要补充额外约束时,才在标题后增加第三行 `/// * condition:`:

```rust
/// # 1. [REPOSITORY] - 批量查询用户实体
/// * `desc`: 根据IDs批量查询,空数组直接返回空
/// * `condition`: `ids 长度超过 500 时分批查询`
```

### 参数注释格式

参数注释统一使用 `//` 在参数行末尾:

```rust
/// # 1. [XX XX] - Title
/// * `desc`: `dese content`
/// * `condition`: `⚠️ xxx xxx`
pub async fn save_record(
    uid: i64,       // 操作者ID
    target_id: i64, // 目标用户ID
    status: i16,    // 状态码: 1. 有效 0. 取消
) -> Result<u64> {
```

### 结构体字段注释

```rust
/// # 1. [XX XX] - Title
/// * `desc 1`: `dese content`
/// * `desc 2`: `dese content`
pub struct ProfileInfo {
    pub user_id: i64,            // 用户 ID
    pub nickname: String,        // 昵称
    pub avatar: String,          // 头像
    pub sex: Option<i16>,        // 性别: 0未知 1男 2女
    pub status: i16,             // 状态: 1有效 0失效
}
```

### 工作模式
GATE > API >  CASE > PORT(可选) > ADAPTER(可选) > SERVICE > REPOSITORY > PG/REDIS
* CASE 有2种方式向下调用repository层  
* 1. 纯函数调用, 直接Call SERVICE Layer
* 2. Port/Adapter (RUST trait抽象的插座/插头) 

### 数据链路
CMD > ENTITY > INFO(DTO/CACHE LAYER) > VO > BUILDER > APP DATA SHELL > BODY


### 