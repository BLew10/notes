# Random Variable
```csharp
Random rand = new Random();
```
- Built in way to generate a random number

- .Next(x,y)
    - This is method built within the ``Random`` data type
    - First number is inclusive, the last is exlcusive
```csharp
    Random rand = new Random();
    for(int i = 1; i <= 10; i++)
    {
     // Every time the loop executes we will get a NEW random value between 2 and 7
        Console.WriteLine(rand.Next(2,8));
    }
```