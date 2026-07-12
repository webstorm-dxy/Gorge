// GorgeFramework 端到端示例：调用 native 类 Math 的静态方法。
//
// native class 存根声明的方法顺序/签名必须与 GorgeFramework/src/math.rs 中
// #[gorge_native_impl] 的声明顺序完全一致（对齐规则 M1）。
//
// 运行：
//   1. 在主 workspace 编译：gorgec examples/native_math.g -o native_math.gorge
//   2. 在框架 workspace 执行：gorge_runner native_math.gorge Program.DoTest
//   预期输出：DoTest -> 返回 (float): 4
native class Math
{
	static float abs(float f);
	static float sqrt(float f);
}

class Program
{
	static float DoTest()
	{
		float a = Math.sqrt(16.0);
		return a;
	}
}
