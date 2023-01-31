# Database Basics
- A data base is a collection of data of different models
- Each model has a table with columns
- Sometimes these different table models connect, creating a relationship

## CRUD
- Create, Read, Update, Delete


- Create
```sql
CREATE TABLE table_name (type nameOfColumn, ...)
```
```sql
INSERT INTO table_name (columnNames....) VALUES (associatedValues)
```

- Read
```sql
SELECT * (or specific columns) from table_name WHERE column = value;
```

- Update
```sql 
UPDATE table_name
SET column1 = value1, ...
WHERE some_column = some_value;
```

- Delete
```sql
DELETE FROM table_name
WHERE some_column = some_value;
```
