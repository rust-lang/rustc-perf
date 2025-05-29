# How to use GraphQL with Rust

# GUI Clients (Electron apps)

Use a client to experiment and build your GraphQL query/mutation.

https://insomnia.rest/download

https://docs.usebruno.com

Once you're happy with the result, save your query in a `<query>.gql` file in this directory. It will serve as
documentation on how to reproduce the Rust boilerplate.

# Cynic CLI

Introspect a schema and save it locally:

```sh
cynic introspect \
    -H "User-Agent: cynic/3.4.3" \
    -H "Authorization: Bearer [GITHUB_TOKEN]" \
    "https://api.github.com/graphql" \
    -o schemas/github.graphql
```

Execute a GraphQL query/mutation and save locally the Rust boilerplate:

``` sh
cynic querygen --schema schemas/github.graphql --query query.gql
```

