(async () => {
  const resp = await Lit.Actions.broadcastAndCollect({
    name: 'some-name',
    value: 'some-value',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
