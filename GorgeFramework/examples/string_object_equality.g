// 端到端验证：字符串/对象比较操作码（StringEqual/ObjectEqual/StringNotEqual/ObjectNotEqual）。
class Program
{
	// 字符串相等：两个相同字符串比较 → true
	static bool TestStringEqualTrue()
	{
		string a = "hello";
		string b = "hello";
		return a == b;
	}

	// 字符串不等：两个不同字符串比较 → true
	static bool TestStringNotEqualTrue()
	{
		string a = "hello";
		string b = "world";
		return a != b;
	}

	// 字符串相等（假）：两个不同字符串 → false
	static bool TestStringEqualFalse()
	{
		string a = "abc";
		string b = "xyz";
		return a == b;
	}

	// 对象引用相等：同一对象自身比较 → true
	static bool TestObjectEqualSelf()
	{
		Program p = new Program();
		return p == p;
	}

	// 对象不等：new 两次的对象引用不同 → true
	static bool TestObjectNotEqualDiff()
	{
		Program p1 = new Program();
		Program p2 = new Program();
		return p1 != p2;
	}

	// null == null → true
	static bool TestNullEqualsNull()
	{
		return null == null;
	}

	// null != null → false
	static bool TestNullNotEqualsNull()
	{
		return null != null;
	}
}
