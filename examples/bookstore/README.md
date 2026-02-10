# Bookstore Example

This example demonstrates Ember-only APIs for a simple bookstore service.

## Features
- List all books
- Get a specific book
- Search by author
- Add a book
- Update a book
- Remove a book
- Remove all books

## Run

```bash
cargo run -p bookstore
```

This example uses Ember placeholders and does not start an HTTP server yet.
It does connect to Postgres and creates the `books` table on startup.

## Configuration

Use the local YAML file (preferred):

`application.yaml`

```yaml
ember:
	service:
		name: "bookstore"
		listen: "0.0.0.0:8080"
database:
	url: "postgres://bookstore:bookstore@127.0.0.1:55432/bookstore?sslmode=disable"
	username: "bookstore"
	password: "bookstore"
```

Profiled configs are supported too (set `EMBER_PROFILE`):

`application-dev.yaml`, `application-prod.yaml`, etc.

Or set the configuration JSON before running:

```bash
# PowerShell
$env:EMBER_CONFIG_JSON = '{"ember":{"service":{"name":"bookstore","listen":"0.0.0.0:8080"}},"database":{"url":"postgres://bookstore:bookstore@127.0.0.1:55432/bookstore?sslmode=disable","username":"bookstore","password":"bookstore"}}'
```

## Postgres (Docker)

Build the image:

```bash
docker build -t ember-bookstore-postgres -f Dockerfile.postgres .
```

Run the container:

```bash
docker run --name ember-bookstore-db -p 55432:5432 ember-bookstore-postgres

Note: The example container uses `POSTGRES_HOST_AUTH_METHOD=trust` for local development.
```
