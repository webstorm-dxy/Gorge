class DebugAdd4
{
	static int Add(int a, int b)
	{
		return a + b;
	}
	static int DoTest()
	{
		return DebugAdd4.Add(5, 3);
	}
}
