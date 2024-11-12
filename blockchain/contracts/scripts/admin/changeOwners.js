const { hre } = require('hardhat');
const fs = require('fs');

/* contracts we aren't changing:
 * - DomainWalletRegistryAddress - changing to diamond, not used in current deployment anyway
 * - DomainWalletOracleAddress- changing to diamond, not used in current deployment anyway
 * - hdKeyDeriver - no Owner
 * - LitToken - Not used in current deployment, littoken is native gas on chronicle
 * - PKPNFTMetadata - no owner
 */

// put your new owner here.  YOU MUST HAVE THE ABILITY TO SEND TXNS FROM HERE OR ELSE YOU COULD END UP SETTING AN OWNER THAT YOU CAN'T EVER CHANGE
const NEW_OWNER = '0xdDD9B576f0ec66Aa5F0573Ab223807b69A5b666A';

const LIT_CORE_ADDRESSES_PATH =
  '../../../networks/datil-prod/deployed-lit-core-contracts-temp.json';
const LIT_NODE_ADDRESSES_PATH =
  '../../../networks/datil-prod/deployed-lit-node-contracts-temp.json';

// const LIT_CORE_ADDRESSES_PATH = "./deployed-lit-core-contracts-temp.json";
// const LIT_NODE_ADDRESSES_PATH = "./deployed-lit-node-contracts-temp.json";

// let's not change the lit core admin for now
// const LIT_CORE_ADMIN_PK = process.env.LIT_CORE_ADMIN_PK;

const CONTRACTS = {
  BackupRecovery: {
    name: 'BackupRecovery',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'backupRecoveryContractAddress',
  },
  Staking: {
    name: 'Staking',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'stakingContractAddress',
  },
  StakingBalances: {
    name: 'StakingBalances',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'stakingBalancesContractAddress',
  },
  PubkeyRouter: {
    name: 'PubkeyRouter',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'pubkeyRouterContractAddress',
  },
  PKPNFT: {
    name: 'PKPNFT',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'pkpNftContractAddress',
  },
  RateLimitNFT: {
    name: 'RateLimitNFT',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'rateLimitNftContractAddress',
  },
  PKPPermissions: {
    name: 'PKPPermissions',
    ownerType: 'diamond',
    project: 'node',
    addressKey: 'pkpPermissionsContractAddress',
  },
  Allowlist: {
    name: 'Allowlist',
    ownerType: 'ownable',
    project: 'node',
    addressKey: 'allowlistContractAddress',
  },
  PKPHelper: {
    name: 'PKPHelper',
    ownerType: 'ownable',
    project: 'node',
    addressKey: 'pkpHelperContractAddress',
  },
  // this contract has no owner
  // "PKPNFTMetadata": {
  //     name: "PKPNFTMetadata",
  //     ownerType: "roles",
  //     project: "node",
  //     addressKey: "pkpNftMetadataContractAddress"
  // },
  ContractResolver: {
    name: 'ContractResolver',
    ownerType: 'roles',
    project: 'core',
    addressKey: 'contractResolver',
    probableAdmin: '0x046BF7BB88E0e0941358CE3F5A765C9acddA7B9c',
  },
  // let's not change this right now
  // ReleaseRegister: {
  //     name: "ReleaseRegister",
  //     ownerType: "roles",
  //     project: "core",
  //     addressKey: "releaseRegisterContractAddress",
  //     probableAdmin: "0xB77AEBbC262Bb809933D991A919A0e4A6A3b2f65",
  // },
};

const ADMIN_ROLE = ethers.keccak256(ethers.toUtf8Bytes('ADMIN'));

// function getDiamondAbi(contractName) {
//     const abiPath = `./abis/${contractName}.abi`;
//     const abi = JSON.parse(fs.readFileSync(abiPath));
//     return abi;
// }

// // uses the default hardhat wallet to set new owners
// async function addNewOwner(contractResolverAddress) {
//     const resolver = await ethers.getContractAt("ContractResolver", contractResolverAddress);

//     //contract = new Contract("dai.tokens.ethers.eth", abi, provider)
// }

async function changeOwners(contracts, addresses) {
  const previousOwner = (await ethers.getSigners())[0];
  console.log('previousOwner', previousOwner.address);
  if (NEW_OWNER.toLowerCase() === previousOwner.address.toLowerCase()) {
    throw new Error(
      'You are already the owner.  Running this script with the same owner will break the role-based contracts'
    );
  }
  for (contractName of Object.keys(contracts)) {
    const contractInfo = contracts[contractName];
    const contractAddress = addresses[contractInfo.addressKey];
    await changeOwner({ ...contractInfo, address: contractAddress });
  }
}

async function changeOwner(contractInfo) {
  console.log(`changeOwner with ${JSON.stringify(contractInfo, null, 2)}`);
  if (contractInfo.ownerType === 'diamond') {
    return changeDiamondOwner(contractInfo);
  } else if (contractInfo.ownerType === 'ownable') {
    return changeOwnableOwner(contractInfo);
  } else if (contractInfo.ownerType === 'roles') {
    return changeRoleAdmin(contractInfo);
  } else {
    throw new Error(`Invalid ownerType for contract ${contractInfo.name}`);
  }
}

async function changeDiamondOwner(contractInfo) {
  const contract = await ethers.getContractAt(
    'OwnershipFacet',
    contractInfo.address
  );
  const tx = await contract.transferOwnership(NEW_OWNER);
  console.log(
    `Transferring ownership of ${contractInfo.name} to ${NEW_OWNER} with tx hash ${tx.hash}`
  );
  await tx.wait();
  console.log(`Ownership of ${contractInfo.name} transferred to ${NEW_OWNER}`);
}

async function changeOwnableOwner(contractInfo) {
  const contract = await ethers.getContractAt(
    contractInfo.name,
    contractInfo.address
  );
  const tx = await contract.transferOwnership(NEW_OWNER);
  console.log(
    `Transferring ownership of ${contractInfo.name} to ${NEW_OWNER} with tx hash ${tx.hash}`
  );
  await tx.wait();
  console.log(`Ownership of ${contractInfo.name} transferred to ${NEW_OWNER}`);
}

async function changeRoleAdmin(contractInfo) {
  if (contractInfo.project !== 'core') {
    throw new Error(
      `Only core contracts can be changed with the changeRoleAdmin function.  ${contractInfo.name} is not a core contract`
    );
  }
  // const litCoreAdminWallet = new ethers.Wallet(
  //     LIT_CORE_ADMIN_PK,
  //     ethers.provider
  // );
  const hardhartAdminWallet = (await ethers.getSigners())[0];

  let previousOwner = hardhartAdminWallet;
  // if (contractInfo.owner === litCoreAdminWallet.address) {
  //     previousOwner = litCoreAdminWallet;
  // } else if (contractInfo.owner === hardhartAdminWallet.address) {
  //     previousOwner = hardhartAdminWallet;
  // } else {
  //     throw new Error(
  //         `Could not find private key for owner for ${contractInfo.name}.  Owner address is ${contractInfo.owner}`
  //     );
  // }

  if (previousOwner.address.toLowerCase() === NEW_OWNER.toLowerCase()) {
    throw new Error(
      `${NEW_OWNER} is already the owner.  Running this script with the same owner will break the role-based contracts`
    );
  }

  const contract = (
    await ethers.getContractAt(contractInfo.name, contractInfo.address)
  ).connect(previousOwner);
  const tx = await contract.grantRole(ADMIN_ROLE, NEW_OWNER);
  console.log(
    `Transferring admin role of ${contractInfo.name} to ${NEW_OWNER} with tx hash ${tx.hash}`
  );
  await tx.wait();
  console.log(`Admin role of ${contractInfo.name} transferred to ${NEW_OWNER}`);
  const renounceTx = await contract.renounceRole(
    ADMIN_ROLE,
    previousOwner.address
  );
  console.log(
    `Renouncing admin role of ${contractInfo.name} from ${previousOwner.address} with tx hash ${renounceTx.hash}`
  );
  await renounceTx.wait();
  console.log(
    `Admin role of ${contractInfo.name} renounced by ${previousOwner.address}`
  );
}

async function getOwners(contracts, addresses) {
  const owners = {};
  for (contractName of Object.keys(contracts)) {
    const contractInfo = contracts[contractName];
    const contractAddress = addresses[contractInfo.addressKey];
    const owner = await getOwner({
      ...contractInfo,
      address: contractAddress,
    });
    owners[contractName] = { ...contractInfo, owner };
  }
  return owners;
}

function getOwner(contract) {
  if (contract.ownerType === 'diamond') {
    return getDiamondOwner(contract);
  } else if (contract.ownerType === 'ownable') {
    return getOwnableOwner(contract);
  } else if (contract.ownerType === 'roles') {
    return getRoleAdmin(contract);
  } else {
    throw new Error(`Invalid ownerType for contract ${contract.name}`);
  }
}

async function getDiamondOwner(contractInfo) {
  const contract = await ethers.getContractAt(
    'OwnershipFacet',
    contractInfo.address
  );
  const owner = await contract.owner();
  return owner;
}

async function getOwnableOwner(contractInfo) {
  const contract = await ethers.getContractAt(
    contractInfo.name,
    contractInfo.address
  );
  const owner = await contract.owner();
  return owner;
}

async function getRoleAdmin(contractInfo) {
  const contract = await ethers.getContractAt(
    contractInfo.name,
    contractInfo.address
  );
  const hasRole = await contract.hasRole(
    ADMIN_ROLE,
    contractInfo.probableAdmin
  );
  if (hasRole) {
    return contractInfo.probableAdmin;
  }
  throw new Error(`Could not determine owner for ${contractInfo.name}`);
}

const go = async () => {
  // saved for later - we don't need to change the core admin right now
  // if (!LIT_CORE_ADMIN_PK || LIT_CORE_ADMIN_PK == "") {
  //     throw new Error("LIT_CORE_ADMIN_PK environment variable is not set.");
  // }

  const nodeAddresses = JSON.parse(fs.readFileSync(LIT_NODE_ADDRESSES_PATH));
  const coreAddresses = JSON.parse(fs.readFileSync(LIT_CORE_ADDRESSES_PATH));
  const addresses = {
    ...nodeAddresses,
    ...coreAddresses,
  };
  // check ownership of contracts
  const contractsWithOwners = await getOwners(CONTRACTS, addresses);
  console.log(
    `Owners before change: ${JSON.stringify(contractsWithOwners, null, 2)}`
  );

  await changeOwners(contractsWithOwners, addresses);
  console.log('Owners changed');

  const contractsWithNewProbableOwners = { ...CONTRACTS };
  for (contractName of Object.keys(contractsWithNewProbableOwners)) {
    const contractInfo = contractsWithNewProbableOwners[contractName];
    if (contractInfo.project === 'core') {
      contractsWithNewProbableOwners[contractName].probableAdmin = NEW_OWNER;
    }
  }

  const newOwners = await getOwners(contractsWithNewProbableOwners, addresses);
  console.log(`Owners after change: ${JSON.stringify(newOwners, null, 2)}`);

  for (contractName of Object.keys(newOwners)) {
    const contractInfo = newOwners[contractName];
    if (contractInfo.owner.toLowerCase() !== NEW_OWNER.toLowerCase()) {
      throw new Error(
        `Ownership of ${contractInfo.name} was not transferred to ${NEW_OWNER}`
      );
    }
  }
};

if (require.main === module) {
  go()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });
}
