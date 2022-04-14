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
## Sign Up
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
## Chat Message
### Send Message
```
mutation ($messageId:Int!,$fromUserId:Int!,$toUserId:Int!,$messageText:String!) {
  sendMessage(messageId:$messageId,fromUserId:$fromUserId,toUserId:$toUserId,messageText:$messageText){
    messageId,
    fromUserId,
    toUserId,
    messageText,
  }
}

{
  "messageId": 10000,
  "fromUserId": 10,
  "toUserId":  500,
  "messageText": "First Message Hello 500",
}
```
### Subscribe Message
* Receiver
```
subscription {
  subscribeMessage(id: 500) {
    messageId,
    fromUserId,
    toUserId,
    messageText,
    sentDatetime,
  }
}
```
* Sender
```
subscription {
  subscribeMessage(id: 10) {
    messageId,
    fromUserId,
    toUserId,
    messageText,
    sentDatetime,
  }
}
```
### Get All Message
* Receiver
```
query {
  getMessageByQuery(id:500){
    messageId,
    fromUserId,
    toUserId,
    messageText,
    sentDatetime,
  }
}
```
* Sender
```
query {
  getMessageByQuery(id:10){
    messageId,
    fromUserId,
    toUserId,
    messageText,
    sentDatetime,
  }
}
```