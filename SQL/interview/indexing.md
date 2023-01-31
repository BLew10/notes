# Indexing
- SQL indexing is the process of creating a separate data structure, called an index, that is used to improve the performance of queries on a database table. An index is a data structure that stores a mapping of the values in one or more columns of a table to the locations of the corresponding rows in the table.
- The point is to improve db performance
- Common Practice is to have pointer to things at are common to query

- When a query is executed, the database engine can use the index to quickly locate the rows that match the conditions specified in the query, rather than having to scan the entire table. This can significantly improve the performance of queries on large tables, particularly when the table is frequently queried or the query conditions are not selective.

- There are several types of indexes that can be created in SQL, including:

- Clustered index: A clustered index determines the physical order of data in a table. Each table can have only one clustered index.
- Non-clustered index: A non-clustered index does not determine the physical order of data in a table. Each table can have multiple non-clustered indexes.
- Unique index: A unique index ensures that no two rows in the table have the same value in the indexed column(s).
- Full-text index: A full-text index is used to improve the performance of text-based searches.
- It is important to note that creating too many indexes or indexing on the wrong columns can negatively impact performance by increasing the time it takes to insert, update or delete data in the table.