// Phase D 端到端验证：break/continue 多层与按类型离块。
// 注意 Gorge 语义（对齐 C#）：if/else 也算一层，
// 因此从 if 内跳出循环需用 `break while` 或把 if 计入层数（如 break 2）。
class Program
{
	// break while：i 累加到 5 时跳出 while，返回 0+1+2+3+4 = 10
	static int TestBreak()
	{
		int sum = 0;
		int i = 0;
		while (i < 100)
		{
			if (i == 5)
			{
				break while;
			}
			sum = sum + i;
			i = i + 1;
		}
		return sum;
	}

	// continue while：跳过 i==2，返回 0+1+3+4 = 8
	static int TestContinue()
	{
		int sum = 0;
		int i = 0;
		while (i < 5)
		{
			int cur = i;
			i = i + 1;
			if (cur == 2)
			{
				continue while;
			}
			sum = sum + cur;
		}
		return sum;
	}

	// break 2 从 if 内跳出：if 算一层，while 算一层，break 2 跳出内层 while。
	// 外层 a=0,1,2；内层 b=0,1,2。a==1 且 b==1 时 break 2（跳出 if 和内层 while）。
	// a=0: b=0,1,2 累加 0+1+2=3；a=1: b=0 累加 10，b==1 时跳出内层；a=2: b=0,1,2 累加 20+21+22=63
	// 结果 = 3 + 10 + 63 = 76
	static int TestBreakInnerFromIf()
	{
		int sum = 0;
		int a = 0;
		while (a < 3)
		{
			int b = 0;
			while (b < 3)
			{
				if (a == 1)
				{
					if (b == 1)
					{
						break 3;
					}
				}
				sum = sum + a * 10 + b;
				b = b + 1;
			}
			a = a + 1;
		}
		return sum;
	}

	// break while 从 switch 内跳出外层 while：i=0 命中 case 0 → break while
	static int TestBreakWhile()
	{
		int sum = 100;
		int i = 0;
		while (i < 10)
		{
			switch (i)
			{
				case 0:
					break while;
			}
			sum = sum + 1;
			i = i + 1;
		}
		return sum;
	}
}
