(async () => {
  // sign "hello world" and allow all the nodes to combine the signature and return it to the action.
  let utf8Encode = new TextEncoder();
  const toSign = utf8Encode.encode('Hello World');

  const signature = await Lit.Actions.signAndCombineEcdsa({
    toSign,
    publicKey,
    sigName,
  });

  Lit.Actions.setResponse({ response: JSON.stringify(signature) });
})();
