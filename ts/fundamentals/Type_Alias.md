# Type Aliases
- Types can get really long winded, in order to simplify the amount of code used you can use a Type Alias
- Example: Say we are creating functions that take in a person object, we dont want to have to explicit state what the person object will intake in every funciton, instead we could do this
```ts 

type Person = {name: string, age: number}

let greetPerson = (person: Person): void => {
    console.log(`Hello, ${person.name}`);
}

// vs

let greetPerson = (person: {name: string, age: number}): void => {
    console.log(`Hello, ${person.name}`);
}
```
- Person is reusable 
