# Functions
```ts
let greet: Function;
greet = () => {
    console.log("Hello World");
}
```
## Optional parameters
- Signified by the `?`
```ts
let add = (a: number, b: number, c?: number | string) => {
    console.log(a + b);
    console.log(c); //will print undefined if nothing is input
}
add(5,10) // 15

// can use a default param as well but it is never used with conjunction to optional paramters
```

## `return` type
- It can be inferred or be explicit
```ts

 // inferred
let minus = (a: number, b: number) => {
    return a - b;
}

// explicit
let minus = (a: number, b: number): number => {
    return a - b;
}

//also can do this, this called a funciton signature
// defines what the funciton mu
let calc = (a:number, b:number, c: string) => number;

calc = (numOne: number, numTwo: number, action: string) => {
    if(action === 'add') return a + b
    if(action === 'minus') return a -b
}
```