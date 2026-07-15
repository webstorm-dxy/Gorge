class C
{
	int add2(int a, int b) { return a + b; }
}
class P
{
	static int Run()
	{
		C c = new C();
		return c.add2(2, 3);
	}
}
