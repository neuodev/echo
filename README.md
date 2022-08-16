# Echo

<p align="center">
    <img src="./icon.gif" alt="echo" title="echo" />
</p>

Echo is a telegram bot that can send hourly updates about the current price of USD/EGP, USD/ASR and gold prices.

You can find the bot on telgram with by searching about `@echo_v1_bot` and then you can start getting hourly updates about the prices

## Message Example

```
Prices updates 🔥🔥
1 USD = 19.15 EGP
1 USD = 3.75 SAR
1 SAR = 5.10 EGP
📈 Gold prices (1g) 👇👇
Gold 24k ⏩ 1093.94 EGP
Gold 22k ⏩ 1002.78 EGP
Gold 21k ⏩ 957.20 EGP
Gold 20k ⏩ 911.62 EGP
Gold 18k ⏩ 820.46 EGP
```

# This is on your own you need these three env variables

```bash
BOT_AUTH_TOKEN=<YOUR Key> # get it from https://core.telegram.org/bots/api follow the steps
EXCHAINGE_RATES_API_TOKEN = <YOUR KEY> # from https://openexchangerates.org
GOLD_API_TOKEN= <YOUR KEY> # from https://www.goldapi.io/
```

# References

[Telegram API](https://core.telegram.org/bots/api) To interact with the bot channel

[Open exchange rates](https://openexchangerates.org) To get the latest prices for every fiat curreny

[Gold API](https://www.goldapi.io) Get the latest price of gold
