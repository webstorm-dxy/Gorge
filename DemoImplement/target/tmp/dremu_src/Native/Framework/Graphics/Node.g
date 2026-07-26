using Gorge;
namespace GorgeFramework;

// 图形节点
native class Node
{
    // 是否存活
    bool alive;
    
    // 存在性参考节点
    Node existenceReference;
    
    // 相对位置
    Vector3 position;
    
    // 相对位置参考节点
    Node positionReference;
    
    // 相对角度
    // TODO 暂时使用欧拉表示，可能考虑转为四元数
    Vector3 rotation;
    
    // 相对角度参考节点
    Node rotationReference;
    
    // 相对尺寸
    Vector3 size;
    
    // 相对尺寸参考节点
    Node sizeReference;
    
    Node();
    
    Vector3 GlobalPosition();
    
    Vector3 LocalPositionToGlobalPosition(Vector3 position);
    
    Vector3 GlobalPositionToLocalPosition(Vector3 position);
}