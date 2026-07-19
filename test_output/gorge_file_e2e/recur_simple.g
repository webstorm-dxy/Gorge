class RecursionTest
{
	static int DoTest()
	{
		return RecursionTest.RecursionEcho(5);
	}
	static int RecursionEcho(int a)
	{
		if(a == 0)
		{
			return 0;
		}
		return RecursionTest.RecursionEcho(a - 1) + 1;
	}
}
