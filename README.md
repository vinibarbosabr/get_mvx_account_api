# get_mvx_account_api

This is a Rust tool to fetch account data from a given MultiversX address.
It makes a GET request (using reqwest) to the MultiversX Gateway [https://gateway.multiversx.com], using the parameter `address`.

Data returned is:
```
Account {
    address:
    nonce:
    balance:
}
```
After running the program, it will ask for an address input, resulting in the above output.

This is my first Rust project written from scratch.
I used two references and sources to build this code as part of a Dojo challenge.

Feel free to implement it to your code, if it's useful.
Credits are appreciated.

Follow me at @vinibarbosabr on X.