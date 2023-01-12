# Clauses

- **ORDER BY:** Used in a SELECT statement to sort the result set based on one or more columns and is defaulted to **ASC**

```sql
SELECT first_name, last_name, salary
FROM employees
ORDER BY last_name DESC, first_name;
```

- **GROUP BY:** This clause is used to group rows in the result set based on one or more columns. It is often used in conjunction with aggregate functions like SUM, COUNT, and AVG to perform calculations on each group of rows.

```sql
SELECT department, SUM(salary)
FROM employees
GROUP BY department;
``` 
    This query will group the rows in the "employees" table by the "department" column and calculate the sum of the "salary" column for each group. The result set will have one row for each unique department, with the department name and the total salary for that department.

- **HAVING:** This clause is used in conjunction with the GROUP BY clause to filter the groups based on a certain condition. It operates on the result set after the GROUP BY clause has been applied.

```sql 
SELECT department, SUM(salary)
FROM employees
GROUP BY department
HAVING SUM(salary) > 50000;
```
    This query is similar to the previous one, but it will only include the departments that have a total salary greater than 50000.

- **LIMIT:** This clause is used to limit the number of rows returned in the result set. It's commonly used to limit the number of rows returned by a SELECT statement, it's useful when you want to return a fixed number of the top or bottom records and avoid to retrieve the entire table.

```sql 
SELECT *
FROM employees
ORDER BY salary DESC
LIMIT 5;
```
    This query will retrieve the top 5 employees with the highest salary in the "employees" table.

- **OFFSET:** This clause is used in conjunction with the LIMIT clause, it's used to skip a certain number of rows before starting to return the rows. It's commonly used for pagination purposes.
```sql 
SELECT *
FROM employees
ORDER BY salary DESC
LIMIT 5 OFFSET 10;
```
    This query will retrieve 5 employees with the highest salary in the "employees" table after skipping the first 10 rows, this can be useful when you are implementing pagination feature on your application and want to retrieve the next page of data.