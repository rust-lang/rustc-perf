# Useful queries and commands
This document contains useful queries and commands that should be performed manually in exceptional
situations.

## Remove data for a stable artifact from the DB
This is important for situations where there is some compilation error for a stable benchmark,
with a stable release of the compiler. While this should be rare, it happens sometimes e.g. because
of future incompatibility lints turning into errors.

The benchmark should be fixed first, and then the DB should be altered (see below).

The easiest solution is to simply completely remove the artifact from the DB.
You can do that either using the following command:

```bash
$ cargo run --bin collector purge_artifact <artifact-name>
# $ cargo run --bin collector purge_artifact 1.70.0 # Remove stable artifact 1.70.0
```

Or using SQL queries. There are `ON DELETE CASCADE` clauses for `aid` (artifact ID) on tables that
reference it, so it should be enough to just delete the artifact from the `artifact` table.
The set of queries below show an example of removing the measured data and errors for Rust `1.69`
and `1.70`:
```sql
DELETE FROM artifact
WHERE name IN ('1.69.0', '1.70.0') AND
      type = 'release';
```
After executing this query, the server should automatically re-benchmark these artifacts again.

## Remove data for a master/try artifact from the DB
This is similar to removing a stable artifact, however it also has to be removed from the
`pull_request_build` table.

```sql
DELETE FROM artifact
WHERE name = "<sha>";

DELETE FROM pull_request_build
WHERE bors_sha = "<sha>";
```
After that, the server has to be restarted, or you have to send a GET request to its `/perf/onpush`
endpoint.
