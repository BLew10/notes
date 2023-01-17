# Dictionaries
```csharp
Dictionary<string,string> person = new Dictionary<string,string>();
person.Add("name", "brandon");
person.Add("FavoriteColor", "purple");

Dictionary<string,string> person = new Dictionary<string,string> { {"name", "brandon"}, {"FavoriteColor", "purple"} };


//iterating thorugh a dictionary
foreach(KeyValuePair<string,string> item in person)
{
    Console.WriteLine($"{entry.Key} - {entry.Value}");
}

foreach(string key in person.Keys)
{
    Console.WriteLine($"{key}")
}

```