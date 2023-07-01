# Useful queries
This document contains useful queries that should be performed manually in exceptional situations.

## Remove data for an artifact from the DB
This is important for situations where there is some compilation error for a stable benchmark,
with a stable release of the compiler. While this should be rare, it happens sometimes e.g. because
of future incompatibility lints turning into errors.

The set of queries below show an example of removing the measured data and errors for the `html5ever`
benchmark for Rust `1.69` and `1.70`.
```sql
-- Remove 1.69 and 1.70 html5ever data and errors
-- Delete errors
DELETE FROM error
WHERE aid IN (
    SELECT id
    FROM artifact
    WHERE name IN ('1.69.0', '1.70.0') AND
          type = 'release'
);

-- Delete collection records. This will enable `bench_published` to re-run the benchmarks.
DELETE FROM collector_progress
WHERE aid IN (
    SELECT id
    FROM artifact
    WHERE name IN ('1.69.0', '1.70.0') AND
          type = 'release'
) AND step IN ('html5ever');

-- Note that there probably won't be any results in pstat, since the benchmark did not compile.
-- But it's possible that some configurations of it have compiled before. To avoid skewing
-- the average, we should thus still delete.

-- Delete compile benchmark data
DELETE FROM pstat
WHERE aid IN (
    SELECT id
    FROM artifact
    WHERE name IN ('1.69.0', '1.70.0') AND
          type = 'release'
);

-- Delete runtime benchmark data
DELETE FROM runtime_pstat
WHERE aid IN (
    SELECT id
    FROM artifact
    WHERE name IN ('1.69.0', '1.70.0') AND
          type = 'release'
);
```

After running these queries, the benchmark should be fixed so that it is compilable again, and then
the version should be re-benchmarked:
```bash
$ cargo run --bin collector bench_published <version>
```
