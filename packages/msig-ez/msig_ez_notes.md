

## Random Notes on ITN 2

TODO:

- [ ] Create chart of the funds required to create certain price impact given
  `sqrt_depth` or full depth.

### Motivation

The query of the perp markets before: 

```json
{
  "amm_markets": [
    {
      "market": { ... },
      "amm": {
        "pair": "ubtc:unusd",
        "base_reserve": "39999787612508.833100890451817684",
        "quote_reserve": "40000212388618.884047039184231057",
        "sqrt_depth": "40000000000000.000000000000000000",
        "price_multiplier": "25662.000000000000000000",
        "total_long": "3555009600921.075803994223087994",
        "total_short": "3554797213429.908904884674905678"
      }
    },
    {
      "market": { ... },
      "amm": {
        "pair": "ueth:unusd",
        "base_reserve": "39999978845205.285782140219073228",
        "quote_reserve": "40000021154805.902357261868231385",
        "sqrt_depth": "40000000000000.000000000000000000",
        "price_multiplier": "1628.580000000000000000",
        "total_long": "1595541783.743771256130502505",
        "total_short": "1574386989.029553396349575733"
      }
    }
  ]
}
```

The `sqrt_depth` is 4E12, meaning `k` is a whopping 16E24.

> If we set the BTC pool to have a depth of 200 btc, that represents 10 million
 nusd pool depth, and a 10k orders will have a price impact of 0.4%

Currently on ITN, most of the orders are around 100 nusd
few of them have more than that

This means calling `{pair: "ubtc:unusd", depth_mult: 40,000,000,000,000,000}`

(200 1e6 * 200 1e6)

maybe we can do less than that since faucet gives 100 nusd we would need 100k
nusd to converge mark price and index price if we were to do 200btc depth