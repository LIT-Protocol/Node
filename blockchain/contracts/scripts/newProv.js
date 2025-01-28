import hre from 'hardhat';
const { ethers } = hre;

function newWallet() {
  const wallet = ethers.Wallet.createRandom();
  let walletJson = {
    address: wallet.address,
    privateKey: wallet.privateKey,
    publicKey: wallet.publicKey,
    mnemonic: wallet.mnemonic.phrase,
  };
  return walletJson;
}

const generateConfigFromTemplate = (wallets) => {
  return `
Admin Private Key: ${wallets.subnetAdmin.privateKey}
Wallet Private Key: ${wallets.subnetProv.privateKey}


Lit subnet Owner for LIT_SUBNET_OWNER_ADDRESS: 
${JSON.stringify(wallets.subnetOwner, null, 2)}


Lit subnet admin for LIT_SUBNET_ADMIN_PUBLIC_KEY:
${JSON.stringify(wallets.subnetAdmin, null, 2)}

Lit subnet prov wallet for LIT_SUBNET_PROV_ADDRESS:
${JSON.stringify(wallets.subnetProv, null, 2)}


ENV vars all in one line:

LIT_SUBNET_OWNER_ADDRESS=${
    wallets.subnetOwner.address
  } LIT_SUBNET_ADMIN_PUBLIC_KEY=${
    wallets.subnetAdmin.publicKey
  } LIT_SUBNET_PROV_ADDRESS=${wallets.subnetProv.address}
  `;
};

function main() {
  const subnetOwner = newWallet();
  const subnetAdmin = newWallet();
  const subnetProv = newWallet();

  const config = generateConfigFromTemplate({
    subnetOwner,
    subnetAdmin,
    subnetProv,
  });

  console.log(config);
}

main();
