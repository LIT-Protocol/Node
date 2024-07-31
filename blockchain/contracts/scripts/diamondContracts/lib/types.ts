export interface DiamondCutManifest {
  operations: DiamondCutOperation[];
}

export enum FacetCutAction {
  Add = 0,
  Replace = 1,
  Remove = 2,
}

export interface DiamondCutOperation {
  operationId: number;
  facetName: string;
  facetAddress: string;
  diamondAddress: string;
  action: FacetCutAction;
  functionSelectors: FunctionSelector[];
}

export interface FunctionSelector {
  signature: string;
  selector: string;
}
