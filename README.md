# An Experimental content server based on ructe templates and actix-web 0.7

# Installation
## I. Setup database using diesel cli and postgreSQL
### a. Install postgreSQL
1. https://www.postgresql.org/download/
2. On windows setup LIB for MSVC see https://msdn.microsoft.com/en-us/library/6y6t9esh.aspx

### b. Setup diesel cli
> cargo install diesel_cli --no-default-features --features "postgres"

### c. Create db with diesel
 ```bash
 cd ecspg
 diesel setup --database-url='postgresql://postgres:yourpassword@localhost/ecs'

 diesel migration run
```

 Also see https://github.com/diesel-rs/diesel/blob/master/diesel_cli/README.md

## II. Setup openSSL for TLS/https
Acquire an ssl cert.
### Example Certbot command in user sapce
> certbot certonly --webroot -w ./ -d ecs.dev.reedwolf.com -d nexus.dev.reedwolf.com --logs-dir certbot/log/ --work-dir certbot/ --config-dir certbot/conf/

## III. Add permission for binary to bind on web ports
> sudo setcap CAP_NET_BIND_SERVICE=+eip /home/deploy_user/ecs_web/ecs 

## IV. Configuration
It takes environment variables.
See .env as example.

## V. Startup
> ./todo

# Build it yourself

## Build for release with https
> cd todo
> cargo build --release -p todo --features https

# Potential additional steps in a cloud environment

## Allow postgres connection from new nodes
https://support.plesk.com/hc/en-us/articles/115003321434-How-to-enable-remote-access-to-PostgreSQL-server-on-a-Plesk-server-
