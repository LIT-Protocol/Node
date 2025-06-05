import { op_increment_fetch_count, op_panic } from 'ext:core/ops';

// Import modules to suppress build error:
// "Following modules were not evaluated; make sure they are imported from other code"
// This is required because we currently extend globalThis instead of using ES modules at runtime.
import * as _ethers from 'ext:lit_actions/00_ethers.js';
import * as _actions from 'ext:lit_actions/02_litActionsSDK.js';
import * as _jwt from 'ext:lit_actions/03_jsonwebtoken.js';

// this block scopes oldFetch so that nobody can ever use it after
{
  const oldFetch = globalThis.fetch;
  const fetch = async function () {
    const fetchCount = await op_increment_fetch_count();
    // extract the URL from the arguments
    let url = null;
    const possibleUrl = arguments[0];
    // is this a string or URL object?
    if (typeof possibleUrl === 'string' || possibleUrl instanceof URL) {
      url = possibleUrl;
    } else if (
      possibleUrl &&
      typeof possibleUrl === 'object' &&
      'url' in possibleUrl
    ) {
      // the first argument is a request object
      url = possibleUrl.url;
    } else {
      url = JSON.stringify(arguments);
    }
    if (!globalThis.litInternalFetchDebugLog) {
      globalThis.litInternalFetchDebugLog = [];
    }
    globalThis.litInternalFetchDebugLog.push({
      // current timestamp in MS
      timestamp: Date.now(),
      url: url.toString(),
      fetchCount,
    });
    return oldFetch.apply(null, arguments);
  };
  Object.freeze(fetch);

  globalThis.fetch = fetch;
}

// Expose Deno's built-in panic op for testing
globalThis.LitTest = { op_panic };

// expose "global" because it was available in the old deno version
// but is not available in the new one, and we don't want to break
// existing code that expects it to be available
globalThis.global = globalThis;
