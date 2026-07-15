// Phase G3 端到端验证：带注入器的构造 new T(args) :{ field: val }
// Vector2 是 native 类，其 injector 字段 x,y 已在 .g 存根声明中定义。
// 构造方法体通过 ^x, ^y 读取注入器字段（G1），注入器从 :{...} 创建（G2+G3）。
native class Vector2
{
	float x; float y;
	Vector2(float x, float y);
	static float distance(Vector2 v1, Vector2 v2);
	static Vector2 scale(Vector2 v1, Vector2 v2);
	float magnitude(); float get_x(); float get_y();
	static Vector2 lerp(Vector2 a, Vector2 b, float t);
}

class Program
{
	// 构造 Vector2(0,0):{x:3,y:4}：注入器构造先 FieldInitialize(注入器) 写 x=3,y=4，
	// 随后构造体用显式参数 (0,0) 覆盖 → get_x()=0（对齐 C# 注入器构造：参数覆盖注入器）。
	// 注入器覆写在「仅注入器、无覆盖参数」的构造路径才成为最终值。
	static float TestInjectorX()
	{
		Vector2 v = new Vector2(0.0, 0.0) :{ x : 3.0, y : 4.0 };
		return v.get_x();
	}

	static float TestNoInjector()
	{
		Vector2 v = new Vector2(5.0, 6.0);
		return v.get_x();
	}
}
