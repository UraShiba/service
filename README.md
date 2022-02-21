# service
Hi

# How to set up diesel
  In Local
  Confirme IPv4
  ```
  docker network inspect my-network
  ```

In container (client)
```
  echo DATABASE_URL=postgres://admin:admin@<IPv4-addrress>/admin > .env
  diesel setup
  diesel print-schema > src/schema.rs
  ```
  
# How to run disel
  * Build server repo
  ```
   cargo build 
  ```
  * Insert data to Database
  ```
   cargo run insert 
  ```
  * Delete data to Database
  ```
   cargo run delete <id> 
  ```
 * Show specific data from Database
  ```
   cargo run query <id> 
 ```
 * Update data in Database
  ```
   cargo run update_pass <id> <pass> 
  ```
