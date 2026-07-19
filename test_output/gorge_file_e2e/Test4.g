class Test4
{
	int counter;
	int increasment;
	int selfIncreasement = -1;

	Test4(int increasment)
	{
		this.increasment = increasment;
	}
	
	void SelfIncreasement()
	{
		counter = counter + selfIncreasement;
	}

	static int DoTest()
	{
		Test4 t = new Test4(2);
		t.counter = 0;
		for(int j = 0; j < 100000000; j = j + 1)
		{
			t.counter = t.counter + t.increasment;
			t.SelfIncreasement();
		}
		return t.counter;
	}
}