run-test:
	pkill -9 anvil || true
	anvil -a 10 > anvil.log 2>&1 &
	npm run test:ci