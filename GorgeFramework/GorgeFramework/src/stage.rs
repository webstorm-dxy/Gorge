//! 计分系统模块（对应 C# `Stage/` 文件夹）。
//!
//! 游戏结果的计分与评定：IScoring 接口、ScoringV1 计分器、
//! 里程碑（ScoreMilepost）、计分记录（ScoreRecord）等。

/// 里程碑枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScoreMilepost {
    NotPass = 0,
    Complete = 1,
    FullCombo = 2,
    AllPerfect = 3,
    MaxScore = 4,
}

/// 判定结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespondResult {
    Miss,
    Good,
    Perfect,
    BestPerfect,
}

/// 计分器接口
pub trait Scoring: Send + Sync {
    fn accuracy(&self) -> f32;
    fn score(&self) -> i32;
    fn combo(&self) -> i32;
    fn max_combo(&self) -> i32;
    fn milepost(&self) -> ScoreMilepost;

    /// 处理一次判定
    fn respond(&mut self, result: RespondResult);
}

/// 计分统计
#[derive(Debug, Clone, Default)]
pub struct ScoreRecord {
    pub perfect_count: i32,
    pub good_count: i32,
    pub miss_count: i32,
    pub max_combo: i32,
}

/// ScoringV1 计分器（对齐 C# `ScoringV1.cs` 完整公式）
///
/// 分数 = clamp(sqrt(700000*(comboBonus/maxComboBonus) + 300000*(accBonus/maxAccBonus)^10)*1000, 0, 10^6) + bestPerfect数
/// 准度 = accuracyBonus / (100 * 总判定数)
#[derive(Debug, Clone)]
pub struct ScoringV1 {
    /// 连击权重
    combo_weight: i32,
    /// 准度权重
    accuracy_weight: i32,
    /// 大P额外加分
    best_perfect_addition: i32,
    /// 准度幂次
    accuracy_exponent: i32,
    /// 各判定结果的准度奖励
    respond_accuracy_bonus: [i32; 4],
    /// 最大连击奖励 (1+2+...+maxMapCombo)
    max_combo_bonus: i32,
    /// 最大准度奖励 (maxMapCombo * 100)
    max_accuracy_bonus: i32,
    /// 当前已获得的连击奖励
    combo_bonus: i32,
    /// 当前已获得的准度奖励
    accuracy_bonus: i32,
    /// 当前连击数
    current_combo: i32,
    /// 总判定次数
    total_responds: i32,
    /// 历史最大连击
    max_combo: i32,
    /// 各判定结果计数（索引对应 RespondResult 顺序：Miss/Good/Perfect/BestPerfect）
    respond_counts: [i32; 4],
    /// 缓存的最新分数
    cached_score: i32,
    /// 缓存的准度
    cached_accuracy: f32,
    /// 当前里程碑（初始 MaxScore，按判定逐级降级）
    current_milepost: ScoreMilepost,
}

impl ScoringV1 {
    /// 创建计分器实例
    ///
    /// `max_map_combo` 为谱面的最大 Combo 数（Note 总数）。
    /// 若传入 0 则自动修正为 1 以避免除零。
    pub fn new(max_map_combo: i32) -> Self {
        let max_combo = if max_map_combo == 0 { 1 } else { max_map_combo };
        let max_combo_bonus = (max_combo + 1) * max_combo / 2;
        let max_accuracy_bonus = max_combo * 100;
        Self {
            combo_weight: 700000,
            accuracy_weight: 300000,
            best_perfect_addition: 1,
            accuracy_exponent: 10,
            respond_accuracy_bonus: [0, 50, 100, 100], // Miss=0, Good=50, Perfect=100, BestPerfect=100
            max_combo_bonus,
            max_accuracy_bonus,
            combo_bonus: 0,
            accuracy_bonus: 0,
            current_combo: 0,
            total_responds: 0,
            max_combo: 0,
            respond_counts: [0, 0, 0, 0],
            cached_score: 0,
            cached_accuracy: 1.0,
            current_milepost: ScoreMilepost::MaxScore,
        }
    }

    /// 判定结果 → 下标映射
    fn result_index(result: RespondResult) -> usize {
        match result {
            RespondResult::Miss => 0,
            RespondResult::Good => 1,
            RespondResult::Perfect => 2,
            RespondResult::BestPerfect => 3,
        }
    }

    /// 重新计算并缓存分数与准度
    fn recalc(&mut self) {
        if self.total_responds == 0 {
            self.cached_score = 0;
            self.cached_accuracy = 1.0;
            return;
        }
        // 连击分 = 700000 * (comboBonus / maxComboBonus)
        let combo_part = (self.combo_weight as f64) * (self.combo_bonus as f64 / self.max_combo_bonus as f64);
        // 准度分 = 300000 * (accuracyBonus / maxAccuracyBonus)^10
        let acc_ratio = self.accuracy_bonus as f64 / self.max_accuracy_bonus as f64;
        let acc_part = (self.accuracy_weight as f64) * acc_ratio.powi(self.accuracy_exponent);
        let pure_score = combo_part as i32 + acc_part as i32;
        let fixed_score = ((pure_score as f64).sqrt() * 1000.0) as i32;
        let clamped = fixed_score.clamp(0, 1_000_000);
        self.cached_score = clamped + self.respond_counts[3] * self.best_perfect_addition;
        // 准度 = accuracyBonus / (100 * totalResponds)
        self.cached_accuracy = self.accuracy_bonus as f32 / (100.0 * self.total_responds as f32);
    }
}

impl Scoring for ScoringV1 {
    fn accuracy(&self) -> f32 { self.cached_accuracy }

    fn score(&self) -> i32 { self.cached_score }

    fn combo(&self) -> i32 { self.current_combo }

    fn max_combo(&self) -> i32 { self.max_combo }

    fn milepost(&self) -> ScoreMilepost {
        self.current_milepost
    }

    fn respond(&mut self, result: RespondResult) {
        let idx = Self::result_index(result);
        self.respond_counts[idx] += 1;
        self.total_responds += 1;

        let bonus = self.respond_accuracy_bonus[idx];
        self.accuracy_bonus += bonus;

        match result {
            RespondResult::Miss => {
                self.current_combo = 0;
                self.current_milepost = ScoreMilepost::Complete;
                // _comboBonus 不变——不增加（断连）
            }
            RespondResult::Good => {
                self.current_combo += 1;
                self.combo_bonus += self.current_combo;
                self.current_milepost = match self.current_milepost {
                    ScoreMilepost::MaxScore | ScoreMilepost::AllPerfect => ScoreMilepost::FullCombo,
                    other => other,
                };
            }
            RespondResult::Perfect => {
                self.current_combo += 1;
                self.combo_bonus += self.current_combo;
                self.current_milepost = match self.current_milepost {
                    ScoreMilepost::MaxScore => ScoreMilepost::AllPerfect,
                    other => other,
                };
            }
            RespondResult::BestPerfect => {
                self.current_combo += 1;
                self.combo_bonus += self.current_combo;
                // BestPerfect 不降级
            }
        }

        if self.current_combo > self.max_combo {
            self.max_combo = self.current_combo;
        }

        self.recalc();
    }
}

impl Default for ScoringV1 {
    fn default() -> Self { Self::new(1) }
}

/// 游戏结果
#[derive(Debug, Clone)]
pub struct GameResult {
    pub score: i32,
    pub accuracy: f32,
    pub max_combo: i32,
    pub perfect_count: i32,
    pub good_count: i32,
    pub miss_count: i32,
    pub milepost: ScoreMilepost,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scoring(max_combo: i32) -> ScoringV1 {
        ScoringV1::new(max_combo)
    }

    #[test]
    fn test_new_scoring_zero_max_combo_auto_fix() {
        let s = scoring(0);
        assert!((s.accuracy() - 1.0).abs() < 0.01);
        assert_eq!(s.score(), 0);
        assert_eq!(s.combo(), 0);
        assert_eq!(s.max_combo(), 0);
    }

    #[test]
    fn test_single_perfect() {
        let mut s = scoring(10);
        s.respond(RespondResult::Perfect);
        assert_eq!(s.combo(), 1);
        assert_eq!(s.max_combo(), 1);
        assert!(s.score() > 0);
    }

    #[test]
    fn test_all_perfect_full_score() {
        // maxMapCombo=10 → maxComboBonus = (10+1)*10/2 = 55
        // maxAccuracyBonus = 10*100 = 1000
        // 10 次 Perfect: comboBonus = 1+2+...+10 = 55, accuracyBonus = 10*100 = 1000
        // comboPart = 700000 * (55/55) = 700000
        // accPart = 300000 * (1000/1000)^10 = 300000 * 1 = 300000
        // pureScore = 700000 + 300000 = 1000000
        // sqrt(1000000)*1000 = 1000*1000 = 1000000
        // clamp(1000000, 0, 1000000) = 1000000
        // + bestPerfect(0)*1 = 1000000
        let mut s = scoring(10);
        for _ in 0..10 {
            s.respond(RespondResult::Perfect);
        }
        assert_eq!(s.score(), 1_000_000);
        assert!((s.accuracy() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_all_best_perfect_full_score() {
        // 全大P: 1000000 + 10*1 = 1000010
        let mut s = scoring(10);
        for _ in 0..10 {
            s.respond(RespondResult::BestPerfect);
        }
        assert_eq!(s.score(), 1_000_010);
        assert!((s.accuracy() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_miss_breaks_combo() {
        // 判定序列: P-P-M-P
        // respond 1 (P): combo=1, comboBonus=1
        // respond 2 (P): combo=2, comboBonus=1+2=3
        // respond 3 (M): combo=0, comboBonus=3, accuracyBonus=100+100+0=200
        // respond 4 (P): combo=1, comboBonus=3+1=4, accuracyBonus=300
        let mut s = scoring(10);
        s.respond(RespondResult::Perfect);
        s.respond(RespondResult::Perfect);
        s.respond(RespondResult::Miss);
        s.respond(RespondResult::Perfect);
        assert_eq!(s.combo(), 1);
        // 4 次判定，准度奖励 300
        let expected_acc = 300.0 / (100.0 * 4.0);
        assert!((s.accuracy() - expected_acc as f32).abs() < 0.01);
        // 分数应低于满分（comboBonus=4, accuracyBonus=300）
        assert!(s.score() < 1_000_000);
        assert!(s.score() > 0);
    }

    #[test]
    fn test_accuracy_calculation() {
        // 10 个 Perfect → accuracyBonus = 1000, totalResponds = 10
        // accuracy = 1000 / (100*10) = 1.0
        let mut s = scoring(10);
        for _ in 0..10 {
            s.respond(RespondResult::Perfect);
        }
        assert!((s.accuracy() - 1.0).abs() < 0.01);

        // 5 Perfect + 5 Miss → accuracyBonus = 500, totalResponds = 10
        // accuracy = 500 / (100*10) = 0.5
        let mut s2 = scoring(10);
        for _ in 0..5 { s2.respond(RespondResult::Perfect); }
        for _ in 0..5 { s2.respond(RespondResult::Miss); }
        assert!((s2.accuracy() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_good_accuracy_bonus() {
        // Good = 50 奖励
        let mut s = scoring(10);
        s.respond(RespondResult::Good);
        assert_eq!(s.combo(), 1);
        // accuracyBonus = 50, totalResponds = 1
        // accuracy = 50 / (100*1) = 0.5
        assert!((s.accuracy() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_mixed_responds() {
        // P, BP, G, M, P → combo: 1,2,3,0,1 → comboBonus: 1+2+3+0+1=7
        // accuracyBonus: 100+100+50+0+100=350, total: 5
        let mut s = scoring(10);
        s.respond(RespondResult::Perfect);
        s.respond(RespondResult::BestPerfect);
        s.respond(RespondResult::Good);
        s.respond(RespondResult::Miss);
        s.respond(RespondResult::Perfect);
        assert_eq!(s.combo(), 1);
        assert_eq!(s.max_combo(), 3);
        let expected_acc = 350.0 / (100.0 * 5.0);
        assert!((s.accuracy() - expected_acc as f32).abs() < 0.01);
    }

    // ==================== R-4 里程碑降级测试 ====================

    #[test]
    fn test_r4_milepost_starts_at_max_score() {
        let s = scoring(10);
        assert_eq!(s.milepost(), ScoreMilepost::MaxScore);
    }

    #[test]
    fn test_r4_milepost_perfect_downgrades_to_all_perfect() {
        // C# 语义：非 BestPerfect 的 Perfect 降 MaxScore→AllPerfect
        let mut s = scoring(10);
        assert_eq!(s.milepost(), ScoreMilepost::MaxScore);
        s.respond(RespondResult::Perfect);
        assert_eq!(s.milepost(), ScoreMilepost::AllPerfect);
    }

    #[test]
    fn test_r4_milepost_good_downgrades_to_full_combo() {
        // C# 语义：Good 降 MaxScore/AllPerfect→FullCombo
        let mut s = scoring(10);
        s.respond(RespondResult::Good);
        assert_eq!(s.milepost(), ScoreMilepost::FullCombo);

        let mut s2 = scoring(10);
        s2.respond(RespondResult::Perfect); // → AllPerfect
        s2.respond(RespondResult::Good);    // → FullCombo
        assert_eq!(s2.milepost(), ScoreMilepost::FullCombo);
    }

    #[test]
    fn test_r4_milepost_miss_downgrades_to_complete() {
        // C# 语义：Miss 直接降至 Complete（非 NotPass）
        let mut s = scoring(10);
        s.respond(RespondResult::Miss);
        assert_eq!(s.milepost(), ScoreMilepost::Complete);
    }

    #[test]
    fn test_r4_milepost_best_perfect_does_not_downgrade() {
        // C# 语义：BestPerfect 不降级
        let mut s = scoring(10);
        s.respond(RespondResult::BestPerfect);
        assert_eq!(s.milepost(), ScoreMilepost::MaxScore);
    }

    #[test]
    fn test_r4_milepost_full_degradation_chain() {
        // 完整降级链：MaxScore → (Perfect) AllPerfect → (Good) FullCombo → (Miss) Complete
        let mut s = scoring(10);
        assert_eq!(s.milepost(), ScoreMilepost::MaxScore);
        s.respond(RespondResult::Perfect);
        assert_eq!(s.milepost(), ScoreMilepost::AllPerfect);
        s.respond(RespondResult::Good);
        assert_eq!(s.milepost(), ScoreMilepost::FullCombo);
        s.respond(RespondResult::Miss);
        assert_eq!(s.milepost(), ScoreMilepost::Complete);
    }
}
