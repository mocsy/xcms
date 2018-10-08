# A html ructe template generator
The big idea here was that since postgres already ahs the schema:
- the data model (data.rs) is generated
- a basic Crud template for ructe is also generated
- Crud web endpoints are generated as well

This code is deliberately not  macro, it is intended as a quick way to get a basic Crud working with actix_web, ructe, diesel and serde.
The goal here is to be able to customize the end result as you see fit.
It's more like a skeleton app, sometimes manual coding is required.

It assumes:
- each table has an ID
- each table's id is the first column
- if a table name ends with 's' it will be cut for the struct: users -> User