Title: GitHub - DoctorMcKay/node-steam-totp: Lightweight module to generate Steam-style TOTP auth codes.

URL Source: https://github.com/DoctorMcKay/node-steam-totp

Markdown Content:
[![Image 1: npm version](https://camo.githubusercontent.com/c345c85e9fee6c097121beb19ec38de84eecd0566ebea0c83d9099a4ebcc1f76/68747470733a2f2f696d672e736869656c64732e696f2f6e706d2f762f737465616d2d746f74702e737667)](https://npmjs.com/package/steam-totp)[![Image 2: npm downloads](https://camo.githubusercontent.com/0f2133e91cb2c002ab47a711e8c91090d275c13cad3cbd676b29e37ea10dbbaf/68747470733a2f2f696d672e736869656c64732e696f2f6e706d2f646d2f737465616d2d746f74702e737667)](https://npmjs.com/package/steam-totp)[![Image 3: license](https://camo.githubusercontent.com/f163a23d00482c1299d8062729d82c26364e3d2966eacbfd0bba3c9e224a3af8/68747470733a2f2f696d672e736869656c64732e696f2f6e706d2f6c2f737465616d2d746f74702e737667)](https://github.com/DoctorMcKay/node-steam-totp/blob/master/LICENSE)[![Image 4: paypal](https://camo.githubusercontent.com/0d6e4d8b50b5983a58205941b1a581b1305903393b7a39da574e3f60af3c7f5b/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f70617970616c2d646f6e6174652d79656c6c6f772e737667)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=N36YVAT42CZ4G&item_name=node%2dsteam%2dtotp&currency_code=USD)

This lightweight module generates Steam-style 5-digit alphanumeric two-factor authentication codes given a shared secret.

**As of v2.0.0, Node.js v6.0.0 or later is REQUIRED. This LTS Node.js release will be supported by this module for the duration of Node's LTS support.**

Usage is simple:

var SteamTotp = require('steam-totp');
var code = SteamTotp.generateAuthCode('cnOgv/KdpLoP6Nbh0GMkXkPXALQ=');

[Read more about Steam's 2FA and trade confirmations.](https://dev.doctormckay.com/topic/289-trading-and-escrow-mobile-trade-confirmations/)

## time([timeOffset])

[](https://github.com/DoctorMcKay/node-steam-totp#timetimeoffset)
*   `timeOffset` - Default 0 if omitted. This many seconds will be added to the returned value.

**v1.2.0 or later is required to use this function**

Simply returns the current local time in Unix time. This is just `Math.floor(Date.now() / 1000) + timeOffset`.

## getAuthCode(secret[, timeOffset][, callback])

[](https://github.com/DoctorMcKay/node-steam-totp#getauthcodesecret-timeoffset-callback)
*   `secret` - Your `shared_secret`, as a `Buffer`, hex string, or base64 string
*   `timeOffset` - Optional. If you know your clock's offset from the Steam servers, you can provide it here. This number of seconds will be added to the current time to produce the final time. Default 0.
*   `callback` - Optional. If you provide a callback, then the auth code will **not** be returned and it will be provided to the callback. If provided, the module will also account for time discrepancies with the Steam servers. If you use this, **do not** provide a `timeOffset`. 
    *   `err` - An `Error` object on failure, or `null` on success
    *   `code` - Your auth code, as a string
    *   `offset` - Your time offset, in seconds. You can pass this to `time` later if you need to, for example to get confirmation keys.
    *   `latency` - The time in milliseconds between when we sent our request and when we received a response from the Steam time server.

**v1.4.0 or later is required to use `callback`.**

Returns your current 5-character alphanumeric TOTP code as a string (if no callback is provided) or queries the current time from the Steam servers and returns the code in the callback (if the callback was provided).

**Note:** You should use your `shared_secret` for this function.

_Alias: generateAuthCode(secret[, timeOffset][, callback])_

## getConfirmationKey(identitySecret, time, tag)

[](https://github.com/DoctorMcKay/node-steam-totp#getconfirmationkeyidentitysecret-time-tag)
*   `identitySecret` - Your `identity_secret`, as a `Buffer`, hex string, or base64 string
*   `time` - The Unix time for which you are generating this secret. Generally should be the current time.
*   `tag` - The tag which identifies what this request (and therefore key) will be for. "conf" to load the confirmations page, "details" to load details about a trade, "allow" to confirm a trade, "cancel" to cancel it.

**v1.1.0 or later is required to use this function**

Returns a string containing your base64 confirmation key for use with the mobile confirmations web page.

**Note:** You should use your `identity_secret` for this function.

_Alias: generateConfirmationKey(identitySecret, time, tag)_

## getTimeOffset(callback)

[](https://github.com/DoctorMcKay/node-steam-totp#gettimeoffsetcallback)
*   `callback` - Called when the request completes 
    *   `error` - An `Error` object, or `null` on success
    *   `offset` - The time offset in seconds
    *   `latency` - The time in milliseconds between when we sent the request and when we received a response

**v1.2.0 or later is required to use this function**

Queries the Steam servers for their time, then subtracts our local time from it to get our offset.

The offset is how many seconds we are **behind** Steam. Therefore, **add** this number to our local time to get Steam time.

You can pass this value to `time()` or to `getAuthCode()` as-is with no math involved.

## getDeviceID(steamID)

[](https://github.com/DoctorMcKay/node-steam-totp#getdeviceidsteamid)
*   `steamID` - Your SteamID as a string or an object (such as a `SteamID` object) which has a `toString()` method that returns the SteamID as a 64-bit integer string.

**v1.3.0 or later is required to use this function**

Returns a standardized device ID in the same format as Android device IDs from Valve's official mobile app. Steam will likely soon stop allowing you to send a different device ID each time you load the page, instead requiring you to consistently use the same device ID. If you use this function's algorithm everywhere you use a confirmation device ID, then your experience should be fine.

The algorithm used is:

1.   Convert the SteamID to a string
2.   Append the value of the `STEAM_TOTP_SALT` environment variable to the SteamID, if it's set
3.   SHA-1 hash it and encode the resulting hash as a lowercase hex value
4.   Truncate the hash to 32 characters
5.   Insert dashes such that the resulting value has 5 groups of hexadecimal values containing 8, 4, 4, 4, and 12 characters, respectively
6.   Prepend "android:" to the resulting value

Note: `STEAM_TOTP_SALT` was added to the v1 branch in v1.5.0 and to the v2 branch in v2.1.0.
