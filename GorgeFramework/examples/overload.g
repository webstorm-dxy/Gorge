// Phase E4 端到端验证：方法与构造方法重载解析（按参数类型三级匹配）。
class Calc
{
	int base;

	// 构造重载 0：无参 → base=0
	Calc()
	{
		base = 0;
	}

	// 构造重载 1：(int) → base=n
	Calc(int n)
	{
		base = n;
	}

	// 方法重载：add(int) 与 add(int,int)
	int add(int a)
	{
		return base + a;
	}

	int add(int a, int b)
	{
		return base + a + b;
	}
}

class Program
{
	// 用无参构造 + add(int)：base=0, add(5)=5
	static int TestOverload1()
	{
		Calc c = new Calc();
		return c.add(5);
	}

	// 用 (int) 构造 + add(int,int)：base=100, add(2,3)=105
	static int TestOverload2()
	{
		Calc c = new Calc(100);
		return c.add(2, 3);
	}
}
