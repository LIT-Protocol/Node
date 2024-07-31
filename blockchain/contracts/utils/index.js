const bs58 = require('bs58');
const ethers = require('ethers');

// the below functions are from https://github.com/saurfang/ipfs-multihash-on-solidity
/**
 * @typedef {Object} Multihash
 * @property {string} digest The digest output of hash function in hex with prepended '0x'
 * @property {number} hashFunction The hash function code for the function used
 * @property {number} size The length of digest
 */

/**
 * Partition multihash string into object representing multihash
 *
 * @param {string} multihash A base58 encoded multihash string
 * @returns {Multihash}
 */
function getBytes32FromMultihash(multihash) {
  const decoded = bs58.decode(multihash);

  return {
    digest: `0x${Buffer.from(decoded.slice(2)).toString('hex')}`,
    hashFunction: decoded[0],
    size: decoded[1],
  };
}

/**
 * Partition multihash string into object representing multihash
 *
 * @param {string} multihash A base58 encoded multihash string
 * @returns {Multihash}
 */
function getBytesFromMultihash(multihash) {
  const decoded = bs58.decode(multihash);

  return `0x${Buffer.from(decoded).toString('hex')}`;
}

/**
 * Encode a multihash structure into base58 encoded multihash string
 *
 * @param {Multihash} multihash
 * @returns {(string|null)} base58 encoded multihash string
 */
function getMultihashFromBytes32(multihash) {
  const { digest, hashFunction, size } = multihash;
  if (size === 0) return null;

  // cut off leading "0x"
  const hashBytes = Buffer.from(digest.slice(2), 'hex');

  // prepend hashFunction and digest size
  const multihashBytes = new hashBytes.constructor(2 + hashBytes.length);
  multihashBytes[0] = hashFunction;
  multihashBytes[1] = size;
  multihashBytes.set(hashBytes, 2);

  return bs58.encode(multihashBytes);
}

/**
 * Parse Solidity response in array to a Multihash object
 *
 * @param {array} response Response array from Solidity
 * @returns {Multihash} multihash object
 */
function parseMultihashContractResponse(response) {
  const [digest, hashFunction, size] = response;
  return {
    digest,
    hashFunction: hashFunction.toNumber(),
    size: size.toNumber(),
  };
}

/**
 * Parse Solidity response in array to a base58 encoded multihash string
 *
 * @param {array} response Response array from Solidity
 * @returns {string} base58 encoded multihash string
 */
function getMultihashFromContractResponse(response) {
  return getMultihashFromBytes32(parseMultihashContractResponse(response));
}

function ipfsIdToIpfsIdHash(ipfsId) {
  const multihashStruct = getBytes32FromMultihash(ipfsId);
  const packed = ethers.solidityPacked(
    ['bytes32', 'uint8', 'uint8'],
    [multihashStruct.digest, multihashStruct.hashFunction, multihashStruct.size]
  );
  return ethers.keccak256(packed);
}

function int2ip(ipInt) {
  let ipIntBigInt = BigInt(ipInt);
  return (
    (ipIntBigInt >> 24n) +
    '.' +
    ((ipIntBigInt >> 16n) & 255n) +
    '.' +
    ((ipIntBigInt >> 8n) & 255n) +
    '.' +
    (ipIntBigInt & 255n)
  );
}

function ip2int(ip) {
  return (
    ip.split('.').reduce(function (ipInt, octet) {
      return (ipInt << 8) + parseInt(octet, 10);
    }, 0) >>> 0
  );
}

/**
 * Partition multihash string into object representing multihash
 *
 * @param {string} multihash A base58 encoded multihash string
 * @returns {Multihash}
 */
function getBytesFromMultihash(multihash) {
  const decoded = bs58.decode(multihash);

  return `0x${Buffer.from(decoded).toString('hex')}`;
}

/**
 * Convert a string to bytes
 *
 * @param {string} val A string
 * @returns {Multihash}
 */
function getBytesFromString(val) {
  return `0x${Buffer.from(val).toString('hex')}`;
}

/**
 * Partition multihash string into object representing multihash
 *
 * @param {string} multihash A base58 encoded multihash string
 * @returns {Multihash}
 */
function getMultihashFromBytes(bytes) {
  return bs58.encode(bytes);
}

async function getParamsFromPKPMint(tx, pkpContract) {
  const receipt = await tx.wait();
  let event = receipt.logs.find((l) => {
    if (l instanceof ethers.EventLog) {
      return l.eventName === 'PKPMinted';
    }
    return false;
  });
  if (event) {
    const { tokenId } = event.args;
    let pubkey = await pkpContract.getPubkey(tokenId);
    return { tokenId, pubkey };
  } else {
    // have to parse topic
    const eventName = 'PKPMinted(uint256,bytes)';
    const eventTopic = ethers.id(eventName);
    // console.log("event topic: ", eventTopic);
    const eventLog = receipt.logs.find((log) => log.topics[0] === eventTopic);
    // console.log("event log: ", eventLog);
    const tokenId = eventLog.topics[1];
    // Length of the pubkey in bytes (not characters)
    let pubkeyLength = parseInt(eventLog.data.slice(66, 130), 16);

    // Extract the pubkey (don't forget to multiply pubkeyLength by 2 because each byte is represented by 2 hex characters)
    let pubkey = '0x' + eventLog.data.slice(130, 130 + pubkeyLength * 2);

    return { tokenId, pubkey };
  }
}

module.exports = {
  getBytes32FromMultihash,
  getMultihashFromBytes32,
  parseMultihashContractResponse,
  getMultihashFromContractResponse,
  ipfsIdToIpfsIdHash,
  getBytesFromMultihash,
  getBytesFromString,
  int2ip,
  ip2int,
  getMultihashFromBytes,
  getParamsFromPKPMint,
};
