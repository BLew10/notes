# Objects 

``Object.assign():`` This method is used to copy the values of all enumerable own properties from one or more source objects to a target object. Here's an example:
```js
const object1 = { a: 1, b: 2, c: 3 };
const object2 = Object.assign({}, object1);
console.log(object2); // { a: 1, b: 2, c: 3 }
```
``Object.entries():`` This method returns an array of a given object's own enumerable property [key, value] pairs. Here's an example:
```js
const object = { a: 1, b: 2, c: 3 };
console.log(Object.entries(object)); // [ ['a', 1], ['b', 2], ['c', 3] ]
```
``Object.keys():`` This method returns an array of a given object's own enumerable properties. Here's an example:
```js
const object = { a: 1, b: 2, c: 3 };
console.log(Object.keys(object)); // [ 'a', 'b', 'c' ]
```
``Object.values():`` This method returns an array of a given object's own enumerable property values. Here's an example:
```js
const object = { a: 1, b: 2, c: 3 };
console.log(Object.values(object)); // [ 1, 2, 3 ]
```
``Object.freeze():`` This method is used to prevent any new properties from being added to an object, or prevent existing properties from being removed or their values changed. Here's an example:
```js
const object = { a: 1, b: 2, c: 3 };
Object.freeze(object);
object.a = 4;
console.log(object); // { a: 1, b: 2, c: 3 }
```
``Object.seal():`` This method is used to prevent any new properties from being added to an object, and prevent existing properties from being removed. But it allows existing properties to be changed.
```js
const object = { a: 1, b: 2, c: 3 };
Object.seal(object);
object.a = 4;
console.log(object); // { a: 4, b: 2, c: 3 }
```