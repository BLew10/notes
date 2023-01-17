# Variables and Types
 - C# is ***statically typed***
    - This means we MUST declare the variable type before hand instead of letting the computer figure it out
    - This is results in an improved cpu performance due to increase in memory efficiency

    ```csharp
    string name = "brandon";
    int age  = 27;
    bool isOlderThanEighteen = true;
    double bankAccount = 100.00;
    ```
- Every line ends in a semicolon

- String Template Literals in C#
```csharp
// Variable to interpolate
string place = "Coding Dojo";
// Interpolated string, note the $
Console.WriteLine($"Hello {place}");
// Another option uses placeholders like so
// Note this does NOT have a $ at the start
Console.WriteLine("Hello {0}", place);
```

Reflection Questions:
- **What is a statically-typed language?**
    - **Answer:** A Statically typed language is one where the variable must of a type declared upon initialization 
    - *Chat GPT:*  A statically-typed language is a programming language in which the type of a variable is determined at compile-time, rather than at runtime. This means that the type of a variable must be explicitly declared and cannot change during the execution of the program. 
- **What is one major difference between a statically-typed language and a dynamically-typed one?**
    - **Answer:** A major difference would be explicit data-typing. Statically type langs require data type to be explicit whereas dynamically dont. Dynamic ones assume based off what the variable is assigned to
    - *Chat GPT:*  There are several major differences between these two types of languages:

        - **Type Checking:** In a statically-typed language, the compiler checks the types of variables and expressions at compile-time, and will not allow the program to run if there are any type errors. In a dynamically-typed language, type errors are not caught until runtime, which can make debugging more difficult.

        - **Performance:** Statically-typed languages typically have better performance than dynamically-typed languages because type errors are caught at compile-time and the compiler can optimize the code.

        - **Readability and maintainability:** Statically-typed languages often result in more readable and maintainable code, as variable types are explicitly defined and can be easily understood by other developers. In a dynamically-typed language, variable types are not explicitly defined, which can make it more difficult to understand the code and to maintain it.

        - **Flexibility:** Dynamically-typed languages are more flexible than statically-typed languages because variables can change type during runtime, which can make it easier to write code that can handle a wide range of inputs.

        - **Learning Curve:** Statically-typed languages can have a steeper learning curve for developers who are not familiar
- **Why are statically-typed languages useful?**  
  - **Answer:** Statically typed languages are useful because they are memory efficient. The computer doesn't have to assume the type, it is explicit sot the computer does not have to remember types or be able to be versatile so that that variable can store all types. It saves space cause it knows that a variable will be a certain type
  - *Chat GPT:* 
    - This provides several benefits such as:
    - The compiler can catch type-related errors at compile-time, rather than at runtime. This makes it easier to detect and fix bugs in the code.
    - The compiler can optimize the code more effectively because it knows the type of variables at compile-time.
    - It allows for better code maintainability, as it's more explicit about the type of data and operations that can be performed on that data.
- **What is the first thing you need to do when declaring any variables in C#?**
    - **Answer:** Define the type
    - *Chat GPT:*  