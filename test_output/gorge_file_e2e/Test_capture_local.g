class TestCaptureLocal
{
	static int DoTest()
	{
		int x = 10;

		delegate<int:int> d = int:(int i)->
		{
			return x + i;
		};

		return d(5);
	}
}
