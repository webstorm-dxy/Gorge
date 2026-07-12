class C
{
	int addTwo(int a, int b) { return a + b; }
}
class P
{
	static int Run() { C c = new C(); return c.addTwo(2, 3); }
}
