(async () => {
  let temp = await Lit.Actions.runOnce(
    { waitForResponse: true, name: 'weather' },
    async () => {
      const url = 'https://api.weather.gov/gridpoints/TOP/31,80/forecast';
      const resp = await fetch(url).then((response) => response.json());
      const temp = resp.properties.periods[0].temperature;
      return temp;
    }
  );

  Lit.Actions.setResponse({ response: JSON.stringify(temp) });
})();

