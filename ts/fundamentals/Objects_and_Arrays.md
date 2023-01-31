# Arrays 
- Arrays can only have types that are initialized at the beginning
```ts
let names = ['brandon','mark','felecia'];
names.push('darryn'); //will work
names.push(1); // WILL NOT

let mixed = [1,true,'brandon'];
//This will be able to add any bool, number or string later since it was initialized
```

# Objects
- Works similary to js
- Key-value pairs work similar to variables in the sense that once type is declared it cannot change
- Object K/v pairs are final upon initialization, no other pairs can be added 
```ts
let ninja = {
    name: 'brandon',
    belt: 'black',
    age:27
}
ninja.name = 'des'; //works
ninja.age = true //NO WORK
ninja.skills = [] //NO WORK
ninja =[] //NO WORK
ninja = {
    // logic
} // works if all the same k/v and types are the same
```
