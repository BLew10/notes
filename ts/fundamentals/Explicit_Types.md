# Explicit Types
- TS lets you declare a type without putting in a value
```ts
let character:string;
```
- Arrays are unique, you actually want to set it to empty to utilize the push method
```ts
let ninjas: string[] = [];
//union type array initalization
let mixed: (string|number)[] = [];
// can use union types on non arrays as well
let stuff: string|number; //notice no parenthesis
```
- Objects
```ts
let ninja: object
//OR
let ninja: {
    name: string,
    belt: string,
    age: number
}
```
