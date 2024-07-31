(async () => {
  const accessControlConditions = [access_control_conditions];
  const authSig = [auth_sig];

  const resp = await Lit.Actions.checkConditions({
    conditions: [accessControlConditions],
    authSig,
    chain: 'ethereum',
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
