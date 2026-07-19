class Test12
{
	static int DoTest()
	{
		delegate<int:int> d1 = int:(int i)->
		{
			delegate<int:int> d2 = int:(int j)->
			{
				return j - 1;
			};
			return d2(i) + 1;
		};

		int i = 0;

		for(int j = 0; j < 10000000; j = j + 1)
		{
			i = i + d1(1);
		}

		return i;
	}
}