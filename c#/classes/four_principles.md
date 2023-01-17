# Four Principles of OOP
```csharp
abstract class Vehicle
{
    protected int wheels;
    protected string color;

    public Vehicle(int wheels, string color)
    {
        this.wheels = wheels;
        this.color = color;
    }

    public abstract void StartEngine();

    public void PrintDetails()
    {
        Console.WriteLine("This vehicle has " + wheels + " wheels and is " + color + " in color.");
    }
}

class Car : Vehicle
{
    private int doors;

    public Car(string color, int doors) : base(4, color)
    {
        this.doors = doors;
    }

    public override void StartEngine()
    {
        Console.WriteLine("The car's engine is now running.");
    }

    public void HonkHorn()
    {
        Console.WriteLine("Honk honk!");
    }
}

class Bike : Vehicle
{
    public Bike(string color) : base(2, color) { }

    public override void StartEngine()
    {
        Console.WriteLine("The bike's engine is now running.");
    }

    public void RingBell()
    {
        Console.WriteLine("Ring ring!");
    }
}

class Program
{
    static void Main(string[] args)
    {
        Car car = new Car("Red", 4);
        car.PrintDetails();
        car.StartEngine();
        car.HonkHorn();

        Bike bike = new Bike("Blue");
        bike.PrintDetails();
        bike.StartEngine();
        bike.RingBell();

        Vehicle vehicle = car;
        vehicle.PrintDetails();
        vehicle.StartEngine();
    }
}
```
## This example demonstrates:

- **Encapsulation:** the member variables of the Vehicle class, such as wheels and color, are protected and can only be accessed by methods within the class.
- **Inheritance:** the Car and Bike classes inherit from the Vehicle class, inheriting its properties and methods.
- **Polymorphism:** The Main method creates objects of Car and Bike classes but assigns them to Vehicle type variable and calling their methods using this variable.
- **Abstraction:** the Vehicle class is declared as abstract, which means that it cannot be instantiated on its own. Instead, it serves as a blueprint for other classes to inherit from, and it's methods are abstract and need to be implemented by child classes.