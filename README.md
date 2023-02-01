# Crypto Prices

The goal of this project is to collect and display crypto prices of a coin from muitlple apis concurrently.
It should also be easy to add new apis, and to maintain.
The user should select a base and a quote coin, and then the candlestics should be gathered from all apis available that
have that base and that quote.

Note that this project was done as a way to learn rust.

## Gathering the data

### Gathering the symbols

In `src/api/apis_models`, the stuct of the data that each api will return is specified, as well as
a way to turn that data into a vector of strings (these strings being the symbols neeeded to request candlesticks)

In `src/api.rs`, there is a generic implementation to make the api call, collect the data and get the vector of symbols.
It takes the stuct of the data as a generic, it the uses the `Api` struct to build the url, makes a request to that url,
and parses it into the struct that was passed.

# Todos

- [x] Get Api Symbols
- [ ] Get Api Candles / Orderbook data
- [ ] Gather data from multiple APIs
    - [ ] Make this concurrent
- [ ] Display the data in a graph