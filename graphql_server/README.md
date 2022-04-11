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

mutation ($messageId:Int!,$fromUserId:Int!,$toUserId:Int!,$messageText:String!,$sentDatetime:String!) {
  sendMessage(messageId:$messageId,fromUserId:$fromUserId,toUserId:$toUserId,messageText:$messageText,sentDatetime:$sentDatetime,){
    messageId,
    fromUserId,
    toUserId,
    messageText,
    sentDatetime,
  }
}

{
  "messageId": 2,
  "fromUserId": 10,
  "toUserId":  500,
  "messageText": "Second Message Hello",
  "sentDatetime": "2022-04-11-18:23"
}

subscription {
  subscribeMessage {
    messageId,
    fromUserId,
    toUserId,
    messageText,
    sentDatetime,
  }
}
```