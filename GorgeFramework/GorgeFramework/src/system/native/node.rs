//! `GorgeFramework.Node` — 场景图节点。
//!
//! 移植自 C# 参考实现 `System/Native/Node.cs`。提供层级变换（位置/旋转/缩放）
//! 及引用链机制，支持节点跟随和全局坐标计算。
//!
//! 使用 `glam` 进行向量/四元数运算。

use glam::{Vec3, Quat};

/// 场景图节点
///
/// 每个节点可以有独立的位置/旋转/缩放，也可通过 `*Reference` 字段
/// 引用另一个节点，此时全局坐标沿引用链递归计算。
#[derive(Debug, Clone)]
pub struct Node {
    /// 是否存活（销毁后为 false）
    pub alive: bool,
    /// 存活依赖：若引用节点不存活，本节点也标记为不存活
    pub existence_reference: Option<Box<Node>>,
    /// 局部位置
    pub position: Vec3,
    /// 位置引用（为空则使用自己的 position）
    pub position_reference: Option<Box<Node>>,
    /// 局部旋转（欧拉角，弧度）
    pub rotation: Vec3,
    /// 旋转引用
    pub rotation_reference: Option<Box<Node>>,
    /// 局部缩放（默认 1,1,1）
    pub size: Vec3,
    /// 缩放引用
    pub size_reference: Option<Box<Node>>,
}

impl Node {
    /// 创建默认节点
    pub fn new() -> Self {
        Self {
            alive: true,
            existence_reference: None,
            position: Vec3::ZERO,
            position_reference: None,
            rotation: Vec3::ZERO,
            rotation_reference: None,
            size: Vec3::ONE,
            size_reference: None,
        }
    }

    /// 全局位置（沿 positionReference 链递归）
    pub fn global_position(&self) -> Vec3 {
        match &self.position_reference {
            Some(r) => r.local_position_to_global(self.position),
            None => self.position,
        }
    }

    /// 全局旋转（沿 rotationReference 链递归，加法合成）
    pub fn global_rotation(&self) -> Vec3 {
        match &self.rotation_reference {
            Some(r) => r.global_rotation() + self.rotation,
            None => self.rotation,
        }
    }

    /// 全局缩放（沿 sizeReference 链递归，乘法合成）
    pub fn global_size(&self) -> Vec3 {
        match &self.size_reference {
            Some(r) => r.global_size() * self.size,
            None => self.size,
        }
    }

    /// 将局部坐标转为全局坐标
    pub fn local_position_to_global(&self, local_pos: Vec3) -> Vec3 {
        let gp = self.global_position();
        let gr = self.global_rotation();
        let gs = self.global_size();
        let scaled = Vec3::new(gs.x * local_pos.x, gs.y * local_pos.y, gs.z * local_pos.z);
        let q = Quat::from_euler(glam::EulerRot::YXZ, gr.y, gr.x, gr.z);
        gp + q * scaled
    }

    /// 将全局坐标转为局部坐标
    pub fn global_position_to_local(&self, global_pos: Vec3) -> Vec3 {
        let gp = self.global_position();
        let gr = self.global_rotation();
        let gs = self.global_size();
        let diff = global_pos - gp;
        let q = Quat::from_euler(glam::EulerRot::YXZ, gr.y, gr.x, gr.z);
        let rotated = q.inverse() * diff;
        Vec3::new(rotated.x / gs.x, rotated.y / gs.y, rotated.z / gs.z)
    }

    /// 更新节点状态（检查 existenceReference 存活）
    pub fn update_node(&mut self) {
        if let Some(r) = &self.existence_reference {
            if !r.alive { self.alive = false; }
        }
    }

    /// 销毁节点
    pub fn destroy(&mut self) { self.alive = false; }
}

impl Default for Node {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_default() {
        let n = Node::new();
        assert!(n.alive);
        assert_eq!(n.global_position(), Vec3::ZERO);
        assert_eq!(n.global_rotation(), Vec3::ZERO);
        assert_eq!(n.global_size(), Vec3::ONE);
    }

    #[test]
    fn test_node_position() {
        let n = Node { position: Vec3::new(1.0, 2.0, 3.0), ..Node::new() };
        assert_eq!(n.global_position(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_node_destroy() {
        let mut n = Node::new();
        assert!(n.alive);
        n.destroy();
        assert!(!n.alive);
    }

    #[test]
    fn test_node_reference_chain() {
        let parent = Box::new(Node { position: Vec3::new(10.0, 0.0, 0.0), ..Node::new() });
        let child = Node {
            position: Vec3::new(5.0, 0.0, 0.0),
            position_reference: Some(parent),
            ..Node::new()
        };
        // 子节点的全局位置 = 父节点全局位置 + 子节点局部位置
        // 由于父节点的 position_reference 为 None，global_position = parent.position = (10,0,0)
        // child.local_position_to_global(child.position) = parent_pos + child_pos_scaled = (10,0,0) + (5,0,0) = (15,0,0)
        let gp = child.global_position();
        assert!((gp.x - 15.0).abs() < 0.01);
        assert!((gp.y).abs() < 0.01);
    }
}
