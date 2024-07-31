// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const fs = require('fs');
var spawn = require('child_process').spawn;
const nacl = require('tweetnacl');
const { ethers } = hre;
const chainName = hre.network.name;
const rpcUrl = hre.network.config.url;

const recipients = [
  '0x47D949fb80900323BB378378f83a33ac637B3f74',
  '0x3f622eAEd12310e00D29B56037e66C070a678aa6',
  '0x4AF608Ba5E5f8DD1957AE8bced87a937FE28702d',
  '0xD2B56De2b5558996bEA1e1912e95e3B89fE41529',
  '0x6B6013d7DD3216D1BB2D810633714E024d03Aa60',
  '0x7aB0bcC2399a2A0Fc3aAEF81339D4BFceea3cBdf',
  '0xA8A31B5e945eA0e1B86c1c4136e071dA4d603228',
  '0x9C603D86f7E85d88D85E2fdDd0d4c03DA965f008',
  '0xdD1Bf187E92E76Afb1c33D0e0f141741773b3c2F',
  '0xb9FCC4024D1B8CFaeE9FB63782c24fCB7B996205',
  '0x2667FF355FC130c0ffAf9D42AdD60e19aaCDeEB4',
  '0x1A1F2311Ed60C5D6e3195f38dD0CC0E05D43241E',
  '0x42355e7dc0A872C465bE9DE4AcAAAcB5709Ce813',
  '0x5Da5b76130F00cA4bd7D15FBAea54c20E4B5fC6a',
  '0xAdE5f302c4Fd2fEDcC9ea850dDACe7Ac78f38e7a',
  '0x1D64CD46561b43De59258c42E42F629f289755E1',
  '0x8271b2FDFc8a8633fCFb25F9e822a0a96d665203',
  '0xdE99C0B9aB0684CD5B6547E6753A5c63C5C520E9',
  '0xdb14c6635e42F616c50F26a1f27E1B622Eb7F1A7',
  '0xED13Db1122526564481E7CeB4798ADf50E1F4CF3',
  '0x0101D1813e9F90e36906DcE13adacB4AcEaA4067',
];

const multisenderContractAddress = '0x7dda866ea0bbF06959008D579daDefB6A8a4fAE9';
const litTokenContractAddress = '0x53695556f8a1a064EdFf91767f15652BbfaFaD04';
// split between all nodes
const amount = ethers.parseEther('10');

console.log('Funding nodes: ' + JSON.stringify(recipients, null, 2));

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const getSigner = async () => {
  const [deployer] = await ethers.getSigners();
  return deployer;
};

const ip2int = (ip) => {
  return (
    ip.split('.').reduce(function (ipInt, octet) {
      return (ipInt << 8) + parseInt(octet, 10);
    }, 0) >>> 0
  );
};

const fundWalletsWithGas = async () => {
  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    multisenderContractAddress,
    signer
  );
  console.log('multisender contract address is ', multisenderContractAddress);

  const tx = await multisenderContract.sendEth(recipients, {
    value: amount * BigInt(recipients.length),
  });
  console.log('fundWalletsWithGas tx: ', tx);
  await tx.wait();

  console.log('mined');
};

const fundWalletsWithTokens = async () => {
  const signer = await getSigner();

  // deposit into wlit
  const litTokenContract = await ethers.getContractAt(
    'WLIT',
    litTokenContractAddress,
    signer
  );

  const totalTokens = amount * BigInt(recipients.length);

  const depositTx = await litTokenContract.deposit({
    value: totalTokens,
  });
  await depositTx.wait();

  // send to multisender
  const transferTx = await litTokenContract.transfer(
    multisenderContractAddress,
    totalTokens
  );
  await transferTx.wait();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    multisenderContractAddress,
    signer
  );

  const stakerTx = await multisenderContract.sendTokens(
    recipients,
    litTokenContractAddress
  );
  console.log('fundWalletsWithTokens stakerTx: ', stakerTx);
  await stakerTx.wait();
  console.log('stakerTx mined');
};

async function main() {
  await fundWalletsWithGas();
  await fundWalletsWithTokens();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
