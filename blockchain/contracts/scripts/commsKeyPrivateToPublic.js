const nacl = require('tweetnacl');
const { fromString } = require('uint8arrays/from-string');
const { toString } = require('uint8arrays/to-string');

async function main() {
  const rl = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const secretKey = await new Promise((resolve) => {
    rl.question(
      'What is the comms key private key you wish to make into a public key? ',
      resolve
    );
  });

  const keys = nacl.box.keyPair.fromSecretKey(fromString(secretKey, 'base16'));
  console.log('Public key: ', toString(keys.publicKey, 'base16'));
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => {
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
