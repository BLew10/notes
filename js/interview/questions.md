# Interview.md
## **Why do we call JavaScript as dynamic language?**
JavaScript is called a dynamic language because it allows for variables to hold different types of data, and the type can change at runtime.
```js
let x = 5;
x = "hello";

```
## **How does JavaScript determine data types ?**
- JavaScript determines data types based on the value assigned to a variable.
```js
let x = 5; // x is a number
let y = "hello"; // y is a string

```
## **What is typeof functions?**
-The typeof function returns a string indicating the type of the variable or value passed to it.
```js 
console.log(typeof 5); // "number"
console.log(typeof "hello"); // "string"

```

## **What are different datatypes in JavaScript ?**
- string, bool, number, object, null

## **Explain Undefined Data types ?**
- variables is declared by not assigned a valuable
```js
let x;
console.log(x); //undefined
```
## **What is Null ?**
- intentional absence of data
```js
let x = null;
console.log(x); //null
```
## **Differentiate between Null and Undefined ?**
- Undefined data type has been create but not assigned a value
- Null indicates absence of data
- Both null and undefined represent absence of a value, but null is explicitly set by the programmer, while undefined means a variable has been declared but has not been assigned a value.
## **Explain Hoisting ?**
- Variables or functions are moved to the top of the scope before execution
- Hoisting in JavaScript refers to the behavior of variable and function declarations being moved to the top of their scope, regardless of where they are actually defined in the code.
```js console.log(x); //undefined
var x = 5;
```

## **Are JavaScript initialization hoisted ?**
- Variable initialization is not hoisted in JavaScript, only variable declarations are hoisted. For example:
```js
console.log(x); // undefined
var x = 5;
```
## **What are global variables ?**
- Global variables are variables that are accessible throughout the entire program, including inside functions. They are declared outside of any function or block scope. For example:
## **What are the issues with Global variables ?**
- One issue with global variables is that they can be easily overwritten by other parts of the code, leading to unexpected behavior. They also can create naming conflicts, when two or more variables have the same name. They can also be accessed by any function, which can lead to problems in terms of security and privacy.
## **What happens when you declare variable with out VAR ?**
- No var, makes variable global
```js
x = 5; // x is a global variable
let y = 10; // y is a local variable
```
## **What is Use Strict ?**
- "use strict" is a JavaScript directive that indicates that the code should be executed in strict mode. In strict mode, certain error-prone and unsafe features are disabled, such as using undeclared variables. For example:

```js "use strict";
x = 5; // ReferenceError: x is not defined
```

## **How to force developers to use Var keyword ?**
  - To force developers to use the var keyword, you can use a linter tool such as ESLint and configure it to detect and report variables that are declared without the var keyword.

  **How can we handle Global Variables ?**
  - To handle global variables, you can use an IIFE (Immediately Invoked Function Expression) to create a new scope and keep your global variables separate. For example:
```js
  (function() {
    let x = 5; // x is a global variable
})();
```

 ## **How can we avoid Global variables ?**
  - To avoid global variables, you can use let and const to define variables in a block scope, and avoid using var keyword.

 ## **What are Closures ?**
 - Closures are functions that have access to variables in their parent scope even after the parent function has returned. For example:
 ```js 
 function makeCounter() {
    let count = 0;
    return function() {
        return count++;
    }
}

let counter = makeCounter();
console.log(counter()); // 0
console.log(counter()); // 1
```

  ## **Why do we need Closures ?**
  - Closures are useful in JavaScript because they allow inner functions to access variables in their parent scope even after the parent function has returned. This allows you to create functions with private variables and methods that can only be accessed by the inner function.

  ## **Explain IIFE ?**
  - IIFE (Immediately Invoked Function Expression) is a function that is immediately invoked after it is defined. It is written as follows:
```js
  (function() {
    // code
})();
```

  ## **What is the use of IIFE ?**
  - The use of IIFE is to create a new scope and keep variables and functions separate from the global scope. This can help prevent naming conflicts and unintended interactions between different parts of your code.

 ## **What is name collision in global scope ?**
 - Name collision in global scope refers to the situation where multiple variables or functions with the same name are defined in the global scope, which can lead to unexpected behavior and errors.

 ## **IIFE vs Normal Function?**
- IIFE's are executed immediately after they are defined, while normal functions are executed only when they are called.

## **What are design patterns ?**
- Design patterns are reusable solutions to common programming problems. They provide a standardized way of organizing and structuring code.
## **Which is the most used design pattern?**
  - The most used design pattern is the module pattern, which is used to organize and structure code in a reusable and maintainable way.


## **How can we do inheritance in JavaScript ?**
- Inheritance in JavaScript can be done using the Object.create() method, which creates a new object with its prototype set to an existing object. For example
```js 
let parent = {
    x: 5
};
let child = Object.create(parent);
console.log(child.x); // 5
```

##  **What is Let Keyword?**
  - The let keyword is used to declare variables in JavaScript. Variables declared with let have block scope, which means they can only be accessed within the block they were defined in. For example:
```js
let x = 5;
if (true) {
    let x = 10;
    console.log(x); // 10
}
console.log(x); // 5
```

## **Let vs Var**
- Let vs Var: let is block scoped while var is function scoped. Variables declared with let are only accessible within the block they were defined in, while variables declared with var are accessible within the entire function they were defined in.

## **What is class in ES6 ?**
- Class in ES6 is a new way to create objects and define their properties and methods. For example:

```js 
class Person {
    constructor(name) {
        this.name = name;
    }
    greet() {
        console.log(`Hello, my name is ${this.name}`);
    }
}
let john = new Person("John");
john.greet(); // "Hello, my name is John"
```
## **So with class Keyword does it imply JavaScript is a OOP language ?**
- The class keyword in JavaScript does not imply that the language is fully object-oriented, it just allows for an easier way to create objects and define their properties and methods.

## **Differentiate between class and normal function ?**
- A class is a blueprint for creating objects and defines the properties and methods that objects of that class will have. A normal function is a block of code that can be invoked to perform a specific task.

## **What is a Arrow function ?**
- Arrow functions are used to make code more concise and readable. They are particularly useful when working with callbacks and higher-order functions.
## **Why do we use Arrow function ?**
- Arrow functions are used to make code more concise and readable. They are particularly useful when working with callbacks and higher-order functions..

## **Differentiate between Arrow vs Normal Function ?**
- The main difference between arrow functions and normal functions is the way they handle the this keyword. In normal functions, this refers to the object that the function is a method of, while in arrow functions, this refers to the lexical scope in which the function was defined
## **Does Arrow function create its own this ?**
- No

## **Explain Asynchronous execution ?**
- Asynchronous execution refers to the execution of code that is not executed in a linear, sequential manner. Instead, asynchronous code can run in the background while other code continues to execute.

## **Synch vs Asynch ?**

How can we do Asynch calls ?
What is a thread ?
Explain Multi-threading ?
Is JavaScript Multi-threaded ?
Then how does Settimeout run ?
What is a WebAPI/Browser API ?
 What is a Event loop and callback queue?
Eventloop and Callback code question


**What is your experience with JavaScript?**
I have several years of experience working with JavaScript, both on the front-end and back-end. I am familiar with the core language features and have experience working with popular libraries and frameworks such as React and Angular.

**Can you explain how JavaScript's event loop works?**
JavaScript uses an event loop to handle asynchronous code execution. The event loop is a continuous loop that checks the message queue for new messages. Each message represents a task that needs to be executed. Once a message is retrieved from the queue, the event loop will execute the corresponding task and then start the process again to check for new messages.

**How do you handle asynchronous code in JavaScript?**
I handle asynchronous code in JavaScript using callback functions, promises, and async/await. I prefer using async/await as it makes the code more readable and easy to understand.

**Can you explain the difference between let, var, and const?**
``let`` and ``var`` are used to declare variables in JavaScript. The main difference between them is that let has block-scope while var has function-scope. On the other hand, const is used to declare a variable that cannot be reassigned.

**Have you ever worked with a JavaScript framework or library? Which ones?**
Yes, I have experience working with several JavaScript frameworks and libraries such as React, Angular, and Vue.js. I have used React for building web applications and Angular for building enterprise-level applications.

**How do you handle errors and debugging in JavaScript?**
I use the browser's developer tools to debug JavaScript code. I also use the console.log() function to output variables and the state of my application. For handling errors, I use try-catch statements and I also make use of JavaScript error handling mechanisms such as onerror and addEventListener.

**Can you explain the difference between a closure and a scope?**
A closure is a function that has access to the variables in the scope in which it was created, even after that scope is closed. A scope is the accessibility/visibility of variables, functions, and objects in some particular part of your code during runtime.

**How do you implement data validation in JavaScript?**
I use JavaScript's built-in validation methods such as isNaN(), typeof, and instanceof to validate data types. I also use regular expressions to validate user input.

**How do you manage state in a JavaScript application?**
I use state management libraries such as Redux or MobX to manage the state of my JavaScript applications. These libraries allow me to manage the state of my application in a centralized and predictable way.

**Can you explain the difference between synchronous and asynchronous JavaScript?**
Synchronous JavaScript runs in a linear fashion, where one line of code must be executed before the next line can be executed. Asynchronous JavaScript, on the other hand, allows multiple lines of code to be executed simultaneously without blocking the execution of other lines of code.