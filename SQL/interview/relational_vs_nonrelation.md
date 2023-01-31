# Relational Vs Non-Relational

## Relational 
- Relational databases are a good choice when you have a well-defined, structured data model and need to ensure data integrity, consistency, and security, and you need to perform complex queries and data analysis.
- A relational database is a type of database management system (DBMS) that stores data in the form of tables and uses relationships between the data to organize it. The data is organized into tables, also known as relations, which consist of rows and columns, similar to a spreadsheet. Each row represents a record, and each column represents a field within that record.

The relationships between the data are defined using keys, which are used to link data in one table to data in another table. A primary key is a unique identifier for a record in a table, and a foreign key is a reference to a primary key in another table.

The main advantage of a relational database is its ability to handle large amounts of data and to provide a consistent, organized way of storing, retrieving and querying the data. This makes it easy to manipulate the data and to perform complex queries.

Relational databases are managed by a DBMS, which is responsible for maintaining the integrity of the data, enforcing constraints, and providing a way to query the data. The most widely used query language for relational databases is SQL (Structured Query Language).
    - Data Integrity: Relational databases provide several integrity constraints such as primary key, foreign key, unique keys, and check constraints which help in maintaining the consistency and accuracy of the data.
    - Data Normalization: Normalization is the process of organizing data into separate tables based on the relationships between the data. This helps in reducing data redundancy and maintaining data consistency.

## Non Relational 
- This is it. This is why we use MongoDB. SQL table rows have the rigid structure: every entry in a database (i.e. every row) has the same fields. This is not the case in MongoDB. Each document is a JSON object and is able to have any number of key-value pairs you so desire. And just like in regular Javascript objects, we can add key-value pairs to objects on the fly. When you use a NoSQL database, you gain speed but lose rigidity; you trade structure for flexibility. This should make sense. Remember, people use Mongo for its speed. If we can't write to objects on the fly, we might slow down our transactions and that would be counterproductive.

Technically, we've lied. MongoDB doesn't use JSON objects to store your data. They use what's called BSON (Binary JSON). BSON is friendlier to store (due to it being binary; don't stress about this) and can support a few more things (like dates, which are not part of JSON, but crucial for databases). But for all intents and purposes, MongoDB documents are to be treated and used as if they were ordinary JSON. Trust us! The next tab will delve into working with documents, so let's get moving!

A non-relational, or NoSQL, database is a type of database management system that does not store data in the traditional table-based structure of relational databases. Instead, it uses a more flexible data model, such as key-value pairs, documents, or graph-based data, which allows it to handle unstructured and semi-structured data.

Key-Value databases: Key-value databases store data as a collection of key-value pairs, where each key is a unique identifier for a piece of data and the value is the data itself. Key-value databases are highly scalable and provide fast access to data, making them suitable for high-performance applications such as caching and real-time analytics. Examples of key-value databases include Redis and Riak.

Document databases: Document databases store data in semi-structured documents, such as JSON or XML, which can contain nested elements and arrays. This allows for easy storage and retrieval of complex data structures. Examples of document databases include MongoDB and Couchbase.