``length`` property: This property returns the number of elements in an array. For example:
```js
let arr = [1, 2, 3, 4];
console.log(arr.length); // 4
```
``concat()`` method: This method is used to join two or more arrays together. It takes in one or more arrays as arguments and returns a new array that is the concatenation of all the input arrays. For example:
```js
let arr1 = [1, 2, 3];
let arr2 = [4, 5, 6];
let arr3 = arr1.concat(arr2);
console.log(arr3); // [1, 2, 3, 4, 5, 6]
```
``indexOf()`` method: This method is used to find the index of the first occurrence of a specified element within an array. It takes in an element as an argument and returns the index of the first occurrence of that element within the array. If the element is not found, the method returns -1. For example:
```js
let arr = [1, 2, 3, 4];
console.log(arr.indexOf(3)); // 2
```
``slice()`` method: This method is used to extract a portion of an array and return it as a new array. It takes in two arguments: the starting index and the ending index (not including the element at the ending index). For example:
```js
let arr = [1, 2, 3, 4];
console.log(arr.slice(1, 3)); // [2, 3]
```
``push()`` method: This method is used to add one or more elements to the end of an array. It takes in one or more elements as arguments and returns the new length of the array. For example:
```js
let arr = [1, 2, 3];
arr.push(4);
console.log(arr); // [1, 2, 3, 4]
```
``pop()`` method: This method is used to remove the last element from an array and return that element. For example:
```js
let arr = [1, 2, 3, 4];
let last = arr.pop();
console.log(arr); // [1, 2, 3]
console.log(last); // 4
```
``shift()`` method: This method is used to remove the first element from an array and return that element. For example:
```js
let arr = [1, 2, 3, 4];
let first = arr.shift();
console.log(arr); // [2, 3, 4]
console.log(first); // 1
```
``unshift()`` method: This method is used to add one or more elements to the beginning of an array. It takes in one or more elements as arguments and returns the new length of the array. For example:
```js
let arr = [1, 2, 3];
arr.unshift(0);
console.log(arr); // [0, 1, 2, 3]
```
``splice()`` method: This method is used to add and/or remove elements from an array. It takes in three arguments: the starting index, the number of elements to remove and the elements to add. It returns an array containing the removed elements. For example:

```js
let arr = [1, 2, 3, 4, 5];
let removed = arr.splice(1, 2, 6, 7);
console.log(arr); // [1, 6, 7, 4, 5]
console.log(removed); // [2, 3]

//In this example, the splice() method is called on the arr array, starting at index 1, removing 2 elements and adding the elements 6 and 7 at the same position. The modified array is now [1, 6, 7, 4, 5], and the removed elements are returned in the removed array [2,3].
```
``join()`` method: This method is used to join all elements of an array into a string. It takes in an optional separator as an argument and returns a string with all the elements of the array separated by that separator. For example:
```js
let arr = [1, 2, 3, 4];
console.log(arr.join()); // "1,2,3,4"
console.log(arr.join("-")); // "1-2-3-4"
```
``forEach()`` method: This method is used to iterate through an array and perform a function on each element. It takes in a callback function as an argument and applies that function to each element of the array. For example:
```js
let arr = [1, 2, 3, 4];
arr.forEach(function(element) {
  console.log(element);
});
```
``map()`` method: This method is used to create a new array with the results of calling a function on every element in the original array. It takes in a callback function as an argument and applies that function to each element of the array. The returned array will be the same length as the original array, but with the new values returned by the callback function. For example:
```js
let arr = [1, 2, 3, 4];
let newArr = arr.map(function(element) {
  return element * 2;
});
console.log(newArr); // [2, 4, 6, 8]
```
``filter()`` method: This method is used to create a new array with all elements that pass the test implemented by the provided function. It takes in a callback function as an argument and applies that function to each element of the array. The returned array will include only the elements for which the callback function returned true. For example:
```js
let arr = [1, 2, 3, 4, 5];
let evenArr = arr.filter(function(element) {
  return element % 2 === 0;
});
console.log(evenArr); // [2, 4]
```