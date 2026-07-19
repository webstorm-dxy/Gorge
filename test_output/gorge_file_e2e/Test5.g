class Test5
{
	static int DoTest()
	{
		Test5A ta = new Test5A(1);
		Test5B tb = new Test5B(1);
		Test5C tc = new Test5C(1);
		
		Test5A tab = new Test5B(1);
		Test5A tac = new Test5C(1);
		
		int i = 0;
		
		for(int j = 0; j < 10000000; j = j + 1)
		{
			i = i + tb.valueA + tb.GetValueA();
		}
		
		return i;
	}
}

class Test5A
{
	int valueA;
	
	Test5A(int value)
	{
		valueA = value;
	}

	int GetValue()
	{
		return valueA;
	}

	int GetValueA()
	{
		return valueA;
	}
}

class Test5B : Test5A
{
	int valueB;	

	Test5B(int value) : super(value + 1)
	{
		valueB = value;
	}

	int GetValue()
	{
		return valueB;
	}

	int GetValueB()
	{
		return valueB;
	}
}

class Test5C : Test5B
{
	int valueC;

	Test5B(int value) : super(value + 1)
	{
		valueC = value;
	}

	int GetValue()
	{
		return valueC;
	}

	int GetValueC()
	{
		return valueC;
	}
}