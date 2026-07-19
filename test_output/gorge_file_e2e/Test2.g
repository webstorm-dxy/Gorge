class Test2
{
	static int DoTest()
	{
		int i = 0;
		for(int j = 0; j < 50000000; j = j + 1)
		{
			i = Test2.Add(i, 1);
		}
		return i;
	}

	static int Add(int a, int b)
	{
		return a + b;
	}
}