# service
### Diesel+GraphQL
  
# How to run diesel
  * run server
  In graphql_server
  ```
   cargo run 
  ```
  * launch playground
  In Host PC
  ```
   http://localhost:5050/graphql
  ```

# Schema
```
mutation ($userId:Int!,$userName:String!,$loginName:String!,$pass:String!) {
  signUp(userId:$userId,userName:$userName,loginName:$loginName,pass:$pass,){
    userId,
    userName,
    loginName,
    pass,
  }
}

{
  "userId": 10,
  "userName": "TEST",
  "loginName": "test@email.com",
  "pass": "password"
}

query {
  getAccountByQuery(id:10){
    userId,
    userName,
    loginName,
    pass,
  }
}

subscription {
  subscribeAccount {
    userId,
    userName,
    loginName,
    pass,
  }
}
```