// Phase G4 端到端验证：@Inject 注解自动派生注入器字段 + metadata
class Note
{
	float hitTime;
	float duration;

	[ float defaultValue = 0.0, string type = "basic", int order = 1, string displayName = "hit" ]
	@Inject
	float hitTime = ^hitTime;

	@Inject
	float duration = ^duration;
}

class Program
{
	static float test() { return 0.0; }
}
