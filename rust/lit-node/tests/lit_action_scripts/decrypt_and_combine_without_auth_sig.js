(async () => {

  const resp = await Lit.Actions.decryptAndCombine({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig: null,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();

