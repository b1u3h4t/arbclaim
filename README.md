# $ARB claim

## 使用步骤
1. .env.example 中配置
```shell
PRIVATE_KEY=私钥
ETH_RPC=以太坊节点
ARB_RPC=Arbitrum节点
ENABLE_TRANSFER_TO_BINANCE=是否转账到Binance，为"true"或者"false"
MY_BINANCE_DEPOSIT_ADDRESS=Binance交易所充值$ARB地址
```

2. 重命名配置文件
```shell
mv .env.example .env
```

3. 运行
```shell
source .env
cargo run --release
```