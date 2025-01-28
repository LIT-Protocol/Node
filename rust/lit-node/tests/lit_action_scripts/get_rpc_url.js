(async () => {
  const resp = await Lit.Actions.getRpcUrl({
    chain: 'filecoin',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();


