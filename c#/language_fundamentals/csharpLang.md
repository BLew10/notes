# C Sharp / .NET Overview
- Compile language
    - The job of a compiler is to parse source code from human-readable to computer readable
    - In a compiled language, the code is read, checked for mistakes before being executed
        - It finds incorrect data types and logic erroes thus preventing it from running 
- OOP Design
- Stability
     - not constantly going through version changes
- SUPPORT
    - Built in house my Microsoft

- Creating a File that can be execute. Run this line in the CLI
```
dotnet new console -o ProjectName
```

- To Execute File
```
dotnet run
```

## Major Libraries
- **ASP.NET**
    - A powerful, event-driven model for handling web requests and responses.
    - A rich set of server-side controls for building user interfaces. These controls provide a variety of features such as client-side scripting, data binding, and validation, which can greatly simplify the development process.
    - A robust security model, which includes built-in support for forms-based authentication and authorization, as well as support for custom authentication and authorization schemes.
    - A caching framework that allows developers to cache frequently-used data, reducing the need for database queries and improving application performance.
    - A rich set of tools for debugging and troubleshooting web applications, including detailed error messages, tracing, and performance monitoring.
    
    - One of the most important feature that ASP.NET provides is the Model-View-Controller (MVC) pattern. This pattern separates the application logic into three main components: the model, which represents the data and business logic; the view, which displays the data to the user; and the controller, which handles user input and updates the model and view accordingly. This separation of concerns makes the code more maintainable and testable.

    - Another important feature is the WebAPI, it is a framework that allows developers to build HTTP-based services using the same framework and patterns as traditional ASP.NET web applications. This allows developers to build web services that can be consumed by a variety of clients, including web browsers, mobile devices, and other web services.

    - Additionally, ASP.NET Core is an open-source, cross-platform version of ASP.NET which allows developers to build web applications that can run on Windows, Linux, and macOS. This allows for more flexibility and more choices in terms of environment and hosting options.

    - In summary, ASP.NET is a powerful web application framework that provides a rich set of features for building and maintaining dynamic, data-driven web applications. Its built-in support for common web development tasks, such as security, caching, and error handling, makes it an efficient choice for building web applications. Additionally, the MVC pattern and WebAPI feature makes it easy to build maintainable, scalable and testable web applications.
- **Entity Framework**
    - An Object-Relational Mapping (ORM) tool developed by Microsoft. It is a part of the .NET Framework and allows developers to interact with databases using an object-oriented programming model, rather than writing raw SQL statements.

    - Some of the key functionality included in the Entity Framework include:

        - **Data Modeling:** EF provides several ways to model and map the database schema to the application's object model, including the Code-First, Database-First and Model-First approaches.

        - **Object-Relational Mapping:** EF maps the application's object model to the database schema, allowing developers to work with entities (objects) instead of tables and rows.

        - **Database-Agnostic:** EF is database-agnostic, meaning it supports multiple databases such as SQL Server, MySQL, and Oracle. This allows for easy switching between databases and also allows for your application to be more database independent.

        - **Query and Data Manipulation:** EF provides a powerful query API, LINQ-to-Entities, that allows developers to write type-safe, expressive, and efficient queries over data. It also provides a change-tracking feature, which allows developers to track changes made to the entities and write more efficient updates.

        - **Transactions and Concurrency:** EF provides support for database transactions and concurrency, allowing developers to write code that runs inside a transaction and ensures that multiple threads or processes can access the database without conflicting with each other.

        - **Caching:** EF provides caching features which allows you to cache the data retrieved from the database and avoid repetitive and unnecessary database trips.

        - **Lazy Loading:** EF provides a feature called lazy loading that allows you to load related entities on demand. This is useful when you are working with large data sets, and you want to load only the data you need to display or manipulate.

    - **Summary:** Overall, the Entity Framework is a powerful tool that simplifies data access and manipulation in .NET applications, allowing developers to focus on the business logic rather than the details of interacting with the database.