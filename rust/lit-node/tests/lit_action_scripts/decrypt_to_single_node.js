(async () => {

  const resp = await Lit.Actions.decryptToSingleNode({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();

