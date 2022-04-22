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
query {
  getAccounts{
    userId,
    userName,
    email,
    pass,
  }
}

mutation ($userName:String!,$email:String!,$pass:String!) {
  signUp(userName:$userName,email:$email,pass:$pass,){
    token
    error
  }
}

mutation ($email:String!,$pass:String!) {
  signIn(email:$email,pass:$pass,){
    token
    error
  }
}

{
  "userName": "TEST",
  "email": "test@com",
  "pass": "test"
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