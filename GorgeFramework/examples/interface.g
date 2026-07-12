// Phase F1 端到端验证：接口方法实现映射与接口分派。
interface IShape
{
	int area();
}

class Rect :: IShape
{
	int w;
	int h;

	Rect(int w, int h)
	{
		this.w = w;
		this.h = h;
	}

	int area()
	{
		return w * h;
	}
}

class Program
{
	// 通过接口类型变量调用 area()：Rect(3,4).area() = 12
	static int TestInterface()
	{
		IShape s = new Rect(3, 4);
		return s.area();
	}
}
