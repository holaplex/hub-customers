# Hub Customers

Customers are entities that are associated with a project within the Holaplex Hub ecosystem. Customers are primarily responsible for managing their own wallets within the project, including creating and managing wallets, as well as making transactions. Customers can also be associated with multiple projects, and can manage multiple wallets within a single project.

When a customer is created, they are assigned a unique ID within the Holaplex Hub ecosystem. Customers are associated with a specific project by providing the project ID when creating a new customer.

Once a customer is created, they can create, delete, and manage wallets associated with their account. Customers can also view transaction histories for their wallets, and submit transactions on their behalf.

Overall, customers are a critical component of the Holaplex Hub ecosystem, providing a way for clients to manage their own wallets and conduct transactions within a secure and easy-to-use platform.

# Getting Started
```
docker compose up -d
sea migrate up --database-url postgres://postgres:holaplex@localhost:5537/hub_customers
cargo run --bin holaplex-hub-customers
```

Visit [http://localhost:3006/playground](http://localhost:3006/playground) to access GraphQL playground.
