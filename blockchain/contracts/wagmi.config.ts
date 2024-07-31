import { defineConfig } from '@wagmi/cli';
import { hardhat } from '@wagmi/cli/plugins';

export default defineConfig({
  out: 'abis/generated.ts',
  contracts: [],
  plugins: [
    hardhat({
      project: '.',
      exclude: ['*IERC165*'],
    }),
  ],
});
