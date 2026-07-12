// Phase G2 简单端到端：创建注入器常量并验证 bytecode 包含它。
class Point
{
	float x; float y;
	Point() { x = 0.0; y = 0.0; }
}

class Program
{
	static float Test()
	{
		Point inj = Point : { x : 3.0, y : 4.0 };
		return 0.0;
	}
}
