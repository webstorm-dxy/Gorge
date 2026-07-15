// 端到端验证：算术运算完整对齐 C#（类型提升 + 字符串拼接 + 浮点取模 + 相等提升）。
class Program
{
	// int + float → float：1 + 2.5 = 3.5
	static float TestIntPlusFloat()
	{
		int a = 1;
		float b = 2.5;
		return a + b;
	}

	// int + string → 字符串拼接："count=" + 42 = "count=42"
	static string TestIntPlusString()
	{
		int n = 42;
		return "count=" + n;
	}

	// float % float → FloatMod：5.5 % 2.0 = 1.5
	static float TestFloatMod()
	{
		float a = 5.5;
		float b = 2.0;
		return a % b;
	}

	// int % int → IntMod：7 % 3 = 1
	static int TestIntMod()
	{
		return 7 % 3;
	}

	// int == float → 提升后比较：1 == 1.0 → true
	static bool TestIntEqualsFloat()
	{
		int a = 1;
		float b = 1.0;
		return a == b;
	}

	// int < float → 提升后比较：3 < 3.5 → true
	static bool TestIntLessFloat()
	{
		int a = 3;
		float b = 3.5;
		return a < b;
	}
}
