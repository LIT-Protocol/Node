const ethers = require('ethers');

const pubkey =
  '0x0381bb0e93a94063d92fc4e5e22e91fc7ffa9335ae35c16d69341d1a35596594c9';

const keyPart1Bytes = ethers.dataSlice(pubkey, 0, 32);
const keyPart2Bytes = ethers.zeroPadValue(ethers.dataSlice(pubkey, 32), 32);
const keyLength = pubkey.replace(/^0x/, '').length / 2;
const keyType = 2;

const tokenId = ethers.keccak256(pubkey);

console.log('tokenId', tokenId);
console.log('keyPart1Bytes', keyPart1Bytes);
console.log('keyPart2Bytes', keyPart2Bytes);
console.log('keyLength', keyLength);
console.log('keyType', keyType);
