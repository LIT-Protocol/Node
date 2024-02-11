#!/bin/bash

# currently this only generates docs for the JS Libs

documentation build ./js_libs/lit/02_litActionsSDK.js -f md --config ./js_libs/documentation.yml -o js_libs/docs/api_docs.md --project-name "Lit Actions SDK"

documentation build ./js_libs/lit/02_litActionsSDK.js -f html --config ./js_libs/documentation.yml -o js_libs/docs/api_docs_html --project-name "Lit Actions SDK"

jsdoc -t /usr/local/lib/node_modules/tsd-jsdoc/dist/ -d ./js_libs/docs/ js_libs/lit/02_litActionsSDK.js

read -r -d '' PREFIX << EOM
export declare namespace Lit {
  export namespace Actions {
EOM

read -r -d '' POSTFIX << EOM
  }
}
EOM

echo $PREFIX | cat - ./js_libs/docs/types.d.ts > /tmp/out && mv /tmp/out ./js_libs/docs/types.d.ts

echo $POSTFIX >> ./js_libs/docs/types.d.ts

prettier --write ./js_libs/docs/types.d.ts

# copy it over so it's available on the public web
cp ./js_libs/docs/types.d.ts ./js_libs/docs/api_docs_html/

echo "Hey you! You need to manually copy ./js_libs/docs/types.d.ts to lit-js-sdk and also to js-sdk."