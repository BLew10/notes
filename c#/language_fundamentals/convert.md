# Conversion and Type Casting
```csharp
string aNumber = "7";
int converted = Convert.ToInt32(aNumber);
Console.WriteLine(14 + converted);// should print 21

string aDecimal = "3.14";
double convertDec = Convert.ToDouble(aDecimal);
Console.WriteLine(1.8 + convertDec); // should print 4.94
```

## Input 
```csharp
Console.WriteLine("Type something, then hit enter: ");
string InputLine = Console.ReadLine();
Console.WriteLine($"You wrote: {InputLine}");
```


# Type Casting
To add to all the talk about inputs and converting values on the previous page, we are going to cover one more series of concepts for converting values known as type casting.

We only use type casting when we are already 100% certain about what we are working with, so it is not a good option for handling input received from ReadLine. In fact, it will not work with values received from ReadLine because it cannot convert strings to numbers. What type casting does is convert similar or ambiguous data types into other types.

What does that look like? Here are some examples:

Implicit Casting
```csharp
int IntegerValue = 3;
double DoubleValue = IntegerValue;
```
Implicit casting allows us to directly convert a variable from one type to another as long as the conversion does not include any loss of information. For example, converting a whole number into a decimal number. We do not lose any data because converting the whole number 3 into a decimal would at best become 3.0. If we tried this the opposite way (double into an integer) we would get an error message.

Explicit Casting
```csharp
double DoubleValue = 3.14159;
// Notice the need to put (int) in front of the value to cast it to integer
int IntegerValue = (int)DoubleValue;
```
On the other side, if we did in fact want to convert a double into an integer, we would use explicit casting. We do this with the understanding that this will cause a loss of information since an integer would not store any values right of the decimal place.

Unboxing
One "workaround" to needing to explicitly declare a data type upon creation of a variable is to "box" the variable into a generic data type. In this case, we can use object as our generic type.
```csharp
object BoxedData = "This is clearly a string";
```
Then if we would like to figure out what type of value our stored object is, we can use the is keyword in a conditional statement.
```csharp
// Make sure it is the type you need before proceeding
if (BoxedData is int) {
    // This shouldn't run
    Console.WriteLine("I guess we have an integer value in this box?");
}
if (BoxedData is string) {
    Console.WriteLine("It is totally a string in the box!");
}
```
Please note, it is not good practice to use "object" as a data type!! This is an example and we will show the true power of "is" in the coming sections when we're dealing with actual objects!

Safe Explicit Casting
```csharp
object ActuallyString = "a string";
string ExplicitString = ActuallyString as string;
 
// THIS WON'T WORK!!
object ActuallyInt = 256;
int ExplicitInt = ActuallyInt as int;
```
If we did want a little bit of security back in our type conversions, we could safely use explicit casting using the as keyword. Unlike explicit casting, if a safe cast fails it will simply return a null value rather than throwing an error. The catch is that this means the values we safely cast can only be of types that can be nullable. Strings are an example of types that can hold "null" as a value. Integers are examples of types that cannot hold "null" and therefore cannot work with safe casting.