// B-3 端到端验证：编译类继承编译类（方法继承、重写、字段继承）。
class Animal
{
	int legs;

	Animal(int legs)
	{
		this.legs = legs;
	}

	int getLegs()
	{
		return legs;
	}

	int sound()
	{
		return 0;
	}
}

class Dog : Animal
{
	int loyalty;

	Dog(int legs, int loyalty) : super(legs)
	{
		this.loyalty = loyalty;
	}

	// 重写 Animal.sound
	int sound()
	{
		return 42;
	}

	int getLoyalty()
	{
		return loyalty;
	}
}

class Program
{
	// 通过 Dog 变量调用重写方法，期望 42
	static int TestOverride()
	{
		Dog d = new Dog(4, 100);
		return d.sound();
	}

	// 调用继承自 Animal 的 getLegs，期望 4
	static int TestInheritedMethod()
	{
		Dog d = new Dog(4, 100);
		return d.getLegs();
	}

	// 调用 Dog 自己的方法读子类字段，期望 100
	static int TestOwnMethod()
	{
		Dog d = new Dog(4, 100);
		return d.getLoyalty();
	}
}
