class Test7Small
{
	int d3Base = 2;

	Test7Small()
	{
	}

	static int DoTest()
	{
		return (new Test7Small()).InstanceDoTest();
	}

	int InstanceDoTest()
	{
		delegate<int:int> d1 = int:(int i)->
		{
			return 1 + i;
		};

		int d2Base = 1;

		delegate<int:int> d2 = int:(int i)->
		{
			return d2Base + i;
		};

		delegate<int:int> d3 = int:(int i)->
		{
			return d3Base + i;
		};

		int i = 0;

		for(int j = 0; j < 2; j = j + 1)
		{
			i = i - d1(0) - d2(1) + d3(2);
		}

		return i;
	}
}
