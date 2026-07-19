class Test11
{
	static int DoTest()
	{
		Test11A^[]^ injectorList = Test11A^ : 
		{
			// 1
			Test11A : {value: 0},
			// 2
			Test11B : {value: 0},
			// 3
			Test11C : {value: 0},
			// 4
			Test11B : {value: 2},
			// 5
			Test11A : {value: 4},
		};

		Test11A^[] injectorArray = new injectorList[injectorList.length];

		Test11A[] objectList = new Test11A[injectorArray.length];

		for (int k = 0; k < injectorArray.length; k = k + 1)
		{
			if (injectorArray[k] == null)
			{
				objectList[k] = null;
			}
			else
			{
				objectList[k] = new injectorArray[k](1);
			}
		}

		int i = 0;

		for(int j = 0; j < 10000000; j = j + 1)
		{
			// i - 1 - 2 + 3 - 4 + 5
			i = i - objectList[0].value - objectList[1].value + objectList[2].value - objectList[3].value + objectList[4].value;
		}

		return i;
	}
}

class Test11A
{
	@Inject
	int value = ^value;

	injector Test11A(int i)
	{
		value = value + i;
	}
}

class Test11B : Test11A
{
	Test11B(int i) : super(i)
	{
		value = value + 1;
	}
}

class Test11C : Test11A
{
	Test11C(int i) : super(i)
	{
		value = value + 2;
	}
}