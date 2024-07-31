(async () => {
  const accessControlConditions = [access_control_conditions];
  const ciphertext = '[ciphertext]';
  const dataToEncryptHash = '[data_to_encrypt_hash]';

  console.log('Checking access...');

  const resp = await Lit.Actions.decryptAndCombine({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig: null,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
