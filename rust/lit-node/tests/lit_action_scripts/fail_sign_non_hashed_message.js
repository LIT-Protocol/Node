const go = async () => {
  // this requests should fail because the message to sign is not 32 bytes.
  let utf8Encode = new TextEncoder();
  const toSign = utf8Encode.encode('Hello World');
  const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
};

go();
