# Conditionals

- **WHERE:** This is used to filter results based on a certain condition. For example, the following query will return all rows from the "employees" table where the "salary" column is greater than 50000:
```sql
SELECT * FROM employees WHERE salary > 50000;
```

- **LIKE:** This is used to filter results based on a pattern match. For example, the following query will return all rows from the "employees" table where the "name" column contains the string "John":
```sql
SELECT * FROM employees WHERE name LIKE '%John%';
```
- **IN:** This is used to filter results based on a list of values. For example, the following query will return all rows from the "employees" table where the "department" column is either "IT" or "Finance":
```sql
SELECT * FROM employees WHERE department IN ('IT', 'Finance');
```

- **BETWEEN:** This is used to filter results based on a range of values. For example, the following query will return all rows from the "employees" table where the "salary" column is between 30000 and 40000:
```sql
SELECT * FROM employees WHERE salary BETWEEN 30000 AND 40000;
```
- **EXISTS:** This is used to filter results based on the existence of a related record in another table. For example, the following query will return all rows from the "orders" table where there exists a related record in the "customers" table:
```sql
SELECT * FROM orders WHERE EXISTS (SELECT 1 FROM customers WHERE customers.id = orders.customer_id);
```
- **IS NULL:** This is used to filter results based on the presence of a null value in a column. For example, the following query will return all rows from the "employees" table where the "phone" column is null:
```sql
SELECT * FROM employees WHERE phone IS NULL;
```
- **AND:** This is used to combine multiple conditions. For example, the following query will return all rows from the "employees" table where the "salary" column is greater than 50000 and the "department" column is "IT":
```sql
SELECT * FROM employees WHERE salary > 50000 AND department = 'IT';
```
- **OR:** This is used to combine multiple conditions. For example, the following query will return all rows from the "employees" table where the "salary" column is greater than 50000 or the "department" column is "IT":
```sql
SELECT * FROM employees WHERE salary > 50000 OR department = 'IT';
```

- **CASE:** This is used to evaluate different conditions and return different values based on the outcome. For example, the following query will return a "bonus" column that is based on the value of the "salary" column:
```sql
SELECT name, salary,
CASE
    WHEN salary > 50000 THEN 'High'
    WHEN salary BETWEEN 30000 AND 49999 THEN 'Medium'
    ELSE 'Low'
END as bonus
FROM employees;
```