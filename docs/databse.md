# PostgreSQL is the only supported database
In order to protect the integrity of the database constraints are used.

`UNLOGGED` tables are used for data which doesn't need to be retained,
but need to be fast.

##Securing Connections
In a multi tenant hosting environment the database connection is recommended to be encrypted.
Note: PostgreSQL native SSL support is not tested with our database layer.
See [SSL](https://www.postgresql.org/docs/9.1/ssl-tcp.html) and [SSH](https://www.postgresql.org/docs/9.1/ssh-tunnels.html).


