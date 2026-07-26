using Gorge;
namespace GorgeFramework;

native class Environment
{    
    static Asset GetAssetByName(string assetName);
    
    static Element FindAliveLane(string typeName, string laneName);

    static Element FindAliveLane(string typeName, int laneId);
    
    static Vector3 ScreenToWorldPoint(Vector3 position);
    
    // 计分
    static void Scoring(RespondResult result);
    
    // 播放反馈音
    static void PlayRespondEffect(string name);

    // 获取视口尺寸
    static Vector2 ViewportSize();
}