#!/bin/bash

set -e -o pipefail

rm -rf build docs
mkdir build docs

LIT_SDK=ext/js/02_litActionsSDK.js

# Build API docs
documentation build "$LIT_SDK" -f md --config documentation.yml -o docs/api_docs.md --project-name "Lit Actions SDK"
documentation build "$LIT_SDK" -f html --config documentation.yml -o docs/api_docs_html --project-name "Lit Actions SDK"

# Generate types.d.ts from JSDoc (https://www.typescriptlang.org/docs/handbook/declaration-files/dts-from-js.html)
sed '/^import /d' "$LIT_SDK" > build/sdk.js
tsc build/sdk.js --declaration --allowJs --emitDeclarationOnly

cat > build/types.d.ts <<EOF
export declare namespace Lit {
  export namespace Actions {
$(sed 's/declare //' build/sdk.d.ts)
  }
}
EOF

prettier build/types.d.ts --write --no-config --ignore-path

cp build/types.d.ts docs/types.d.ts

# copy it over so it's available on the public web
cp build/types.d.ts docs/api_docs_html/

echo "Hey you! You need to manually copy docs/types.d.ts to lit-js-sdk and also to js-sdk."
