(async () => {
  const accessControlConditions = [access_control_conditions];

  const resp = await Lit.Actions.checkConditions({
    conditions: [accessControlConditions],
    authSig: null,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
