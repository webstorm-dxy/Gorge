// Phase E3 端到端验证：强制类型转换 (T)expr。
class Program
{
	// (int)3.7 = 3（float→int 截断）
	static int TestFloatToInt()
	{
		float f = 3.7;
		int i = (int)f;
		return i;
	}

	// (float)5 用于浮点运算：(float)5 / 2 = 2.5
	static float TestIntToFloat()
	{
		int n = 5;
		float r = (float)n / 2.0;
		return r;
	}
}
