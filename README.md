# service
Hi

# How to set up diesel
  In Local
  IPv4の確認
  * docker network inspect my-network

In container
  * echo DATABASE_URL=postgres://admin:admin@<IPv4-addrress>/admin > .env
  * diesel setup
  * diesel print-schema > src/schema.rs
  
# How to run disel
  * cargo build
  Dataの挿入
  * cargo run insert
  Dataの削除
  * cargo run delete <id>
  Dataの指定
  * cargo run query <id>
  DataのUpdate
  * cargo run update_pass <id> <pass>
