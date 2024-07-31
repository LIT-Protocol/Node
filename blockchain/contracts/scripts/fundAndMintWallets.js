const hre = require('hardhat');
const fs = require('fs');
var spawn = require('child_process').spawn;
const nacl = require('tweetnacl');

const { ethers } = hre;
const amountToSend = ethers.parseEther('10');

const MULTISENDER_ADDRESS = '0xBd119B72B52d58A7dDd771A2E4984d106Da0D1DB';
const LIT_TOKEN_ADDRESS = '0x53695556f8a1a064EdFf91767f15652BbfaFaD04';

const generateWallet = () => {
  const wallet = ethers.Wallet.createRandom();
  // console.log("address:", wallet.address);
  // console.log("mnemonic:", wallet.mnemonic.phrase);
  // console.log("privateKey:", wallet.privateKey);
  return wallet;
};

function generateComsKeys() {
  const keys = nacl.box.keyPair();
  return {
    publicKey: '0x' + Buffer.from(keys.publicKey).toString('hex'),
    privateKey: '0x' + Buffer.from(keys.secretKey).toString('hex'),
  };
}

const generateWallets = (walletCount) => {
  const newWallets = [];
  for (let i = 0; i < walletCount; i++) {
    newWallets.push({
      nodeWallet: generateWallet(),
      stakerWallet: generateWallet(),
      comsKeysSender: generateComsKeys(),
      comsKeysReceiver: generateComsKeys(),
    });
  }
  return newWallets;
};

const getSigner = async () => {
  const [deployer] = await ethers.getSigners();
  return deployer;
};

const fundWalletsWithGas = async (nodeOperatorsCredentials) => {
  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    MULTISENDER_ADDRESS,
    signer
  );

  const nodeTx = await multisenderContract.sendEth(
    nodeOperatorsCredentials.map((w) => w.nodeWallet.address),
    { value: amountToSend }
  );
  console.log('fundWalletsWithGas nodeTx: ', nodeTx);

  const stakerTx = await multisenderContract.sendEth(
    nodeOperatorsCredentials.map((w) => w.stakerWallet.address),
    { value: amountToSend }
  );
  console.log('fundWalletsWithGas stakerTx: ', stakerTx);

  await nodeTx.wait();
  console.log('mined nodeTx');
};

const fundWalletsWithTokens = async (nodeOperatorsCredentials) => {
  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    MULTISENDER_ADDRESS,
    signer
  );

  const stakerTx = await multisenderContract.sendTokens(
    nodeOperatorsCredentials.map((w) => w.stakerWallet.address),
    LIT_TOKEN_ADDRESS
  );
  console.log('fundWalletsWithTokens stakerTx: ', stakerTx);
  await stakerTx.wait();
  console.log('stakerTx mined');
};

async function sendGas() {
  // *** 1. Generate wallets
  const nodeOperatorsCredentials = generateWallets(1);

  // *** 2. Fund node and staker wallets with gas
  await fundWalletsWithGas(nodeOperatorsCredentials);
  await fundWalletsWithTokens(nodeOperatorsCredentials);

  const serialized = nodeOperatorsCredentials.map((w) => {
    return {
      nodeWallet: {
        address: w.nodeWallet.address,
        privateKey: w.nodeWallet.privateKey,
        publicKey: w.nodeWallet.publicKey,
      },
      stakerWallet: {
        address: w.stakerWallet.address,
        privateKey: w.stakerWallet.privateKey,
        publicKey: w.stakerWallet.publicKey,
      },
      comsKeysSender: {
        publicKey: w.comsKeysSender.publicKey,
        privateKey: w.comsKeysSender.privateKey,
      },
      comsKeysReceiver: {
        publicKey: w.comsKeysReceiver.publicKey,
        privateKey: w.comsKeysReceiver.privateKey,
      },
    };
  });

  fs.writeFileSync(
    'nodeOperatorsCredentials.json',
    JSON.stringify(serialized, null, 2)
  );
}

(async () => {
  try {
    sendGas();
  } catch (error) {
    console.error(error);
  }
})();
