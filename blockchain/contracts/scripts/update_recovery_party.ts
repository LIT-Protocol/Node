//RUN: npx hardhat run --network localchain scripts/update_recovery_party.ts
import hre from 'hardhat';
import inquirer from 'inquirer';

const { ethers } = hre;
async function run() {
  if (
    !process.env.ADMIN_PRIVATE_KEY ||
    !process.env.BACKUP_RECOVERY_CONTRACT_ADDRESS
  ) {
    throw new Error('Provide your admin private key');
  }

  const signer = new ethers.Wallet(
    process.env.ADMIN_PRIVATE_KEY as string
  ).connect(ethers.provider);

  const backupRecovery = await ethers.getContractAt(
    'BackupRecoveryFacet',
    process.env.BACKUP_RECOVERY_CONTRACT_ADDRESS as string,
    signer
  );

  const addresses = await inquirer.prompt([
    {
      type: 'input',
      name: 'backupPartyMembers',
      message: 'Enter backup party addresses (seperated by comma)',
      default: 'none',
      validate: (input) => {
        let addresses = input.split(',');
        if (addresses.length < 2) {
          return false;
        } else {
          return true;
        }
      },
    },
  ]);
  let bpm = addresses.backupPartyMembers.split(',');
  const tx = await backupRecovery.registerNewBackupParty(bpm);
  await tx.wait();
  console.log('backup party members updated');
  console.log(await backupRecovery.getNextBackupPartyMembers());
}

run();
