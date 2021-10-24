<main id="bodyColumn" class="span10">
<h1>Rbatis Dynamic SQL</h1>
<p>This library is a framework for generating dynamic SQL statements.  Think of it as a typesafe SQL templating library, with additional support for Rbatis and rbaits_sql.</p>
<p>The library will generate full DELETE, INSERT, SELECT, and UPDATE statements formatted for use by Rbatis or any http library. The most common use case is to generate statements, and a matching set of parameters, that can be directly used by Rbatis.  The library will also generate statements and parameter objects that are compatible with rbaits_sql.</p>
<p>The library works by implementing an SQL-like DSL that creates an object containing a full SQL statement and any parameters required for that statement.  The SQL statement object can be used directly by Rbatis as a parameter to a mapper method.</p>
<p>The library will generate these types of SQL statements:</p>
<ul>

<li>COUNT statements - specialized SELECT statements that return a Long value</li>
<li>DELETE statements with flexible WHERE clauses</li>
<li>INSERT statements of several types:
<ul>

<li>A statement that inserts a single row with values supplied from a corresponding Object</li>
<li>A statement that inserts a single row with values supplied directly in the statement</li>
<li>A statement that inserts multiple rows using multiple VALUES clauses</li>
<li>A statement that inserts multiple rows using a rbaits_sql batch</li>
<li>A statement that inserts into a table using the results of a SELECT statement</li>
</ul>
</li>
<li>SELECT statements with a flexible column list, a flexible WHERE clause, and support for distinct, “group by”, joins, unions, “order by”, etc.</li>
<li>UPDATE statements with a flexible WHERE clause, and flexible SET clauses</li>
</ul>
<p>The primary goals of the library are:</p>
<ol style="list-style-type: decimal">

<li>Typesafe - to the extent possible, the library will ensure that parameter types match the database column types</li>
<li>Expressive - statements are built in a way that clearly communicates their meaning (thanks to Hamcrest for some inspiration)</li>
<li>Flexible - where clauses can be built using any combination of and, or, and nested conditions</li>
<li>Extensible - the library will render statements for Rbatis, rbaits_sql or plain sql. It can be extended to  generate clauses for other frameworks as well.  Custom where conditions can be added easily if none of the built in conditions are sufficient for your needs.</li>
<li>Small - the library is a small dependency to add.  It has no transitive dependencies.</li>
</ol>
<p>This library is design for Zero cost Dynamic SQL, implemented using (proc-macro,compile-time,Cow(Reduce unnecessary cloning)) techniques。 don't need ONGL engine(mybatis)</p>
 </main>