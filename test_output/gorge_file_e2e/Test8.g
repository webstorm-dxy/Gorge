class Test8
{
	static int DoTest()
	{
		int i = 0;
		
		Test8N t = new Test8N(2, 3);
		t.SetGorgeField(1);
		t.SetCSharpField(6);
		
		Test8A t2 = new Test8A(4,6,7);

		for(int j = 0; j < 10000000; j = j + 1)
		{
			i = Test8N.Add(i - Test8N.GetConst() - t.GetGorgeField() - Test8N.Echo(4), t.GetCSharpField()) + t2.gorgeField + t2.GetCSharpField() - t2.GetGorgeField();
		}

		return i;
	}
}

native class Test8N
{
	int gorgeField;
	
	// return gorgeField
	int GetGorgeField();

	// gorgeField = i
	void SetGorgeField(int i);

	// return [HiddenFieldInC#]
	int GetCSharpField();

	// [HiddenFieldInC#] = i
	void SetCSharpField(int i);

	Test8N(int gorgeField, int cSharpField);

	// return 1
	static int GetConst();

	// return i
	static int Echo(int i);

	// return a + b
	static int Add(int a,int b);
}

class Test8A : Test8N
{
	int subClassField;

	Test8A(int a, int b, int c) : super(a + b, a - b)
	{
		subClassField = c;
	}

	int GetGorgeField()
	{
		return subClassField;
	}

	void SetGorgeField(int i)
	{
		subClassField = i;
	}
}