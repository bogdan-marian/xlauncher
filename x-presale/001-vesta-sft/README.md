# X Launcher presale contract

This is the X Launcher presale contract.
Is a very simple contract and al the storage values are initialized at deploy time

## The flow

- owner deploys the contract
- owner uses `fundContract` in order to add specific token funds to the contract
- clients can use `buy` endpoint in order to buy specific amounts of available token funds. The contract decides based
  on the stored `price` field and the paid `EGLD` value the amount that will be transferred in the client wallet.
- if `max_balance_val` <= `client_total_balance` then the `buy` method will fail and the contract will refuse the
  current request.

## future development

We plan in the future version of this contract to keep track for each client individually of the amount of funds
purchased and fail if required to fail by checking against storage tracked values.

## xlauncher wallets

- presale and staking wallet: `erd12euqqtr68sqhdgy8xmzdtasvjgs00ju23cek30t33484qz3t4d6qz7e365`

