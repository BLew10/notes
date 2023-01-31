# Classes
```ts
class Hospital {
    name: string;
    traumaLevel: number;
    employees: Employee[]; // assumming we made a class
    constructor(n:string, t:number; e:Employee[]) {
        this.name = n;
        this.traumaLevel = t;
        this.employees = e;
    }
}

const Kaiser = new Hospital(...) // ... = name, traumaLevel, employee list
```
- Can use access modifiers: `public`, `private`, `readonly`