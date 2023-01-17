# Class Keywords
- ``virtual``
    - In C#, the ``virtual`` keyword is used to indicate that a method or property can be overridden by a derived class. When a method or property is marked as virtual, a derived class can provide its own implementation of that method or property, while still maintaining compatibility with the base class. This allows for a more flexible and extensible design, as the behavior of a class can be changed or customized by its derived classes.

- ``override``

- ``abstract``
    - In C#, the ``abstract`` keyword is used to define an abstract class or method.

    - An abstract class is a class that cannot be instantiated on its own. Instead, it serves as a blueprint for other classes to inherit from. An abstract class can have both abstract and non-abstract methods. Abstract methods are methods that have only a method signature and no implementation. These methods must be implemented by any derived class that inherits from the abstract class.

    - Abstract classes are useful when you want to provide a common interface for derived classes but with different implementation.

    - The abstract keyword is also used to define abstract methods. These are methods that have only a method signature (name, return type, and parameters) and no implementation. Abstract methods must be implemented by any derived class that inherits from the abstract class.

    - An abstract method can only be defined within an abstract class, and it is a contract that derived class should implement it. An abstract method cannot be private.

    - In summary, the abstract keyword is used to define an abstract class or method, which cannot be instantiated and is a blueprint for other classes to inherit from. The derived class need to implement the abstract methods to provide their own implementation. 