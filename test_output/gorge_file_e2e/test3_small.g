class Test3Small
{
	static int DoTest()
	{
		int i = 0;
		for(int j = 0; j < 1000; j = j + 1)
		{
			i = Test3Small.Add(i, Test3Small.RecursionEcho(10) - 9);
		}
		return i;
	}
	static int Add(int a, int b)
	{
		return a + b;
	}
	static int RecursionEcho(int a)
	{
		if(a == 0)
		{
			return 0;
		}
		return Test3Small.RecursionEcho(a - 1) + 1;
	}
}
