(async () => {
  const accessControlConditions = [access_control_conditions];
  const ciphertext = '[ciphertext]';
  const dataToEncryptHash = '[data_to_encrypt_hash]';
  const authSig = [auth_sig];

  const resp = await Lit.Actions.decryptToSingleNode({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
