# Loops

- Examples 
```csharp
 // For loop that INCLUDES 5
for(int i = 1; i <= 5; i++)
{
    Console.WriteLine(i);
}
// For loop that EXCLUDES 5
for(int i = 1; i < 5; i++){
    Console.WriteLine(i);
}

 // Our original for loop
for(int i = 1; i <= 5; i++)
{
    Console.WriteLine(i);
}
// Our while loop
int i = 1;
while(i <=5)
{
    Console.WriteLine(i);
    i++;
}


```
## Reflection Questions

- What order do the four elements in a for loop execute in?
    - Answer: Iterator (``int i = 1``), End Condition (``i < 5``), Incrementation (``i++``), Code to be Executed (``Console.WriteLine(i);``)
- Describe the differences between for loops and while loops.
    - For loops are used to run a preset amount of times
    - While loops will run until the condition is met and are most desirablity used when it the end condition is dynamic
- Describe one example each of when you would use a for or a while loop.
    - For Loops example would be when want to roll a dice a certain number of times or iterate through an array of a known length
    - While loop would be preferable in a game that has code that we want to run until a game is over. Whether the game be a dice roll or a guessing game