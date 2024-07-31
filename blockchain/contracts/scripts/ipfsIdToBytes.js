const { getBytesFromMultihash } = require('./utils.js');

const ipfsId = 'QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm';

const converted = getBytesFromMultihash(ipfsId);

console.log('as bytes: ', converted);
