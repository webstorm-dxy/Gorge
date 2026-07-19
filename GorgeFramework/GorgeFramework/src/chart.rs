//! 谱面数据链模块（对应 C# `Chart/` 文件夹）。
//!
//! 包含从谱面包（Package）到谱表（Staff）/乐段（Period）再到仿真总谱（SimulationScore）
//! 的完整数据模型。

pub mod period;
pub mod staff;
pub mod package;
pub mod simulation_score;

// 重导出常用类型
pub use period::{PeriodConfig, IPeriod, PeriodData, ElementPeriod, AudioPeriod};
pub use staff::{IStaff, ElementStaff, AudioStaff};
pub use package::{Package, AssetFile, SourceCodeFile, PackageError, is_zip_file};
pub use simulation_score::{
    SimulationScore, AssetLoader, AssetSet, Asset, AssetBackend, MockAssetBackend,
};
