(async () => {
  let utf8Encode = new TextEncoder();
  const accessControlConditions = [access_control_conditions];
  const to_encrypt = utf8Encode.encode(
    'this is a secret that was encrypted with lit'
  );

  const resp = await Lit.Actions.runOnce(
    { waitForResponse: true, name: 'encrypt' },
    async () => {
      const { ciphertext, dataToEncryptHash } = await Lit.Actions.encrypt({
        accessControlConditions,
        to_encrypt,
      });
      //   console.log('ciphertext in runOnce:', ciphertext);
      //   console.log('dataToEncryptHash in runOnce:', dataToEncryptHash);
      return JSON.stringify({ ciphertext, dataToEncryptHash });
    }
  );
  const { ciphertext, dataToEncryptHash } = JSON.parse(resp);

  //   console.log('ciphertext:', ciphertext);
  //   console.log('dataToEncryptHash:', dataToEncryptHash);

  const decrypted = await Lit.Actions.decryptAndCombine({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig: null,
    chain: 'ethereum',
  });

  Lit.Actions.setResponse({ response: decrypted });
})();
