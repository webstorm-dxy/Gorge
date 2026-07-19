class Test3
{
	static int DoTest()
	{
		int i = 0;
		for(int j = 0; j < 500000; j = j + 1)
		{
			i = Test3.Add(i, Test3.RecursionEcho(100) - 99);
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
		return Test3.RecursionEcho(a - 1) + 1;
	}
}