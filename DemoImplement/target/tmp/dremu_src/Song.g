[
    string displayName = "音频"
]
@AudioStaff
class Song
{
    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 0.373,
        }
    ]
    @Song
    static GorgeFramework.AudioAsset^ GetSong()
    {
        return GorgeFramework.AudioAsset : {
            name : "audio:Song",
        };
    }


}
