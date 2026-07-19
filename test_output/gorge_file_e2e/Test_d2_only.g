class TestD2Only
{
	static int DoTest()
	{
		return (new TestD2Only()).InstanceDoTest();
	}

	int InstanceDoTest()
	{
		int d2Base = 1;

		delegate<int:int> d2 = int:(int i)->
		{
			return d2Base + i;
		};

		return d2(3);
	}
}
