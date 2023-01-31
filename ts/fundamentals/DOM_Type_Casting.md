# DOM
## Dealing with Possible `null`
```ts 
let anchor = document.querySelector('a')!;
// ts knows this will be an anchor element and will give you access to all anchor tag mehtods
```
- The `!` allows you to say to the compiler with certainity that `anchor` will not be `null`
- TS knows that with that querySelector method which type it will be
- if you select by class or ID you must type cast to get the methods associated with a the type
- Let's show an example of how you would do this by class or ID
```ts
let anchor = document.querySelector('.anchor_class') as HTMLAnchorElement;
```
