const { ethers } = require('ethers');
const uuid = require('uuid');

const PKP_CONTRACT_ADDRESS = '0x6A275664C0d1789A59a8f8141A21409a1508a2f7';

const go = async () => {
  const walletJson = JSON.parse(process.env.LIT_FREE_MINT_KEY);

  const wallet = new ethers.Wallet(walletJson.privateKey);

  const freeMintId = ethers.hexlify(uuid.parse(uuid.v4()));

  // sign for real
  const toSign = ethers.solidityPackedKeccak256(
    ['address', 'uint256'],
    [PKP_CONTRACT_ADDRESS, freeMintId]
  );
  let sig = await wallet.signMessage(ethers.getBytes(toSign));
  console.log('sig', sig);

  const r = sig.slice(0, 66);
  const s = '0x' + sig.slice(66, 130);
  const v = '0x' + sig.slice(130, 132);

  console.log('r: ', r);
  console.log('s: ', s);
  console.log('v: ', v);

  const msgHash = ethers.solidityPackedKeccak256(
    ['string', 'bytes32'],
    ['\x19Ethereum Signed Message:\n32', toSign]
  );

  console.log(
    `Now Call pkpContract.freeMintNext(2, ${freeMintId}, ${msgHash}, ${v}, ${r}, ${s})`
  );
};

go();
