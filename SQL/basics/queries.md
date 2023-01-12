# **Common Queries**

- **CREATE TABLE**

```sql
CREATE TABLE tableName (id INTERGER PRIMARY KEY, name TEXT, quantity INTEGER)
```

- **SELECT:** This query is used to retrieve data from one or more tables in a database. The syntax is:
```sql
SELECT column1, column2, ... FROM table_name;
```

- **INSERT:** This query is used to insert data into a table. The syntax is:
```sql
INSERT INTO table_name (column1, column2, ...) 
VALUES (value1, value2, ...);
```
- **UPDATE:** This query is used to update existing data in a table. The syntax is:
```sql
UPDATE table_name
SET column1 = value1, column2 = value2, ...
WHERE some_column = some_value;
```
- **DELETE:** This query is used to delete data from a table. The syntax is:

```sql
DELETE FROM table_name
WHERE some_column = some_value;
```
- **JOIN:** This query is used to combine rows from two or more tables based on a related column between them. The syntax is:
```sql
SELECT column1, column2, ...
FROM table1
JOIN table2
ON table1.column = table2.column;
```

**An example:**

```sql
SELECT first_name, last_name, salary
FROM employees
WHERE salary > 50000
ORDER BY last_name ASC;
```
This query will select the first name, last name and salary of employees who have a salary greater than $50,000 and will order the results by last name in ascending order.