class Test6
{
	static int DoTest()
	{
		Test6A ta = new Test6A(1);
		Test6B tb = new Test6B(1);
		Test6C tc = new Test6C(1);
		
		Test6A tab = new Test6B(1);
		Test6A tac = new Test6C(1);

		Test6I tia = new Test6A(1);
//		Test6I tib = new Test6B(1);
//		Test6I tic = new Test6C(1);
		
		int i = 0;
		
		for(int j = 0; j < 10000000; j = j + 1)
		{
			i = i + tia.GetValue();
		}
		
		return i;
	}
}

interface Test6I
{
	int GetValue();
}

class Test6A :: Test6I
{
	int valueA;
	
	Test6A(int value)
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

class Test6B : Test6A
{
	int valueB;	

	Test6B(int value) : super(value + 1)
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

class Test6C : Test6B
{
	int valueC;

	Test6B(int value) : super(value + 1)
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