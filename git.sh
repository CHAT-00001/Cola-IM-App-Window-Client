# GIT命令  -- ⚠️AI不要修改


# 1. 初始化 Git 仓库（如果尚未初始化）
git init

# 2. 创建并配置 .gitignore（参考之前的 Rust 配置）
# （手动创建或用命令生成，例如：curl -o .gitignore https://www.toptal.com/developers/gitignore/api/rust,visualstudiocode）

# 3. 检查文件状态（确认 .gitignore 生效）
git status

# 4. 暂存文件（⚠️ 建议用具体路径而非 git add .，避免误加编译产物）
# 使用以下命令逐个目录添加：
git add src/ app/ api/ cola_*/ gateway/ health/ im/ kits/ network/ repo/ migrations/ assets/
# 注意：target/、build_output*.txt 等已被 .gitignore 忽略

# 修改.gitignore删掉旧屏蔽规则
# 清除仓库缓存重新扫描：
# bash
# 运行
git rm -r --cached .
git add .
git commit -m "init project"
# 创建本地分支
git branch -M main

# 日常更新
git add .
git commit -m "更新内容"
git push

# 从远程主分支恢复到本地
git checkout origin/main -- crates/cola_user

# 检查大小
git count-objects -vH
# 5. 提交到本地仓库
git commit -m "Initial commit: Add Rust project source code"

# 6. （可选）关联远程仓库（首次推送时需要）
git remote -v
git remote add origin https://github.com/CHAT-00001/Cola-IM-SERVICE-CMS.git

# 7. （可选）推送到远程仓库
git push -u origin main

# 8. 覆盖推送（⚠️ 仅在需要强制覆盖远程时使用）
git push -u origin main --force


## 方案 1：安全合并（推荐，保留两边代码）

```
# 拉取远端代码并合并到本地main
git pull origin main
# 如果出现冲突，手动解决冲突文件后，执行
git add .
git commit -m "merge remote main"
# 再推送
git push -u origin main
```

# 先删除远程旧标签
git push origin :v0.1.1

# 重新推送本地标签
git push origin v0.1.1

# 场景 3：想新建别的版本标签（推荐常规做法）
# 直接打新版本号，比如 v0.1.2：
git tag -a v0.1.2 -m "Release v0.1.2"
git push origin v0.1.2
