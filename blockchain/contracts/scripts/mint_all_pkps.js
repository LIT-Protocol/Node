const hre = require('hardhat');
const fs = require('fs');
var spawn = require('child_process').spawn;

const { ethers } = hre;
const chainName = hre.network.name;
const rpcUrl = hre.network.config.url;

const provider = ethers.provider;

const getContract = async (contractName, addr) => {
  const Factory = await ethers.getContractFactory(contractName);
  return Factory.attach(addr);
};

async function mintAllPKPs() {
  const PKPNFT = await getContract(
    'PKPNFT',
    '0x8F75a53F65e31DD0D2e40d0827becAaE2299D111'
  );
  const Multiminter = await getContract(
    'Multiminter',
    '0x58981CD51EF3534844B7688aB31AA2b8B1bA3A6f'
  );
  const wallets = JSON.parse(
    fs.readFileSync(
      '/Users/chris/Documents/WorkStuff/LIT/lit-assets/blockchain/contracts/nodeOperatorsCredentials.json'
    )
  );
  console.log(`Got ${wallets.length} wallets.`);
  const total = await PKPNFT.getUnmintedRoutedTokenIdCount(2);
  const mintCost = await PKPNFT.mintCost();
  const startTime = Date.now();
  const amountToMint = 75;
  const batchSize = 40;
  for (let i = 0; i < total; i += batchSize) {
    const batchPromises = [];
    for (let j = 0; j < batchSize && i + j < total; j++) {
      const wallet = wallets[(i + j) % wallets.length];
      const walletPrivateKey = new ethers.Wallet(wallet.privateKey, provider);
      const transaction = async () => {
        let retries = 30;
        let delay = 500; // Initial delay in ms
        while (retries) {
          try {
            const tx = await PKPNFT.connect(walletPrivateKey).mintNext(2, {
              value: mintCost,
            });
            return tx;
          } catch (error) {
            console.error(
              `Error in mintNext: ${error.message}. Retrying in ${delay}ms...`
            );
            await new Promise((resolve) => setTimeout(resolve, delay));
            retries--;
            delay *= 2; // Exponential backoff
            if (!retries) throw error;
          }
        }
      };
      batchPromises.push(transaction());
    }
    await Promise.all(batchPromises);
    console.log(`Progress: ${i} of ${total}`);
    let elapsedTime = Date.now() - startTime;
    let estimatedTime = elapsedTime / i;
    let remainingTime = estimatedTime * (total - i);
    let etaHours = Math.floor(remainingTime / 3600000);
    let etaMinutes = Math.floor((remainingTime % 3600000) / 60000);
    let etaSeconds = Math.floor((remainingTime % 60000) / 1000);
    console.log(
      `Estimated time to completion: ${etaHours} hours, ${etaMinutes} minutes, and ${etaSeconds} seconds.`
    );
  }
}

(async () => {
  try {
    mintAllPKPs();
  } catch (error) {
    console.error(error);
  }
})();
