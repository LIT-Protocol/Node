(async () => {
  const resp = await Lit.Actions.checkConditions({
    conditions: [accessControlConditions],
    authSig,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp.toString()) });
})();
