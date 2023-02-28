# Hub Customers
Management of Holaplex Hub project customers.

# Getting Started
```
docker compose up -d
sea migrate up --database-url postgres://postgres:holaplex@localhost:5537/hub_customers
cargo run --bin holaplex-hub-customers
```

Visit [http://localhost:3006/playground](http://localhost:3006/playground) to access GraphQL playground.
