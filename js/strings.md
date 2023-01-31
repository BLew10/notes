``length`` property: This property returns the number of characters in a string. For example:
```js
let str = "Hello, World!";
console.log(str.length); // 13
```
``concat(``) method: This method is used to join two or more strings together. It takes in one or more strings as arguments and returns a new string that is the concatenation of all the input strings. For example:
```js
let str1 = "Hello, ";
let str2 = "World!";
let str3 = str1.concat(str2);
console.log(str3); // "Hello, World!"
```
``indexOf()`` method: This method is used to find the index of the first occurrence of a specified substring within a string. It takes in a substring as an argument and returns the index of the first occurrence of that substring within the string. If the substring is not found, the method returns -1. For example:
```js
let str = "Hello, World!";
console.log(str.indexOf("World")); // 7
```
``slice()`` method: This method is used to extract a portion of a string and return it as a new string. It takes in two arguments: the starting index and the ending index (not including the character at the ending index). For example:
```js
let str = "Hello, World!";
console.log(str.slice(7, 12)); // "World"
```
``replace()`` method: This method is used to replace a specified substring with another substring. It takes in two arguments: the substring to be replaced and the substring to replace it with. It returns a new string with the specified substitution made. For example:
```js
let str = "Hello, World!";
console.log(str.replace("World", "Javascript")); // "Hello, Javascript!"
```
``toUpperCase()`` and toLowerCase() method: These methods are used to convert a string to upper or lower case respectively. They don't take any arguments and return a new string with all the characters in upper or lower case. For example:
```js
let str = "Hello, World!";
console.log(str.toUpperCase()); // "HELLO, WORLD!"
console.log(str.toLowerCase()); // "hello, world!"
```
``split()`` method: This method is used to split a string into an array of substrings. It takes in a separator as an argument and returns an array containing the substrings that were created by splitting the string at the specified separator. For example:
```js
let str = "Hello, World!";
console.log(str.split(" ")); // ["Hello,", "World!"]
```
``trim()`` method: This method is used to remove whitespace from the beginning and end of a string. It doesn't take any arguments and returns a new string with the whitespace removed.
```js
let str = "    Hello, World!    ";
console.log(str.trim()); // "Hello, World!"
```

``charAt()`` method: This method is used to get the character at a specified index within a string. It takes in an index as an argument and returns the character at that index. For example:
```js
let str = "Hello, World!";
console.log(str.charAt(0)); // "H"
```
``substring()`` method: This method is used to extract a portion of a string and return it as a new string. It takes in two arguments: the starting index and the ending index (not including the character at the ending index). It's similar to the slice() method but it does not accept negative values as input. For example:
```js
let str = "Hello, World!";
console.log(str.substring(7, 12)); // "World"
```
``substr()`` method: This method is used to extract a portion of a string and return it as a new string. It takes in two arguments: the starting index and the number of characters to include in the returned string. For example:
```js
let str = "Hello, World!";
console.log(str.substr(7, 5)); // "World"
```
``startsWith()`` and ``endsWith()`` method: These methods are used to check whether a string starts or ends with a specified substring. They take in a substring as an argument and return a Boolean value indicating whether the string starts or ends with that substring. For example:
```js
let str = "Hello, World!";
console.log(str.startsWith("Hello")); // true
console.log(str.endsWith("World!")); // true
```
``repeat()`` method: This method is used to repeat a string a specified number of times. It takes in a number as an argument and returns a new string that is the original string repeated that number of times. For example:
```js
let str = "Hello, ";
console.log(str.repeat(3)); // "Hello, Hello, Hello, "
```
These are some of the additional common methods used for strings in JavaScript. Keep in mind that this list is not exhaustive, and there are more methods available for working with strings.