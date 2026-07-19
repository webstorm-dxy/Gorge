class Test1
{
	static int DoTest()
	{
		int i = 0;
		for(int j = 0; j < 50000000; j = j + 1)
		{
			i = i + 1;
		}
		return i;
	}
}