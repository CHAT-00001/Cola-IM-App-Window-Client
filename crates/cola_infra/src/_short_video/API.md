## 短视频模块APIs接口 - V2命令式网关



# HOME 前台类
path: `api/v2/video/gateway`
params:`qty / page`
service:
1. `home_new`,     - 最新视频
2. `home_hot`      - 热门视频
3. `home_recommend`- 推荐
4. `home_category` - 分类
5. `home_channel`  - 频道
6. `home_nearby`   - 附近
7. `home_search`   - 搜索
        
        
# ADD 发布类 - 视频
path: `api/v2/video/gateway`
service:
1. `publish_video` - 发布视频
2. `edit_video`    - 编辑视频
3. `change_perm`   - 修改权限
4. `change_status` - 修改状态
5. `delete_video`  - 删除视频

# ADD 互动类 - 视频
path: `api/v2/video/gateway`
service:
1. `view_video`    - 浏览视频
2. `like_video`    - 点赞视频
3. `dislike_video` - 不喜欢视频
4. `recommend_video` - 推荐视频

# COMMENT - 评论
path: `api/v2/video/gateway`
service:
1. `send_comment`  - 发布评论
2. `get_comment`   - 获取评论
3. `delete_comment`- 逻辑删除评论
4. `like_comment`  - 点赞评论
5. `dislike_comment` - 不喜欢评论
6. `my_comment`    - 我发布的评论

# DANMAKU - 弹幕
path: `api/v2/video/gateway`
service:
1. `send_comment`  - 发布评论
2. `get_comment`   - 获取评论
3. `delete_comment`- 逻辑删除评论
4. `like_comment`  - 点赞评论
5. `dislike_comment` - 不喜欢评论
6. `my_comment`    - 我发布的评论

# COLLECT - 收藏
path: `api/v2/video/gateway`
service:
1. `add_collect`   - 添加收藏
2. `get_collect_videos`   - 获取我收藏的视频
3. `delete_collect`- 删除收藏
4. `edit_collect`  - 编辑收藏

# SHARE - 分享
path: `api/v2/video/gateway`
service:
1. `add_share`     - 添加分享
2. `get_share`     - 获取分享
3. `delete_share`  - 删除分享

# REPORT - 检举
path: `api/v2/video/gateway`
service:
1. `add_report`     - 添加举报
2. `get_report`     - 获取举报
3. `delete_report`  - 删除举报