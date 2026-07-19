class Test4Tiny
{
	int counter;
	int increasment;
	int selfIncreasement = -1;

	Test4Tiny(int increasment)
	{
		this.increasement = increasment;
	}
	
	void SelfIncreasement()
	{
		counter = counter + selfIncreasement;
	}

	static int DoTest()
	{
		Test4Tiny t = new Test4Tiny(2);
		t.counter = 0;
		for(int j = 0; j < 100; j = j + 1)
		{
			t.counter = t.counter + t.increasment;
			t.SelfIncreasement();
		}
		return t.counter;
	}
}
