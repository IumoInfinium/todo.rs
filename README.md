# Todo Rest API in Rust

Uses Axum as a web service which is based on the Tokio, a asynchronous library. It also uses `serde` for serialization and deserialization of data.

In short it uses

- Axum (a miniature web service)
- Tokio (asynchronous code)
- Serde (serialization and deserialization)

### Endpoints

- GET `/todos` -> get the list of all the todods
- POST `/todos` -> create a new todo, given an input string
- PATCH `/todos/:id` -> Patch a exising todo given the id
- DELETE `/todos/:id` -> Delete a todo using its id.

> Note: Id is the UUID using the crate `uuid`


> Just learning rust