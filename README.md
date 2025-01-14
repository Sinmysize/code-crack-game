# Code Game

A very stupid game about guessing a code.
Currently none of the endpoints will be active until Jan 31.

## Table of Contents

- [About Code Game](#about-code-game)
- [How to play](#how-to-play)
- [Creating an account](#creating-an-account)
- [Sending a code](#sending-a-code)
- [Getting user statistics](#sending-a-code)
- [Deleting your account](#deleting-your-account)
- [Getting server information](#getting-server-info)
- Getting the leaderboard (WIP)

## About Code Game

Code Game is a pretty dumb project I made to introduce myself into the [Rust Programming Language](https://www.rust-lang.org/). In Code Game, you attempt to guess/crack a 4 character code. *This may change in the future if it gets too easy for anyone who might be playing this*. I thought it would be funny to make it more into a game to see who can guess the most codes with or without a program.

The likely hood of me doing major updates to this project is pretty slim since I am working on a much larger scale project, so for the few who may enjoy this project do not expect much from this.

All server code is open source.

## How to Play

This entire game is based off of making HTTP requests to the url https://game.sinmysize.com/code-game/ in order to guess/crack a code.

You can make these requests through tools such as [Insomnia by Kong](https://insomnia.rest/), [Postman](https://www.postman.com/), etc. to test the endpoints and creating a program or script using languages such as [Node.js](https://nodejs.org/en), [Python](https://www.python.org/), [Rust](https://www.rust-lang.org/), or any language that can make HTTP requests to try and guess/crack the code.

Each code will be a string of lowercase letters and numbers that is determined by the code length the server has set. To find the code length, read [Getting Server Info](#getting-server-info).

- First, [create an account](#creating-an-account) so you can begin sending codes to the server.

- Second, make a request to [send a code](#sending-a-code) to the server and see if you guessed it correctly.

- Finally, repeat the second step until you successfully guessed a code.

The methods of cracking the codes are primarily up to you. I hope you enjoy this small silly game I made.

# REST-API

## Creating an Account

- **URL** : `/code-game/user/register`

- **Method** : `POST`

- **Auth** : `None`

- **Headers** : `Content-Type: application/json`

**Body** :

```json
{
    username: "String",
    password: "String"
}
```

### Response

- **Success** : `200 OK`

    - Returns 200 when you successfully register an account.

- **Error** : `418 I'm a teapot`

    - Returns 418 if an account with the same username exists. *Do not ask why I chose 418*.

## Sending a Code

- **URL** : `/code-game/code`

- **Method** : `POST`

- **Auth** : `None`

- **Headers** : `Content-Type: application/json`

**Body** :

```json
{
    code: "String",
    username: "String",
    password: "String" // Ensures the code is sent by YOU
}
```

### Response

- **Success** : `200 OK`

    - Returns 200 if you guess the code correctly.

- **Failed Code** : `401 Unauthorized`

    - Returns 401 if you guess the code incorrectly.

## Getting User Statistics

- **URL** : `/code-game/user/{username}`

- **Method** : `GET`

- **Auth** : `None`

- **Headers** : `Content-Type: application/json`

- **Body** : `None`

### Response

- **Success** : `200 OK`

    - Returns 200 and user's statistics if user exists.

        - **username** : Given username.
        - **cracked** : Number of codes guessed correctly.
        - **rank** : Global rank of user.

*Example*:
```json
{
    username: "John Doe",
    cracked: "5",
    rank: "3"
}
```

- **Failure** : `404 Not Found`

    - Returns 404 if the user does not exist.


## Deleting Your Account

### **[ DISCLAIMER ]**

**ONCE ACCOUNT IS DELETED IT `CANNOT BE RECOVERED`**
    
**ALL PROGRESS WILL BE `PERMANENTLY DELETED`**

#

- **URL** : `/code-game/user/{username}`

- **Method** : `DELETE`

- **Auth** : `None`

- **Headers** : `Content-Type: application/json`

**Body** :

```json
{
    username: "String",
    password: "String" // Ensures the request is sent by YOU
}
```

### Response

- **Success** : `200 OK`

    - Returns 200 if your account is successfully deleted.

- **Error** : `403 Forbidden`

    - Returns 403 for a few reasons:
        1. Account has already been deleted.
        2. Account does not exist.
        3. Credentials given are not correct.

## Getting Server Info

- **URL** : `/code-game/statistics`

- **Method** : `GET`

- **Auth** : `None`

- **Headers** : `Content-Type: application/json`

- **Body** : `None`


### Response

- **Success** : `200 OK`

    - Will always return 200 and server stats.
        - **code_length** : Length of code.
        - **server_uptime** : Time since server ran in seconds.

*Example* 
```json
{
    code_length: "4",
    server_uptime: "248400"
}
```

## Getting the Leaderboard (WIP)

- **URL** : `/code-game/user/leaderboard`

- **Method** : `GET`

- **Auth** : `None`

- **Headers** : `Content-Type: application/json`

- **Body** : `None`


### Response

- **Success** : `200 OK`

    - Will always return 200.

*Example* 
```json
{

}
```
