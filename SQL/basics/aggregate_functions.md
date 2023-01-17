# Aggregate Functions

- **`COUNT:`** This operation returns the number of rows in a table that match a specific condition. The basic syntax is `SELECT COUNT(*) FROM table_name`, which will return the total number of rows in the specified table. If you want to count the number of rows that match a specific condition, you can use the WHERE clause in the SELECT statement, like this:
```sql
SELECT COUNT(*) FROM orders WHERE order_status='shipped';
/* This query will return the number of rows from the "orders" table where the "order_status" column is 'shipped' */
```

- **`SUM:`** This operation returns the total sum of a specified column. The basic syntax is `SELECT SUM(column_name) FROM table_name`, and it will return the total sum of the specified column in the table. For example,

```sql
SELECT SUM(price) FROM products;
/* This query will return the total sum of the "price" column in the "products" table. */

```

 - **`AVG:`** This operation returns the average value of a specified column. The basic syntax is "SELECT AVG(column_name) FROM table_name", and it will return the average value of the specified column in the table. For example,

```sql
SELECT AVG(price) FROM products;
/* This query will return the average value of the "price" column in the "products" table. */
```

- **`MIN:`** This operation returns the minimum value of a specified column. The basic syntax is "SELECT MIN(column_name) FROM table_name", and it will return the minimum value of the specified column in the table. For example,
```sql
SELECT MIN(price) FROM products;
/* This query will return the minimum value of the "price" column in the "products" table. */
```

- **`MAX:`** This operation returns the maximum value of a specified column. The basic syntax is "SELECT MAX(column_name) FROM table_name", and it will return the maximum value of the specified column in the table. For example,
```sql
SELECT MAX(price) FROM products;
/* This query will return the maximum value of the "price" column in the "products" table. */
```

- **`GROUP BY:`** This clause is used in conjunction with aggregate functions like COUNT, SUM, AVG, MIN and MAX, to group the result-set by one or more columns. The basic syntax is "SELECT column_name, aggregate_function(column_name) FROM table_name GROUP BY column_name". For example:
```sql
SELECT category, SUM(price) FROM products GROUP BY category;
/* This query will return the total sum of the "price" column in the "products" table grouped by the "category" column. */
```

- **`HAVING:`** This clause is used in conjunction with GROUP BY, to filter the groups based on a certain condition after the groups are already created. The basic syntax is "SELECT column_name, aggregate_function(column_name) FROM table_name GROUP BY column_name HAVING aggregate_function(column_name) operator value"
For example,
```sql
SELECT category, COUNT(product_id) FROM products GROUP BY category HAVING COUNT(product_id) > 5;
/* This query will return the number of products in each category which have more than 5 products. */
```

Note that the examples provided are simple and not