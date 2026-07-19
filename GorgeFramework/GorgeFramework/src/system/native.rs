//! System/Native 模块 —— 框架 native 类（对应 C# `System/Native/` 文件夹）。
//!
//! 全部框架 native 类以纯 Rust 结构体 + 桥接宏的形式实现于此。

pub mod bool_signal;
pub mod color_argb;
pub mod color_curve;
pub mod float_signal;
pub mod float_signal_filter;
pub mod history;
pub mod input_graph;
pub mod input_graph_edge;
pub mod input_graph_state;
pub mod input_signal_filter;
pub mod logger;
pub mod math;
pub mod node;
pub mod node_native;
pub mod note;
pub mod note_linkage;
pub mod period_config;
pub mod priority;
pub mod random;
pub mod signal_filter;
pub mod signal_filter_native;
pub mod signal_tsiga;
pub mod touch_signal;
pub mod transform;
pub mod variable_float;
pub mod vector2;
pub mod vector3;

// 拆分自 resource.rs
pub mod asset;
pub mod graph;
pub mod graph_asset;
pub mod image_asset;
pub mod native_audio_asset;
pub mod native_video_asset;

// 拆分自 commands.rs
pub mod append_signal_command;
pub mod derive_element_command;
pub mod destroy_element_command;

// 拆分自 time.rs
pub mod time_item;
pub mod time_stack;

// 拆分自 element.rs
pub mod element_line_point;
pub mod element_line;

// 拆分自 function_curve.rs
pub mod constant_function_curve;
pub mod linear_function_curve;
pub mod quadratic_function_curve;
pub mod linear_curve;
pub mod arc_function_curve;
pub mod cubic_hermite_spline;

// function_curve.rs 保留基类/组合器/trait
pub mod function_curve;
pub mod function_curve_combinators;

// 元素/模拟器/变换器
pub mod element_native;
pub mod element_simulator;
pub mod lerp_color_curve;
pub mod annulus_mesh_transformer;
pub mod curve_warp_transformer;

// S6: 资产族 + 精灵族 + 音频/视频 + 环境（已去掉 _native 后缀）
pub mod audio_asset;
pub mod wav_audio_asset;
pub mod video_asset;
pub mod audio;
pub mod video;
pub mod sprite;
pub mod nine_slice_sprite;
pub mod curve_sprite;
pub mod environment;
