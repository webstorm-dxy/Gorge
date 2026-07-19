//! `FloatExtension` —— f32 位模式扩展。
//!
//! 移植自 C# 参考实现 `Utilities/FloatExtension.cs`。
//! 提供 `bit_int` 方法将 f32 的 IEEE 754 位模式按 i32 解释。

/// 将 f32 的 IEEE 754 位模式解释为 i32
///
/// 对应 C# `BitConverter.ToInt32(BitConverter.GetBytes(f), 0)`。
/// Rust 的 `f32::to_bits()` 直接返回位模式 u32，转为 i32 即可。
pub fn bit_int(f: f32) -> i32 {
    f.to_bits() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_int_one() {
        // 1.0f32 的 IEEE 754 位模式 = 0x3F800000
        assert_eq!(bit_int(1.0), 0x3F800000i32 as i32);
    }

    #[test]
    fn test_bit_int_zero() {
        assert_eq!(bit_int(0.0), 0);
    }

    #[test]
    fn test_bit_int_negative() {
        // -1.0f32 的 IEEE 754 位模式 = 0xBF800000
        assert_eq!(bit_int(-1.0), 0xBF800000u32 as i32);
    }
}
