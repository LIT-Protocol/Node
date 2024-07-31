(async () => {
  let utf8Encode = new TextEncoder();
  const accessControlConditions = [access_control_conditions];
  const to_encrypt = utf8Encode.encode('Hello World');

  const { ciphertext, dataToEncryptHash } = await Lit.Actions.encrypt({
    accessControlConditions,
    to_encrypt,
  });
  Lit.Actions.setResponse({ response: ciphertext });
})();
