class TestD2D3
{
	int d3Base = 2;

	static int DoTest()
	{
		return (new TestD2D3()).InstanceDoTest();
	}

	int InstanceDoTest()
	{
		int d2Base = 1;

		delegate<int:int> d2 = int:(int i)->
		{
			return d2Base + i;
		};

		delegate<int:int> d3 = int:(int i)->
		{
			return d3Base + i;
		};

		return d2(1) + d3(2);
	}
}
