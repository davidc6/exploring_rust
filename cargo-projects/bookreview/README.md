# Book review service

- `main.rs`
    - bootstraps the application: loads configuration file, connects to Postgres, inits `startup.rs`
- `startup.rs`
    - attaches connection to the application state, registeres HTTP services and starts listenint to HTTP connections
- `libs.rs`
    - shared code, all modules are declared here
- `configuration.rs` - creates application configuration
- `routes/` - all the routes are store in this directory
- `migrations/` - database migrations

### Notes

- Query pipelining - allows client to send all queries to the server up front, the server will then take care of the rest (as oppose to waiting for result and sending more queries) - https://docs.rs/tokio-postgres/0.5.5/tokio_postgres/index.html#pipelining
- Table-driven tests (aka parametirized) - each table query is a complete test case with inputs and expected results (https://github.com/golang/go/wiki/TableDrivenTests)
- AAA (arrange-act-assert) test pattern

### Resources

- SQLx - https://github.com/launchbadge/sqlx (Rust SQL toolkit)
- Cloud Spanner - https://cloud.google.com/spanner (managed relational DB)
-  