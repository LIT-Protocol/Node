const { ethers } = require('ethers');
const nacl = require('tweetnacl');
const { fromString } = require('uint8arrays/from-string');
const { toString } = require('uint8arrays/to-string');

const senderKeyPair = nacl.box.keyPair();
const receiverKeyPair = nacl.box.keyPair();

const senderComsKeys = {
  publicKey: `0x${toString(senderKeyPair.publicKey, 'base16')}`,
  secretKey: `0x${toString(senderKeyPair.secretKey, 'base16')}`,
};

const receiverComsKeys = {
  publicKey: `0x${toString(receiverKeyPair.publicKey, 'base16')}`,
  secretKey: `0x${toString(receiverKeyPair.secretKey, 'base16')}`,
};

const stakerWallet = ethers.Wallet.createRandom();
let stakerWalletJson = {
  address: stakerWallet.address,
  privateKey: stakerWallet.privateKey,
  publicKey: stakerWallet.publicKey,
  mnemonic: stakerWallet.mnemonic.phrase,
};
const nodeWallet = ethers.Wallet.createRandom();
let nodeWalletJson = {
  address: nodeWallet.address,
  privateKey: nodeWallet.privateKey,
  publicKey: nodeWallet.publicKey,
  mnemonic: nodeWallet.mnemonic.phrase,
};
console.log(
  JSON.stringify(
    {
      staker: stakerWalletJson,
      node: nodeWalletJson,
      senderComsKeys,
      receiverComsKeys,
    },
    null,
    2
  )
);
