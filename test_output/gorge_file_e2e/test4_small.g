class Test4Small
{
	int counter;
	int increasment;
	int selfIncreasement = -1;

	Test4Small(int increasment)
	{
		this.increasment = increasment;
	}
	
	void SelfIncreasement()
	{
		counter = counter + selfIncreasement;
	}

	static int DoTest()
	{
		Test4Small t = new Test4Small(2);
		t.counter = 0;
		for(int j = 0; j < 1000; j = j + 1)
		{
			t.counter = t.counter + t.increasment;
			t.SelfIncreasement();
		}
		return t.counter;
	}
}
