//! 仿真模块 trait（对应 C# `Runtime/Environment/SimulationModule.cs`）。
//!
//! 定义仿真运行时各子管理器的生命周期钩子。

/// 仿真模块生命周期 trait（对应 C# `SimulationModule` 抽象类）。
///
/// 所有子管理器（ChartManager、AudioManager、SimulationManager 等）通过实现本 trait
/// 参与统一的 LoadScore / UnloadScore / StartSimulation / StopSimulation 生命周期。
///
/// 每个方法分为两层：外层做状态检查和保护，内层 `do_*` 由具体模块实现业务逻辑。
///
/// # 与 C# 差异
///
/// C# `SimulationModule` 是抽象类，持有 `IsScoreLoaded`/`IsSimulating` 状态字段，
/// 每个具体模块继承它并共享同一套状态保护逻辑。Rust 版本改为 trait，具体模块需自行维护状态。
/// 这使得模块间状态独立（一个模块的状态不影响其他模块），与 C# 的集中继承设计不同。
pub trait SimulationModule {
    /// 是否已加载谱面
    fn is_score_loaded(&self) -> bool;

    /// 是否正在仿真
    fn is_simulating(&self) -> bool;

    /// 设置谱面加载状态
    fn set_score_loaded(&mut self, loaded: bool);

    /// 设置仿真状态
    fn set_simulating(&mut self, simulating: bool);

    /// 加载谱面（外层状态检查）
    ///
    /// 若已加载则先卸载再加载，然后调用 `do_load_score()`。
    fn load_score(&mut self) {
        if self.is_score_loaded() {
            self.unload_score();
        }
        self.do_load_score();
        self.set_score_loaded(true);
    }

    /// 卸载谱面（外层状态检查）
    ///
    /// 若未加载则直接返回。若正在仿真则先停止仿真。
    fn unload_score(&mut self) {
        if !self.is_score_loaded() {
            return;
        }
        if self.is_simulating() {
            self.stop_simulation();
        }
        self.do_unload_score();
        self.set_score_loaded(false);
    }

    /// 启动仿真（外层状态检查）
    ///
    /// 若未加载谱面则 panic。若已在仿真则先停止再启动。
    fn start_simulation(&mut self) {
        if !self.is_score_loaded() {
            panic!("尝试在谱面加载前启动仿真");
        }
        if self.is_simulating() {
            self.stop_simulation();
        }
        self.do_start_simulation();
        self.set_simulating(true);
    }

    /// 停止仿真（外层状态检查）
    ///
    /// 若未在仿真中则直接返回。
    fn stop_simulation(&mut self) {
        if !self.is_simulating() {
            return;
        }
        self.do_stop_simulation();
        self.set_simulating(false);
    }

    // ==================== 子类须覆写的方法 ====================

    /// 执行加载谱面的具体逻辑
    fn do_load_score(&mut self);

    /// 执行卸载谱面的具体逻辑
    fn do_unload_score(&mut self);

    /// 执行启动仿真的具体逻辑
    fn do_start_simulation(&mut self);

    /// 执行停止仿真的具体逻辑
    fn do_stop_simulation(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试用模块：记录生命周期调用序
    #[derive(Default)]
    struct TestModule {
        score_loaded: bool,
        simulating: bool,
        call_log: Vec<String>,
    }

    impl TestModule {
        fn new() -> Self { Self::default() }
    }

    impl SimulationModule for TestModule {
        fn is_score_loaded(&self) -> bool { self.score_loaded }
        fn is_simulating(&self) -> bool { self.simulating }
        fn set_score_loaded(&mut self, loaded: bool) { self.score_loaded = loaded; }
        fn set_simulating(&mut self, simulating: bool) { self.simulating = simulating; }

        fn do_load_score(&mut self) { self.call_log.push("do_load_score".into()); }

        fn do_unload_score(&mut self) {
            self.call_log.push("do_unload_score".into());
        }

        fn do_start_simulation(&mut self) {
            self.call_log.push("do_start_simulation".into());
        }

        fn do_stop_simulation(&mut self) {
            self.call_log.push("do_stop_simulation".into());
        }
    }

    #[test]
    fn test_f2_simulation_module_lifecycle_order() {
        let mut m = TestModule::new();
        m.load_score();
        m.start_simulation();
        m.stop_simulation();
        m.unload_score();
        assert_eq!(
            m.call_log,
            vec![
                "do_load_score",
                "do_start_simulation",
                "do_stop_simulation",
                "do_unload_score",
            ]
        );
    }

    #[test]
    fn test_f2_simulation_module_double_load_unloads_first() {
        let mut m = TestModule::new();
        m.load_score();
        m.load_score(); // 第二次加载应先卸载
        assert_eq!(m.call_log, vec!["do_load_score", "do_unload_score", "do_load_score"]);
    }

    #[test]
    fn test_f2_simulation_module_unload_not_loaded_noop() {
        let mut m = TestModule::new();
        m.unload_score();
        assert!(m.call_log.is_empty());
    }

    #[test]
    fn test_f2_simulation_module_stop_not_simulating_noop() {
        let mut m = TestModule::new();
        m.stop_simulation();
        assert!(m.call_log.is_empty());
    }

    #[test]
    #[should_panic(expected = "尝试在谱面加载前启动仿真")]
    fn test_f2_simulation_module_start_without_load_panics() {
        let mut m = TestModule::new();
        m.start_simulation();
    }
}
