(async () => {
  const sig1 = await Lit.Actions.signAndCombineEcdsa({
    toSign: ethers.utils.arrayify(
      ethers.utils.keccak256(ethers.utils.toUtf8Bytes('test message 1'))
    ),
    publicKey,
    sigName: 'sig1',
  });

  const sig2 = await Lit.Actions.signAndCombineEcdsa({
    toSign: ethers.utils.arrayify(
      ethers.utils.keccak256(ethers.utils.toUtf8Bytes('test message 2'))
    ),
    publicKey,
    sigName: 'sig2',
  });

  const sigs = {
    sig1: JSON.parse(sig1),
    sig2: JSON.parse(sig2),
  };

  Lit.Actions.setResponse({ response: JSON.stringify(sigs) });
})();

