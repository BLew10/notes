# Lists
- Dynamic arrays and what we typically see when using arrays in other languagse

```csharp
List<string> names = new List<string>() { "Brandon", "Kylie" };
//short hand also works
List<string> names = new List<string> {"Brandon", "Kylie"};

```

- Common functions Used on Lists
    - ``Add``
    - ``Insert``
    - ``Remove``
    - ``RemoveAt``
    - ``Count`` (``Length`` for arrays)
```csharp
// Initializing an empty list of Motorcycle Manufacturers that expects string values
List<string> bikes = new List<string>();
// Use the Add function in a similar fashion to push
bikes.Add("Kawasaki");
bikes.Add("Triumph");
bikes.Add("BMW");
// Accessing a generic list value is the same as an array
Console.WriteLine(bikes[2]); //Prints "BMW", remember we start at 0!
// To get the size of a List, we use Count instead of Length
Console.WriteLine($"We currently know of {bikes.Count} motorcycle manufacturers.");

// Using our list of motorcycle manufacturers from before
// we can easily loop through the list of them with a for loop
// this time, however, Count is the attribute for a number of items.
Console.WriteLine("The current manufacturers we have seen are:");
for (int idx = 0; idx < bikes.Count; idx++)
{
    Console.WriteLine("-" + bikes[idx]);
}
// Insert a new item between a specific index
bikes.Insert(2, "Yamaha");
// Removal from List
// Remove is a lot like Javascript array pop, but searches for a specified value
bikes.Remove("Suzuki");
bikes.Remove("Yamaha");
// We can also remove from a specific index
bikes.RemoveAt(0); 
// RemoveAt has no return value however, so we cannot get back what we removed
// The updated list can then be iterated through using a foreach loop
Console.WriteLine("List of Non-Japanese Manufacturers:");
foreach (string manu in bikes)
{
    Console.WriteLine("-" + manu);
}


 ```