# Interfaces
```csharp
interface ILayEggs
{
    int EggsPerBatch {get;set;}
    void LayEggs();
}
```
- Acts as a 'contract' if something is labeled with the ``ILayEggs`` Interface 
    - It must have the interface qualities

## Properties in interfaces
- You will notice that we used an auto-implemented property rather than a field in order to declare a variable for the number of eggs laid in a batch. This is required in order for interfaces to work.

## Methods in interfaces
- When creating a method that is part of an interface, we do not actually fill out what the method does. Rather, we describe that a method must exist, and it will be the responsibility of the programmer that is including this interface in a class to fill out what exactly that method will do for that particular class. Remember that interfaces often describe a unique thing that a class can do that is separate from its parent and sibling classes, so often it is to our advantage that we get to detail what this method does.


```csharp
// Adding an interface looks just like adding inheritance
public class Bird : Animal, ILayEggs
{
    public bool CanFly;
    // Our EggsPerBatch from our interface
    public int EggsPerBatch {get;set;}
    public Bird(bool canfly, string diet, int epb) : base("Bird", diet, true)
    {
        CanFly = canfly;
        EggsPerBatch = epb;
    }
    // Filling out the LayEggs method from our interface
    public void LayEggs()
    {
        Console.WriteLine($"{Name} laid {EggsPerBatch} egg(s)!");
    }
    public override void ShowInfo()
    {
        base.ShowInfo();
        Console.WriteLine($"Can Fly: {CanFly}");
    }
}
```

## Polymorphisms and Interfaces
- Remember from the previous lesson that polymorphism allows us to group alike objects together. Last time, we grouped all our Animals together under an AllAnimals List, but now we have another feature some objects have in common: the ability to lay eggs! Because of this, we can create a special grouping based on this feature where all our objects can be grouped thanks to their connection to the interface we wrote.
```csharp
Bird MyBird = new Bird(true, "Herbivore", 3);
Reptile MyReptile = new Reptile("Onmivorous", 6);
List<ILayEggs> canLayEggs = new List<ILayEggs>();
canLayEggs.Add(MyBird);
canLayEggs.Add(MyReptile);
foreach(ILayEggs e in canLayEggs)
{
    e.LayEggs();
}
```

## Difference between a Parent Class and Interface
- A parent class and an interface in C# have some similarities, but they serve different purposes and have some key differences:

- A parent class, also known as a base class or superclass, is a class from which other classes can inherit properties and methods. A class can only inherit from one parent class, but it can implement multiple interfaces.
- An interface defines a contract for a class, specifying a set of methods, properties, and events that a class must implement. It does not provide any implementation details, only the signatures of the members.
- A class can inherit the implementation of the members from its parent class, but it must provide its own implementation for the members of the interface.
- A parent class can have a constructor and can have fields (state) and the interface can't have any of that.
- Interfaces are used to define common functionality that multiple classes should implement, while a parent class is used to provide a common implementation that can be shared among multiple classes.
- In summary, a parent class is used for inheritance and implementation sharing, while an interface is used for polymorphism and contract definition.