(async () => {
  const resp = await Lit.Actions.getRpcUrl({
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
