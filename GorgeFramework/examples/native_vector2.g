// GorgeFramework 端到端示例：native 类 Vector2 的构造、静态方法、实例方法。
//
// 存根声明的方法顺序必须与 GorgeFramework/src/vector2.rs 的 #[gorge_native_impl]
// 完全一致（对齐规则 M1）：distance(静态0)、scale(静态1)、magnitude(实例2)、
// get_x(实例3)、get_y(实例4)、lerp(静态5)。缺一个都会导致方法编号错位。
//
// 运行：
//   gorgec examples/native_vector2.g -o native_vector2.gorge
//   gorge_runner native_vector2.gorge Program.TestDistance   → 5
//   gorge_runner native_vector2.gorge Program.TestMagnitude  → 10
//   gorge_runner native_vector2.gorge Program.TestLerpX      → 5   （B-2 混合类型参数）
native class Vector2
{
	float x;
	float y;

	Vector2(float x, float y);

	static float distance(Vector2 v1, Vector2 v2);
	static Vector2 scale(Vector2 v1, Vector2 v2);

	float magnitude();
	float get_x();
	float get_y();

	static Vector2 lerp(Vector2 a, Vector2 b, float t);
}

class Program
{
	static float TestDistance()
	{
		Vector2 a = new Vector2(0.0, 0.0);
		Vector2 b = new Vector2(3.0, 4.0);
		return Vector2.distance(a, b);
	}

	static float TestMagnitude()
	{
		Vector2 v = new Vector2(6.0, 8.0);
		return v.magnitude();
	}

	// B-2 混合类型参数：lerp((0,0),(10,20),0.5) = (5,10)，返回 x = 5
	static float TestLerpX()
	{
		Vector2 a = new Vector2(0.0, 0.0);
		Vector2 b = new Vector2(10.0, 20.0);
		Vector2 r = Vector2.lerp(a, b, 0.5);
		return r.get_x();
	}
}
