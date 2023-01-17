# Conditionals 
 - If / Else If / Else 
 ```c
 int temperature = 68;
if(temperature >= 72)
{
    // This executes if the temperature is greater than or equal to 72
    Console.WriteLine("It's a beautiful day to go outside!");
} else if(temperature > 64) {
    // This executes only if the temperature was NOT greater than or equal to 72 and IS above 64
    Console.WriteLine("I think it should be fine to go outside today with a light jacket.");    
} else {
    // If neither of the above conditions are met, we run the else statement
    Console.WriteLine("Maybe I'll stay inside today.");
}
```

- Common Operators
```c
<
>
<=
>=
==
!=
```

- Logical Operators 
```c
&&
||
! 
```

- Examples 
```c
int temperature = 62;
string weather = "Cloudy";
// If the temperature is greater than or equal to 72 OR the weather is sunny, we run the first condition
if(temperature >= 72 || weather == "Sunny")
{
    Console.WriteLine("It's a beautiful day to go outside!");
// Otherwise, if the temperature is greater than 64 AND it is not raining out, we run the second condition
} else if(temperature > 64 && weather != "Rainy") {
	Console.WriteLine("I think it should be fine to go outside today with a jacket.");    
} else {
    Console.WriteLine("Maybe I'll stay inside today.");
}
```

