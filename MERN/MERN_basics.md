# Play Action Picks
- MongoDB
    - It is a NoSQL
    - Non-relational
    - Stores stuff in JSON like documents
    - Easy to parse the return
    - Flexible, dynamic. not as rigid as relational
- Mongoose
    - Allows for us to use js logic when interacting with MongoDB
    - Mongoose is an Object-Document Mapping (ODM) library that is commonly used in the MERN stack (MongoDB, Express, React, Node.js) to interact with MongoDB, a NoSQL document-based database. It provides a simple and elegant way to interact with MongoDB by providing a schema-based solution for modeling the data.
- Express
    - Express is a web application framework that is commonly used in the MERN stack (MongoDB, Express, React, Node.js) to build web applications. It is built on top of Node.js and provides a simple and flexible way to handle HTTP requests and responses.

    - Routing: Express provides a simple and flexible way to handle routing, which allows you to map a specific route (e.g. '/users') to a specific function (e.g. to fetch all users from the database).

    - Middleware: Express provides a way to handle middleware, which are functions that run before or after a request is processed. Middleware functions can be used to perform various tasks such as logging, authentication, data validation, and error handling.

    - Request and response handling: Express provides a simple and flexible way to handle HTTP requests and responses. It allows you to easily access the request data, such as the body, query parameters, and headers, and to send appropriate responses back to the client.

    - Templating engine: Express can be integrated with a variety of template engines such as Pug, EJS, and Handlebars, which allows you to render dynamic HTML pages on the server-side.

    - API handling: Express can be used to build RESTful API's (Application Programming Interface) which allows different applications to communicate with each other and access data.

    - Error handling: Express provides a way to handle errors and return appropriate error messages to the client.
## React
- React is a JavaScript library for building user interfaces that is commonly used in the MERN stack (MongoDB, Express, React, Node.js). It allows developers to build reusable and composable UI components, and handle the logic and state of those components.
- Reusable Components: React allows developers to create reusable components that can be easily shared and reused throughout an application. This helps to keep the code clean and maintainable.

Virtual DOM: React uses a virtual DOM (Document Object Model) to optimize updates and minimize the number of changes made to the actual DOM. This can improve the performance of web applications, especially those with large amounts of dynamic content.

Unidirectional Data Flow: React follows a unidirectional data flow, which means that data is passed down the component tree in a single direction. This makes it easier to understand and predict how data is being used and changed in an application.

- Client Side Rendering: Creates a more dynamic user experience

## JSON Web Token
- A JSON Web Token (JWT) is a JSON object that is used to securely transmit information between parties. When used for login and registration, a JWT is typically used to authenticate a user on the server and to maintain their session.
- Basically uses a token that securely stores user information that 

The main benefit of using a JWT for login and registration is that it allows for stateless authentication. This means that the server does not need to maintain a session for the user, and can instead rely on the information encoded in the JWT to identify the user and verify their authentication. This can simplify the server's implementation, reduce the load on the server, and make the application more scalable.

When a user logs in, the server will generate a JWT, which contains information about the user, such as their user ID and other claims. The JWT is then sent back to the client and is typically stored in a browser cookie or in local storage.

When the client makes subsequent requests to the server, it will include the JWT in the request headers. The server will then use the information encoded in the JWT to identify the user and verify their authentication.

JWT's are also signed by a secret key, this is done to ensure that the content of the token has not been tampered with. This makes it more secure than using a simple session ID that could be easily intercepted and used by an attacker.

It's worth noting that JWT's are not an end all solution for authentication, it's important to use them in conjunction with other security measures such as HTTPS, CORS, and rate limiting.