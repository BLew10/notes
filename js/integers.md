# Integers 

``parseInt()`` method: This method is used to convert a string value to an integer. It takes in a string as an argument and returns the integer representation of that string. For example:
```js
let num = "1234";
let intNum = parseInt(num);
console.log(intNum); // 1234
```
``Number()`` method: This method is used to convert a non-numeric value to a number. It takes in a value as an argument and returns the numeric representation of that value. For example:
```js
let num = "1234";
let intNum = Number(num);
console.log(intNum); // 1234
```
``Math.floor()`` method: This method is used to round a number down to the nearest integer. It takes in a number as an argument and returns the largest integer less than or equal to that number. For example:
```js
let num = 12.34;
let intNum = Math.floor(num);
console.log(intNum); // 12
```
``Math.ceil()`` method: This method is used to round a number up to the nearest integer. It takes in a number as an argument and returns the smallest integer greater than or equal to that number. For example:
```js
let num = 12.34;
let intNum = Math.ceil(num);
console.log(intNum); // 13
```
``bitwise operator:`` This operator uses bit manipulation to perform mathematical operations on integers. For example, the bitwise AND operator (&) compares each bit of the first number to the corresponding bit of the second number. If both bits are 1, the corresponding result bit is set to 1. Otherwise, the corresponding result bit is set to 0.
```js
let num1 = 12; // 1100
let num2 = 4; // 0100
console.log(num1 & num2); // 4
```