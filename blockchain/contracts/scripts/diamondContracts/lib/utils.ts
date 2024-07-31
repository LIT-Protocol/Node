import { Contract, Fragment } from 'ethers';
import { FunctionSelector } from './types';

// get function selectors from ABI
export function getSelectors(contract: Contract): FunctionSelector[] {
  // Extract all the function fragments
  const functionFragments = contract.interface.fragments.filter(
    Fragment.isFunction
  );
  const selectors = functionFragments.map((f) => ({
    signature: f.format('minimal'),
    selector: f.selector,
  }));
  return selectors;
}
