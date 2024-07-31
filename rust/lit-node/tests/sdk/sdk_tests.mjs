import * as LitNodeJsSdk from '@lit-protocol/lit-node-client-nodejs';
import { getAuthSig } from './utils.mjs';
import assert from 'assert';

const litNodeClient = new LitNodeJsSdk.LitNodeClientNodeJs({
  litNetwork: 'custom',
  bootstrapUrls: [
    'http://127.0.0.1:7470',
    'http://127.0.0.1:7471',
    'http://127.0.0.1:7472',
  ],
  checkNodeAttestation: false, // disable node attestation check for local testing
});
await litNodeClient.connect();

const chain = 'localchain';
const accessControlConditions = [
  {
    contractAddress: '',
    standardContractType: '',
    chain,
    method: 'eth_getBalance',
    parameters: [':userAddress', 'latest'],
    returnValueTest: {
      comparator: '>=',
      value: '0',
    },
  },
];

const authSig = await getAuthSig();

const testProvisioningAndSigning = async () => {
  let jwt = await litNodeClient.getSignedToken({
    accessControlConditions,
    chain,
    authSig,
  });
  const { verified } = LitNodeJsSdk.verifyJwt({
    jwt,
    publicKey: litNodeClient.networkPubKey,
  });

  assert(jwt && verified);
};

const testEncryptingAndDecryptingString = async () => {
  const { ciphertext, dataToEncryptHash } = await LitNodeJsSdk.encryptString(
    {
      accessControlConditions,
      authSig,
      chain,
      dataToEncrypt: 'Lit is 🔥',
    },
    litNodeClient
  );
  const decryptedString = await LitNodeJsSdk.decryptToString(
    {
      accessControlConditions,
      ciphertext,
      dataToEncryptHash,
      authSig,
      chain,
    },
    litNodeClient
  );

  assert(decryptedString === 'Lit is 🔥');
};

const testEncryptingAndDecryptingZip = async () => {
  const { ciphertext, dataToEncryptHash } =
    await LitNodeJsSdk.zipAndEncryptString(
      {
        accessControlConditions,
        authSig,
        chain,
        dataToEncrypt: 'Lit is zipping 🔥',
      },
      litNodeClient
    );
  const decryptedZip = await LitNodeJsSdk.decryptToZip(
    {
      accessControlConditions,
      ciphertext,
      dataToEncryptHash,
      authSig,
      chain,
    },
    litNodeClient
  );

  const decryptedString = await decryptedZip['string.txt'].async('text');

  assert(decryptedString === 'Lit is zipping 🔥');
};

const testLitActionCode = async () => {
  const code = `
        (async () => {
          console.log('Hello Lit 🔥!');
        })();
    `;

  const response = await litNodeClient.executeJs({
    code,
    authSig,
    jsParams: {},
    authMethods: [],
  });

  assert(response.logs === 'Hello Lit 🔥!\n');
};

const testLitActionIpfs = async () => {
  const ipfsId = 'QmTLZxMgjHoZiNZyGnS4CXjSjbpZSnie3y32HoqK1ykmkW';

  const response = await litNodeClient.executeJs({
    ipfsId,
    authSig,
    jsParams: {},
  });

  const litAuth = JSON.parse(response.response)['Lit.Auth'];

  assert(litAuth.actionIpfsIds.includes(ipfsId));
};

const testSessionKey = async () => {
  const sessionKey = litNodeClient.getSessionKey();
  assert(sessionKey.publicKey && sessionKey.secretKey);
};

Promise.all([
  testProvisioningAndSigning(),
  testEncryptingAndDecryptingString(),
  testEncryptingAndDecryptingZip(),
  testLitActionCode(),
  testLitActionIpfs(),
  testSessionKey(),
])
  .then(() => {
    console.log('All SDK tests completed successfully.');
    process.exit(0); // return success
  })
  .catch((error) => {
    console.error('An error occurred:', error);
    process.exit(1); // return error
  });
