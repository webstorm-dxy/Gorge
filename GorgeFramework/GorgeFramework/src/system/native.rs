//! System/Native 模块 —— 框架 native 类（对应 C# `System/Native/` 文件夹）。
//!
//! 全部框架 native 类以纯 Rust 结构体 + 桥接宏的形式实现于此。

pub mod bool_signal;
pub mod color_argb;
pub mod color_curve;
pub mod commands;
pub mod element;
pub mod element_native;
pub mod element_simulator;
pub mod float_signal;
pub mod float_signal_filter;
pub mod function_curve;
pub mod history;
pub mod input_graph;
pub mod input_graph_state;
pub mod input_signal_filter_native;
pub mod logger;
pub mod math;
pub mod node;
pub mod node_native;
pub mod note_native;
pub mod note_linkage;
pub mod period_config;
pub mod priority;
pub mod random;
pub mod resource;
pub mod signal_filter;
pub mod signal_filter_native;
pub mod signal_tsiga;
pub mod time;
pub mod touch_signal;
pub mod transform;
pub mod variable_float;
pub mod vector2;
pub mod vector3;
pub mod lerp_color_curve;
pub mod annulus_mesh_transformer;
pub mod curve_warp_transformer;
// S6: 资产族 + 精灵族 + 音频/视频 + 环境
pub mod audio_asset;
pub mod video_asset;
pub mod audio_native;
pub mod video_native;
pub mod sprite_native;
pub mod nine_slice_sprite;
pub mod curve_sprite_native;
pub mod environment_native;
