# Arrays
- With all programming languages, we sometimes need to cluster related values (think a grocery list or a list of student names) into a collection under a single variable name. With C#, there are quite a few ways we can do this. The first one we will go over is the array.

- Arrays in C# are similar to other languages you may have worked with, such as JavaScript or Python, in that they are a numerically indexed collection of values. However, unlike these other languages, arrays in C# must have an exact length that is specified when the array is created, and this length cannot be changed.

```csharp
// Declaring an array of length 5, initialized by default to all zeroes
int[] numArray = new int[5];
// Declaring an array with pre-populated values
// For arrays initialized this way, the length is determined by the size of the given data
int[] numArray2 = {1,2,3,4,5};


int[] array3; // Declared without initializing a size
array3 = new int[] {1,3,5,7,9}; // We can now fill the array by using the new operator


string[] myCars = { "Mazda Miata", "Ford Model T", "Dodge Challenger", "Nissan 300zx" };
// The 'Length' property tells us how many values are in the array (4 in our example here). 
// We can use this to determine where the loop ends
for (int idx = 0; idx < myCars.Length; idx++)
{
    Console.WriteLine($"I own a {myCars[idx]}");
}


string[] myCars = new string[] { "Mazda Miata", "Ford Model T", "Dodge Challenger", "Nissan 300zx"}; 
foreach (string car in myCars)
{
    // We no longer need the index, because variable 'car' already represents each indexed value
    Console.WriteLine($"I own a {car}");
}
```

## Reflection Questions
-  Describe the difference between a for loop and a foreach loop. What do you get from either loop on each iteration?
    - A for loop and a foreach loop are both used to iterate over a collection of items, such as an array. A for loop typically uses an index variable to keep track of the current position in the array, and allows for more control over the iteration process. A foreach loop, on the other hand, automatically iterates through each element in the array without the need for an index variable.
- When looping through an array of length 10, what will be the index of the final value?
    - When looping through an array of length 10, the index of the final value will be 9. This is because array indices are zero-based, meaning they start at 0 rather than 1.
- Why do you think we can only store one data type in any given array? (For example, why must our array only contains integers or strings and nothing else?)
    - Arrays are designed to hold a single data type in order to optimize the amount of memory needed to store the data. Allowing for multiple data types in a single array would require more memory to store the different types, and would also make the array less predictable and more difficult to work with.