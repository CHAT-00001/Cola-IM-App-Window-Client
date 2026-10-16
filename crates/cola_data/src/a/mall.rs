// // cola_data/src/mall/mall.rs  -- 商城首页
// // 2026/5/21 04:00 by wx: cestbon10080
// // * --------
// // * --------
//
// ////////aggregate
//
// use serde::Serialize;
// use crate::api::page::PageInfo;
// use crate::new::vo::new::VideoVo;
//
// ////////
//
// /// # [VO] 轮播图组件
// #[derive(Debug, Serialize, Clone)]
// pub struct BannerVo {
//     pub id: i64,
//     pub title: String,
//     pub image_url: String,
//     pub target_url: String, // 跳转链接（比如跳转到某个视频或活动页）
// }
//
// /// # [VO] 快捷菜单/网格导航组件
// #[derive(Debug, Serialize, Clone)]
// pub struct MenuVo {
//     pub id: i16,
//     pub name: String,
//     pub icon_url: String,
//     pub menu_type: String, // 菜单类型，如 "cola_live", "rank", "cola_fs"
// }
//
// /// # [RESPONSE] 整个聚合首页的完全体响应
// #[derive(Debug, Serialize)]
// pub struct MallAggregateResponse {
//     pub banners: Vec<BannerVo>,      // 轮播图模块
//     pub menus: Vec<MenuVo>,          // 菜单模块
//     pub recommend_list: Vec<VideoVo>,// 推荐视频列表（复用你完美的 VideoVo！）
//     pub hot_list: Vec<VideoVo>,      // 热门视频列表
//     pub page_info: PageInfo,         // 针对主列表的分页信息
// }
//
// // * --------
// //////// END

////////

// 伪代码：在你的 BIZ 层 / Service 层
// pub async fn get_home_aggregate_page(&self, user_id: i64) -> Result<MallAggregateResponse> {
//     // 🌟 三路数据同时并发并发请求，充分压榨 CPU
//     let banners_task = self.banner_service.get_active_banners();
//     let menus_task = self.menu_service.get_home_menus();
//     let videos_task = self.video_service.get_recommend_video_vos(user_id);
//
//     // 等待所有异步任务完成
//     let (banners, menus, video_vos) = tokio::try_join!(banners_task, menus_task, videos_task)?;
//
//     Ok(MallAggregateResponse {
//         banners,
//         menus,
//         recommend_list: video_vos,
//         hot_list: vec![], // 或者其他并发查出的数据
//         page_info: PageInfo::default(),
//     })
// }
