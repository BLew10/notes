# LinkedIn Questions
- What does operator overloading allow you to do?
    - Define custom functionality for common operators like addition and equality
- What is operator overloading? 
    - Operator overloading is a feature in some programming languages that allows operators (such as +, -, *, /, etc.) to have different meanings depending on the context in which they are used. In C#, this means that the behavior of an operator can be redefined for user-defined types, such as classes and structs, to allow them to work with the operator in a way that makes sense for their specific implementation.
    - For example, you can overload the + operator for a custom class that represents a complex number, so that when two complex numbers are added together, the real and imaginary parts are added separately. This allows you to use the + operator with your custom class in the same way you would use it with built-in types like ints and doubles.
    - Overloading operators can make your code more readable and easier to understand by allowing you to use operators that are familiar to programmers, rather than having to use custom methods to perform operations on your user-defined types.
    - It's worth noting that not all operators can be overloaded, and also not all languages support operator overloading.

- My understanding of it is that it allows you to create unique functionality for built in functions 

Example
```csharp
class ComplexNumber
{
    public double Real { get; set; }
    public double Imaginary { get; set; }

    public ComplexNumber(double real, double imaginary)
    {
        Real = real;
        Imaginary = imaginary;
    }

    public static ComplexNumber operator +(ComplexNumber a, ComplexNumber b)
    {
        return new ComplexNumber(a.Real + b.Real, a.Imaginary + b.Imaginary);
    }

    public static ComplexNumber operator -(ComplexNumber a, ComplexNumber b)
    {
        return new ComplexNumber(a.Real - b.Real, a.Imaginary - b.Imaginary);
    }

    public override string ToString()
    {
        return $"({Real}, {Imaginary}i)";
    }
}

ComplexNumber a = new ComplexNumber(1, 2);
ComplexNumber b = new ComplexNumber(3, 4);

ComplexNumber c = a + b;
Console.WriteLine(c); // Outputs "(4, 6i)"

ComplexNumber d = a - b;
Console.WriteLine(d); // Outputs "(-2, -2i)"

```

