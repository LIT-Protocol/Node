const { ethers } = require('ethers');
const ipfsId = 'QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm';

const converted = ethers.keccak256(ethers.toUtf8Bytes('LIT_ACTION_' + ipfsId));

console.log('as bytes: ', converted);
