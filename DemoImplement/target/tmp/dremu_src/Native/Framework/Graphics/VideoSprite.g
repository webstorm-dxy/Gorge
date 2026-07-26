using Gorge;
namespace GorgeFramework;

native class VideoSprite : Node
{
    // 视频
    Video video;
    
    // 颜色
    ColorArgb color;
    
    float videoWidth;
    
    float videoHeight;
    
    VideoSprite(Video video);
    
    void SetTime(float time);
}