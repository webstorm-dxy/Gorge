class TestCaptureField
{
	int field = 10;

	TestCaptureField()
	{
	}

	static int DoTest()
	{
		return (new TestCaptureField()).InstanceDoTest();
	}

	int InstanceDoTest()
	{
		delegate<int:int> d = int:(int i)->
		{
			return field + i;
		};

		return d(5);
	}
}
