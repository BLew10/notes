# Functions

- A function is a block of code we can execute by calling on it at a later time. We often end up writing functions when there are actions we want to perform repeatedly without having to write the entire logic for that action over and over again.

-  To execute a function, we invoke it. A function is invoked when we call its name and pass along any necessary arguments. Invoking, or calling, a function executes a block of code and usually gives back some kind of output. Some functions take no input and some functions have no output. Even though a function doesn't return anything, it can do something inside the block of code that alters a program.

- Let's look at an example of a function in C# to learn more:

```csharp
static void SayHello()
{
    Console.WriteLine("Hello how are you doing today?");
}
```

- This may look a little different than what you're used to if you've written functions in other languages before. So let's break down what's going on here piece by piece:

## ``static``

- This keyword controls how we can invoke a function in our code. A static function can be called within the scope of its definition or by calling it on the class it's defined in, while a non-static method has to be called on an instance of an object. We'll talk more about instantiating objects when we learn about classes. For now, all of our functions will be static.

## ``void``
- Just like how when you declare any variable in C# you must also declare its type, functions follow the same rule. We must declare the return type of our function at the moment we create it.

- However, you will notice that the above function lacks a return statement. It does not actually return anything. Since it returns "nothing" we tell our program that it will return "void".

- If, for example, we wrote a function that adds two integers together and returns the sum, our function would replace "void" with "int" to denote that we are returning an integer.

```csharp
static int AddNumbers(int a, int b)
{
    return a + b;
}
```

- We must declare a return type for every function. Failure to do so will garner an error from your compiler.

## ``SayHello()``
- This is the most familiar part of the function to other languages. This is simply the name we have assigned to the function for when we go to invoke it. Speaking on invoking...

## Invoking a Function
 - A new function can be run by calling its name and passing in the necessary input. In this case, the function requires no input so we won't pass anything in. This function doesn't return anything, but it will have the side effect of printing something to the terminal console.
```csharp
static void SayHello()
{
    Console.WriteLine("Hello how are you doing today?");
}
// See how we called the name of the function followed by ()
// This must be done OUTSIDE the function you created
// ...unless you like infinite loops or are writing a recursive function
SayHello();
```

## Function Parameters
- We can get further functionality from our functions by passing data to be used or manipulated in some way into our function. Defining what kind of input we want is known as declaring a parameter.

- Let's declare a parameter called "firstName" that will be of type string. Our function will still not return anything, but its output will be a little bit more personalized thanks to the parameter. Note that we need to not only declare a parameter name but also specify the parameter's type.
```csharp
// Notice how we specify that we will take in a string called firstName within the parentheses
static void SayHello(string firstName)
{
    // Once inside the function, we treat firstName like any other variable
    // The exact value of this variable will be determined only after we call it
    // Think of the variable name as a placeholder for a value we do not know yet
    Console.WriteLine($"Hello, {firstName}, how are you doing today?");
}
```

- We can invoke this function by calling its name and passing in the correct number of arguments.

``SayHello("Andrew");``

 - Wait, what's the difference between a parameter and an argument? These two words get mixed up a lot in programming. In this example, "firstName" is a parameter while "Andrew" is an argument. We define parameters (give a name to a variable) when we create our functions. We pass arguments (give that variable a value) when we call that function with the intent to use it.

## Multiple parameters
We can pass as many parameters as we'd like into our function as long as we set it up properly.
```csharp
// Notice how new parameters are separated by commas and their types are defined
static void SayHello(string firstName, string lastName)
{
    Console.WriteLine($"Hello, {firstName} {lastName}, how are you doing today?");
}
SayHello("Luke", "Skywalker");
```
 ## Default Parameter Values
It's good to have default parameter values sometimes. Let's revisit our SayHello function. If we don't know the name of the person that we are greeting, we can have a default value. For example, we can just say "Hey, buddy."

```csharp
static void SayHello(string firstName = "buddy")
{
    Console.WriteLine($"Hey, {firstName}");
}
// We can call it without providing any arguments and the default value will be used...
SayHello();
// ...or we can call it with an argument and that argument's value will be used
SayHello("Yoda");
```

 ## Return
-  As mentioned previously, we must declare a return type when we create our function. If nothing is to be returned, we pass in "void", but if something were to be returned we would specify what data type that returned item will be.

- As an example, let's modify our SayHello() function to now return our greeting rather than simply logging it to our terminal. This means our return type will no longer be "void", it will now be "string", since we are returning a string as the last line in our code.

```csharp
static string SayHello(string firstName = "buddy")
{
    return $"Hey {firstName}";
}
string greeting;
greeting = SayHello();
Console.WriteLine(greeting);

```

This version of SayHello specifies string as the return type. We call SayHello and store the value it returns in the greeting variable, which is also of type string. Finally, we can Console.WriteLine the result to see that it worked.

## Reflection
- What are the various necessary components needed to declare a function in C#?
    - 
- What is the major difference between a function with a return statement and one that does not have a return statement?
- What is "void" used for?
    - Void is used to state that we will not be returning a data type
- What is the difference between an argument and a parameter?
    - A parameter is what are function is taking in as input and acts as a placeholder for where the input will be used
    - An argument is what is the input passed in when the function is being called