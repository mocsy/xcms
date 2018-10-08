#Authentication
##Why Password hashing?
Passwords are not stored, and can not be recovered, only replaced.
Instead calculated words, called hashes are stored.
This allows the service to identify if the provided password is correct,
and at the same time provides added security.

##Password hash storage
The [Argon2](https://en.wikipedia.org/wiki/Argon2) password hashing
algorithm's d variant is used to calculate password hashes.
Argon2-d was chosen because the data independent variant, Argon2-i,
requires more than ten 10 passes to be resilient against dedicated hardware,
FPGA, or GPU based attacks, while the D variant is currently generally expected
to be more resilient with fewer passes.

The D variant is data dependent and as a result an attacker who gains read access
to the memory space of the application could fetch argon temporary data from the
memory and using it can break the password hash faster.
This might not be a significant argument against argon2-d in the current use case: anyone with access to
the memory space of the server application can access other internals of the server.
In practice the server instance must be appropriately protected against unauthorized
attackers gaining root access.
From now on the design process assumes the memory space of the services can only be
accessed by authorized personnel, and no guarantees are made about an otherwise
compromised environment.
Note: hash calculation &login could take up to a 1000x more time than other requests

##The password salt
The encryption mechanism is deterministic. Given all the inputs are the same,
the results will be the same.
This hash can be stored in the database in place of the password.
Due to it's deterministic nature, a unique secret salt parameter must be defined by
the service operators during setup, so an attacker with the knowledge of the algorithm,
won't be able to reproduce the password hashes.
These hashes are computed from the password in the appropriate backed service, and
are never sent to client applications.
These are also never sent over the network internally, excluding the database connection.
Configure the PW_SALT= parameter in environment variables, see the supplied .env file.

##Backend network security
On the backed, in case an attacker could listen on the network communication between the
login service and the database, they won't ever see the passwords, just their hashes.
See Securing Connections in [Database](database.md)
