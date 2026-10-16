# DEVELOPMENT - 开发文档
2026-05-19 18:15

# DATA - lib 数据层



## 依赖顺序
- 被 api/core/storage/ 单向依赖


## 目录结构
- DATA/xxxx/ 应用名称
- - /command 创建命令
- - /entity 数据表映射
- - /info 数据对象信息
- - /model 数据视图模型（封装单条/列表响应）
- - /aggregate 聚合模型，把多条model聚合返回


## aggregate 聚合视图
- 复杂的前端页面

## command 创建数据命令
- 构造函数

## entity 数据表✊实体映射
- 纯表结构

## event 消息事件
- 

## model 数据对象
- vo视图对象，可直接嵌入响应壳data中