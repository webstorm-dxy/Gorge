class Test9
{
	static int DoTest()
	{
		int i = 0;
		
		Test9A^ injectorA = Test9A : {
			nativeObjectField : Test9NInner : {:},
			gorgeIntField : 3
		};

		Test9A t = new injectorA();

		for(int j = 0; j < 10000000; j = j + 1)
		{
			// i - 1 - 2 + 3 - 4 + 5
			i = i - t.nativeIntField - t.nativeObjectField.innerField + t.gorgeIntField - t.gorgeObjectField.innerFieldA + t.gorgeObjectField.innerFieldB;
		}

		return i;
	}
}

native class Test9NInner
{
	// 2
	[auto defaultValue]
	@Inject
	int innerField;

	Test9NInner();
}

native class Test9N
{
	// 1
	[auto defaultValue]
	@Inject
	int nativeIntField;
	
	@Inject<Test9NInner^>
	Test9NInner nativeObjectField;

	Test9N();
}

class Test9Inner
{
	[auto defaultValue = 4]
	@Inject
	int innerFieldA = ^innerFieldA;

	[auto defaultValue = 6]
	@Inject
	int innerFieldB = ^innerFieldB;
	
	Test9N()
	{
	}
}

class Test9A : Test9N
{
	@Inject
	int gorgeIntField = ^gorgeIntField;

	[auto defaultValue = Test9Inner : {innerFieldB : 5}]
	@Inject<Test9Inner^>
	Test9Inner gorgeObjectField = new ^gorgeObjectField();
	
	Test9A() : super()
	{
	}
}