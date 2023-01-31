# Four Principles of OOP

## **Why do we need OOP?**
- It helps to think in terms of real world objects  
    - This helps us create real solutions around real world objects and create connections between them
    - Give an example:
        - Hospital
            - create a hospital class. It has a name, CMO, list of patients, list of doctors, list of departments, trauma level
        - Doctors
            - List of patients, name, department, MD or DO, 
            - Scope of practice (methods), procedures they can perform
            - Ability to order
        - Patients
            - List of doctors, list of medications, name address

## **What are the pillars of OOP?**
- Abstraction
    - Show what is only necessary
- Polymorphism
    - An object can act differently under different conditions
    - Ex: Doctor can make perform a procedure
        - If he is a PCP he can do X, if he is a surgeon he can perform Y
- Encapsulation
    - Ability to hide complexity?
- Inheritance
    - Parent Child relationship

## **What is a Class and Object ?**
- A class a blueprint for an object whereas an instance of that class is an object
## **Abstraction VS Encapsulation ?**
- They complement each other
- Abstraction is a design thought process. Not everything needs to be accesible. You want the functionality but it does not need to be accessible.
    - You think, what *has* to be shown
- Encapsulation is the implementation of Abstraciton. It is using the access modifiers to protect information
## **Explain Inheritance ?**
- Defines a parent child relationship between two classes where child has access to all of the parent but not the other way around
    - Example: Employee -> Doctor, Nurses
## **Explain Virtual Keyword ?**
- This is used in the parent class to determine which class can be overriden by the child class
## **What is Overriding ?**
- Overriding is changing the functionality of a parent method in a child class
    - Say the parent class Employee is MakeOrders()
    ```csharp
    public virtual void MakeOrders(){
        AbilityToMakeOrders();
    }
    //doctor child
    public override void MakeOrders(){
        base.AbilityToMakeOrders();
        OrderXRays();
    }
    ```


## **Explain Overloading ?**
- The ability to have the same method name in the same class but with different signatures, meaning it can take in different parameters

    ```csharp
     public override void MakeOrders();
    public  void MakeOrders(string patient);
    public  void MakeOrders(string patient, string OrderType, int NumOrders);
    ```

## **Overloading VS Overriding ?**
- Overriding is the ability to change the functionality of a parent method in a chlid class using the ``virtual`` keyword and ``override`` keyword
- Overloading is the ability to make a method with the same name in the same class have different signatures thus making it have the ability to perform different given different inputs

## **What is Polymorphism ?**
- Ability to of an object to at differently under diffent conditions
- Cannot be implemented without a parent child relationship (inheritance)
```csharp
Employee employee = new Doctor();
employee = new Nurse();
```
## **Can Polymorphism work without Inheritance ?**
- NO
## **Explain Static VS Dynamic Polymorphism ?**
- Static polymorphism is implemented by method overloading
    - Checked during the compile time
- Dynamic polymoprhism is implemented via method overriding
    - Run time
    - It won't know which method to use until runtime
    ```csharp
    // using logic from nurse, doct
    Employee employee = new Doctor();
    employee.MakeOrders();
    employee = new Nurse();
    employee.MakeOrders();
    ```
    - Two different ``MakeOrders()``, therefore dynamic
    - Where

    
## **Explain Operator Overloading ?**
- You can redefine operators (+, -, *, /) with different functionalities

## **How to do Custom Operator Overloading ?**
 ```csharp
//method within a class
public static [ClassName] operator +(ClassName arg1, ClassName arg2)
{
// return new ClassName(arg1.field + arg2.field)
}
```
## **Why do we need Abstract Classes ?**
- A abstract class is partially defined parent class, the rest of defining goes to the child class
```csharp
public abstract class Customer {
    public string name {get; set;}
    public string address {get; set;}
    public string productName {get; set;}
    public decimal productAmount {get; set;}
    public abstract decimal CalculateDiscount();
}

public class Gold : Customer {
    public override CalculateDiscount(){
        productAmount-=10;
    }
}
public class Silver : Customer {
    public override CalculateDiscount(){
        productAmount-=10;
    }
}
```
- It is strictly used to be passed into children. You cannot invoke an instance of an abstract class
## **Are Abstract methods Virtual ?**
- YES
## **Can we create an instance of Abstract Classes ?**
- NO
## **Is it compulsory to implement Abstract Methods ?**
- YES, they must be implemented in the Child Class
## **Why simple base class cannot replace Abstract Classes?**
- You must implement logic in a simple base class
## **Explain Interface and Why do we need it ?**
- An interface is a contract meaning that a class that uses an interface must implement the fields and methods associated with that interfac
- A class can have multiple interfaces but not multiple parent classes
- Easier to control change management, impact analysis and changes that would break your code. C# makes you aware that you are not following the strict guidelines that interfaces provide
## **Can we write logic in Interface ?**
- NO
## **Can we define methods as private in Interface ?**
- No, ALL PROPERTIES ARE PUBLIC
## **To change Interface what's the best practice ?**
- Best practice is to create another Interface
## **Explain Multiple Inheritance in Interfaces ?**
- Helps to add new methods withou affecting the older interfaces
## **Explain Interface Segregation Principle ?**
- Don't have unnecessary methods 
## **Can we create instance of Interface ?**
- NO
## **Can we do multiple inheritance of Interface ?**
- Yes
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
