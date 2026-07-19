class Test10
{
	static int DoTest()
	{
		int i = 0;
		
		int[] arrayA = new int[2];
		arrayA[0] = 1;
		arrayA[1] = 2;
		
		int[]^ listB = int : { 3, 4, 5 };
		int[] arrayB = new listB[3];
		
		for(int j = 0; j < 10000000; j = j + 1)
		{
			// i - 1 - 2 + 3 - 4 + 5
			i = i - arrayA[0] - arrayA[1] + arrayB[0] - arrayB[1] + arrayB[2];
		}

		return i;
	}
}