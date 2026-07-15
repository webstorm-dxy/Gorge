// 端到端验证：实例方法返回非 Int 类型（float/bool）不再丢失。
// 覆盖两个修复：
//   (1) VM InvokeInstance 非 Int 返回值写回（此前只写 return_int）
//   (2) codegen 比较运算结果类型恒为 Bool（此前用操作数类型，bool 读错栈槽）
//   (3) codegen 局部变量 `Type v = new T(...)` 登记 var_class（此前回退 InvokeInstance(0)）
class Box
{
	float value;

	Box(float v)
	{
		this.value = v;
	}

	// 实例方法返回 float
	float getValue()
	{
		return this.value;
	}

	// 实例方法返回 bool（比较运算结果）
	bool isPositive()
	{
		return this.value > 0.0;
	}
}

class Program
{
	// 实例方法返回 float：应为 2.5
	static float TestInstanceFloat()
	{
		Box b = new Box(2.5);
		return b.getValue();
	}

	// 实例方法返回 bool：应为 true（2.5 > 0）
	static bool TestInstanceBoolTrue()
	{
		Box b = new Box(2.5);
		return b.isPositive();
	}

	// 实例方法返回 bool：应为 false（-1.0 > 0 为假）
	static bool TestInstanceBoolFalse()
	{
		Box b = new Box(-1.0);
		return b.isPositive();
	}
}
