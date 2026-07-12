class C
{
	int base;
	C(int n) { base = n; }
	int calc(int a, int b) { return base + a + b; }
}
class P
{
	static int Run() { C c = new C(100); return c.calc(2, 3); }
}
