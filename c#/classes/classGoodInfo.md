- What is the difference between a class and an object in C#?
    - A class is a blueprint for an object, while an object is an instance of a class. For example:
```csharp
    class Dog {
  public string Name { get; set; }
  public int Age { get; set; }
}

Dog myDog = new Dog();
myDog.Name = "Fido";
myDog.Age = 3;
```
- What is the purpose of the "public" and "private" keywords in C# class construction?
    - The "public" keyword makes a class member accessible from anywhere, while the "private" keyword makes a class member only accessible within the class. For example:
```csharp
class Dog {
  public string Name { get; set; }
  private int Age { get; set; }
}
```
- How do you define a constructor in a C# class?
    - A constructor is a method that is called when an object is created, and is used to initialize the object's properties. For example:
```csharp
class Dog {
  public string Name { get; set; }
  public int Age { get; set; }

  public Dog(string name, int age) {
    Name = name;
    Age = age;
  }
}

Dog myDog = new Dog("Fido", 3);
```
- What is polymorphism in C#?
    - Polymorphism is the ability for a classes fields/methods or properties to work with multiple types of objects. For example:
```csharp
class Shape {
    public virtual void Draw() {
        Console.WriteLine("Drawing a shape.");
    }
}
class Circle : Shape {
    public override void Draw() {
        Console.WriteLine("Drawing a circle.");
    }
}
class Square : Shape {
    public override void Draw() {
        Console.WriteLine("Drawing a square.");
    }
}
```

- How do you define a static class in C#?
    - A static class is a class that cannot be instantiated, and can only contain static members. For example:
```csharp
static class MathHelper {
    public static double Pi = 3.14;
    public static double AreaOfCircle(double radius) {
        return Pi * radius * radius;
    }
}
```
- How do you inherit from a class in C#?
    - You can inherit from a class using the ":" operator. For example:
```csharp
class Animal {
    public string Name { get; set; }
}
class Dog : Animal {
    public int Age { get; set; }
}
```
- What is the difference between a struct and a class in C#?
    - A struct is a value type, while a class is a reference type. This means that when you pass a struct to a method, a copy of the struct is passed and any changes made within the method do not affect the original struct, while when you pass a class, a reference to the object is passed, which means any changes made within the method will affect the original object.
```csharp
struct Point {
    public int X, Y;
}
class PointClass {
    public int X, Y;
}
```
- Can you explain encapsulation in C#?
    - Encapsulation is the practice of hiding the implementation details of a class from the outside world, and only exposing a public interface. This allows for greater flexibility in making changes to the class, and also improves security.
```csharp
class Dog {
    private string _name;
    public string Name {
        get { return _name; }
        set { _name = value; }
    }
}
```
- Can you explain Inheritance in C#?
    - Inheritance is the mechanism by which one class can inherit properties and methods from a parent class. For example:
```csharp
class Animal {
    public string Name { get; set; }
    public void Move() {
        Console.WriteLine("Moving");
    }
}
class Dog : Animal {
    public int Age { get; set; }
    public void Bark() {
        Console.WriteLine("Barking");
    }
}

```
- How can you create an instance of a class in C#?
```csharp
Dog myDog = new Dog();
```