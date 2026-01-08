# Useful queries and commands
This document contains useful queries and commands that should be performed manually in exceptional
situations.

## Remove data for an artifact from the DB
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
