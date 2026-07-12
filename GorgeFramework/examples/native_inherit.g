// Phase F2 端到端验证：编译类继承 native 类。
// Vector2 是 native 存根声明（真实实现由 gorge_framework 提供）。
// 存根方法顺序须与 vector2.rs 的 #[gorge_native_impl] 一致：
//   distance(静态0) scale(静态1) magnitude(实例2) get_x(实例3) get_y(实例4) lerp(静态5)
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

// 编译类继承 native 类 Vector2，新增字段 label 与方法
class Labeled : Vector2
{
	int label;

	Labeled(float x, float y, int label) : super(x, y)
	{
		this.label = label;
	}

	int getLabel()
	{
		return label;
	}
}

class Program
{
	// 调用继承自 native Vector2 的实例方法 get_x：Labeled(3,4,7).get_x() = 3
	static float TestInheritedNative()
	{
		Labeled p = new Labeled(3.0, 4.0, 7);
		return p.get_x();
	}

	// 调用子类自己的方法读子类字段：getLabel() = 7
	static int TestOwnField()
	{
		Labeled p = new Labeled(3.0, 4.0, 7);
		return p.getLabel();
	}
}
