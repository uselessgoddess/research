Title: GitHub - DoctorMcKay/node-steam-session: Node.js module for authenticating with the Steam auth server. Allows for generating refresh tokens and web auth cookies for use with steam-user and other packages.

URL Source: https://github.com/DoctorMcKay/node-steam-session

Markdown Content:
## Steam Session Manager

[](https://github.com/DoctorMcKay/node-steam-session#steam-session-manager)
[![Image 1: npm version](https://camo.githubusercontent.com/16133e4c7c53415f74e94f13eac91658a34fc6aef7fc9f00f7a7404f0ced069e/68747470733a2f2f696d672e736869656c64732e696f2f6e706d2f762f737465616d2d73657373696f6e2e737667)](https://npmjs.com/package/steam-session)[![Image 2: npm downloads](https://camo.githubusercontent.com/2ac486a35bae23900d911e39e4f01286de5ff8a3c7ebfd4f9e5eed77b8a5b1b2/68747470733a2f2f696d672e736869656c64732e696f2f6e706d2f646d2f737465616d2d73657373696f6e2e737667)](https://npmjs.com/package/steam-session)[![Image 3: license](https://camo.githubusercontent.com/960357b02826580e2a53afaed64cab22d517661f851cfd35b7940976bdf2f4fd/68747470733a2f2f696d672e736869656c64732e696f2f6e706d2f6c2f737465616d2d73657373696f6e2e737667)](https://github.com/DoctorMcKay/node-steam-session/blob/master/LICENSE)[![Image 4: sponsors](https://camo.githubusercontent.com/dd4e5e60648a73e9738b2552eff1ad7730c7f1e0d34a034f59d96b835f3835c7/68747470733a2f2f696d672e736869656c64732e696f2f6769746875622f73706f6e736f72732f446f63746f724d634b61792e737667)](https://github.com/sponsors/DoctorMcKay)

This module enables you to negotiate Steam tokens by authenticating with the Steam login server. **This is for use with your own accounts.** This is not to be used to authenticate other Steam users or to gain access to their accounts. For that use-case, please use the [Steam OpenID service](https://steamcommunity.com/dev) (you may want to consider using [steam-signin](https://www.npmjs.com/package/steam-signin)) and the many available [WebAPIs](https://steamapi.xpaw.me/).

Node.js v12.22.0 or later is required to use this module.

*   [Concepts](https://github.com/DoctorMcKay/node-steam-session#concepts)
*   [Example Code](https://github.com/DoctorMcKay/node-steam-session#example-code)
*   [Exports](https://github.com/DoctorMcKay/node-steam-session#exports)
    *   [Enums](https://github.com/DoctorMcKay/node-steam-session#enums)
        *   [EAuthSessionSecurityHistory](https://github.com/DoctorMcKay/node-steam-session#eauthsessionsecurityhistory)
        *   [EAuthSessionGuardType](https://github.com/DoctorMcKay/node-steam-session#eauthsessionguardtype)
        *   [EAuthTokenPlatformType](https://github.com/DoctorMcKay/node-steam-session#eauthtokenplatformtype)
        *   [EResult](https://github.com/DoctorMcKay/node-steam-session#eresult)
        *   [ESessionPersistence](https://github.com/DoctorMcKay/node-steam-session#esessionpersistence)

    *   [Custom Transports](https://github.com/DoctorMcKay/node-steam-session#custom-transports)

*   [LoginSession](https://github.com/DoctorMcKay/node-steam-session#loginsession)
    *   [Properties](https://github.com/DoctorMcKay/node-steam-session#properties)
        *   [steamID](https://github.com/DoctorMcKay/node-steam-session#steamid)
        *   [loginTimeout](https://github.com/DoctorMcKay/node-steam-session#logintimeout)
        *   [accountName](https://github.com/DoctorMcKay/node-steam-session#accountname)
        *   [accessToken](https://github.com/DoctorMcKay/node-steam-session#accesstoken)
        *   [refreshToken](https://github.com/DoctorMcKay/node-steam-session#refreshtoken)
        *   [steamGuardMachineToken](https://github.com/DoctorMcKay/node-steam-session#steamguardmachinetoken)

    *   [Methods](https://github.com/DoctorMcKay/node-steam-session#methods)
        *   [Constructor(platformType[, options])](https://github.com/DoctorMcKay/node-steam-session#constructorplatformtype-options)
        *   [startWithCredentials(details)](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails)
        *   [startWithQR()](https://github.com/DoctorMcKay/node-steam-session#startwithqr)
        *   [submitSteamGuardCode(authCode)](https://github.com/DoctorMcKay/node-steam-session#submitsteamguardcodeauthcode)
        *   [forcePoll()](https://github.com/DoctorMcKay/node-steam-session#forcepoll)
        *   [cancelLoginAttempt()](https://github.com/DoctorMcKay/node-steam-session#cancelloginattempt)
        *   [getWebCookies()](https://github.com/DoctorMcKay/node-steam-session#getwebcookies)
        *   [refreshAccessToken()](https://github.com/DoctorMcKay/node-steam-session#refreshaccesstoken)
        *   [renewRefreshToken()](https://github.com/DoctorMcKay/node-steam-session#renewrefreshtoken)

    *   [Events](https://github.com/DoctorMcKay/node-steam-session#events)
        *   [polling](https://github.com/DoctorMcKay/node-steam-session#polling)
        *   [timeout](https://github.com/DoctorMcKay/node-steam-session#timeout)
        *   [remoteInteraction](https://github.com/DoctorMcKay/node-steam-session#remoteinteraction)
        *   [steamGuardMachineToken](https://github.com/DoctorMcKay/node-steam-session#steamguardmachinetoken-1)
        *   [authenticated](https://github.com/DoctorMcKay/node-steam-session#authenticated)
        *   [error](https://github.com/DoctorMcKay/node-steam-session#error)

*   [LoginApprover](https://github.com/DoctorMcKay/node-steam-session#loginapprover)
    *   [Properties](https://github.com/DoctorMcKay/node-steam-session#properties-1)
        *   [steamID](https://github.com/DoctorMcKay/node-steam-session#steamid-1)
        *   [accessToken](https://github.com/DoctorMcKay/node-steam-session#accesstoken-1)
        *   [sharedSecret](https://github.com/DoctorMcKay/node-steam-session#sharedsecret)

    *   [Methods](https://github.com/DoctorMcKay/node-steam-session#methods-1)
        *   [Constructor(accessToken, sharedSecret[, options])](https://github.com/DoctorMcKay/node-steam-session#constructoraccesstoken-sharedsecret-options)
        *   [getAuthSessionInfo(qrChallengeUrl)](https://github.com/DoctorMcKay/node-steam-session#getauthsessioninfoqrchallengeurl)
        *   [approveAuthSession(details)](https://github.com/DoctorMcKay/node-steam-session#approveauthsessiondetails)

## Concepts

[](https://github.com/DoctorMcKay/node-steam-session#concepts)
Logging into Steam is a two-step process.

1.   You start a login session either using your account credentials (username and password) or by generating a QR code 
    *   Use [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails) to start a login session using your account credentials
    *   Use [`startWithQR`](https://github.com/DoctorMcKay/node-steam-session#startwithqrdetails) to start a QR login session

2.   Assuming any credentials you provided when you started the session were correct, Steam replies with a list of login guards 
    *   See [EAuthSessionGuardType](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/enums-steam/EAuthSessionGuardType.ts)
    *   If your account doesn't have Steam Guard enabled or you provided a valid code upfront, there may be 0 guards required
    *   Only one guard must be satisfied to complete the login. For example, you might be given a choice of providing a TOTP code or confirming the login in your Steam mobile app

3.   When you satisfy any guards, Steam sends back an access token and a refresh token. These can be used to: 
    *   [Log on with node-steam-user](https://github.com/DoctorMcKay/node-steam-user#logondetails)
    *   [Obtain web session cookies](https://github.com/DoctorMcKay/node-steam-session#getwebcookies)
    *   Authenticate with WebAPI methods used by the mobile app

## Example Code

[](https://github.com/DoctorMcKay/node-steam-session#example-code)
See the [examples directory on GitHub](https://github.com/DoctorMcKay/node-steam-session/tree/master/examples) for example code.

## Exports

[](https://github.com/DoctorMcKay/node-steam-session#exports)
When using CommonJS (`require()`), steam-session exports an object. When using ES6 modules (`import`), steam-session does not offer a default export and you will need to import specific things.

The majority of steam-session consumers will only care about enums, and the [`LoginSession`](https://github.com/DoctorMcKay/node-steam-session#loginsession) and potentially [`LoginApprover`](https://github.com/DoctorMcKay/node-steam-session#loginapprover) classes.

## Enums

[](https://github.com/DoctorMcKay/node-steam-session#enums)
### EAuthSessionSecurityHistory

[](https://github.com/DoctorMcKay/node-steam-session#eauthsessionsecurityhistory)

const {EAuthSessionSecurityHistory} = require('steam-session');
import {EAuthSessionSecurityHistory} from 'steam-session';

[View on GitHub](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/enums-steam/EAuthSessionSecurityHistory.ts)

### EAuthSessionGuardType

[](https://github.com/DoctorMcKay/node-steam-session#eauthsessionguardtype)

const {EAuthSessionGuardType} = require('steam-session');
import {EAuthSessionGuardType} from 'steam-session';

[View on GitHub](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/enums-steam/EAuthSessionGuardType.ts)

Contains the possible auth session guards.

### EAuthTokenPlatformType

[](https://github.com/DoctorMcKay/node-steam-session#eauthtokenplatformtype)

const {EAuthTokenPlatformType} = require('steam-session');
import {EAuthTokenPlatformType} from 'steam-session';

[View on GitHub](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/enums-steam/EAuthTokenPlatformType.ts)

Contains the different platform types that can be authenticated for. You should specify the correct platform type when you instantiate a [`LoginSession`](https://github.com/DoctorMcKay/node-steam-session#loginsession) object.

Audiences present in tokens issued for the different platform types:

*   `SteamClient` - `['web', 'client']`
*   `WebBrowser` - `['web']`
*   `MobileApp` - `['web', 'mobile']`

### EResult

[](https://github.com/DoctorMcKay/node-steam-session#eresult)

const {EResult} = require('steam-session');
import {EResult} from 'steam-session';

[View on GitHub](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/enums-steam/EResult.ts)

Contains possible result codes. This is a very large enum that used throughout Steam, so most values in this enum will not be relevant when authenticating.

### ESessionPersistence

[](https://github.com/DoctorMcKay/node-steam-session#esessionpersistence)

const {ESessionPersistence} = require('steam-session');
import {ESessionPersistence} from 'steam-session';

[View on GitHub](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/enums-steam/ESessionPersistence.ts)

Contains possible persistence levels for auth sessions.

## Custom Transports

[](https://github.com/DoctorMcKay/node-steam-session#custom-transports)
It's possible to define a custom transport to be used when interacting with the Steam login server. The default transport used to interact with the Steam login server is chosen depending on your provided [EAuthTokenPlatformType](https://github.com/DoctorMcKay/node-steam-session#eauthtokenplatformtype). For the `SteamClient` platform type, a `WebSocketCMTransport` will be used to communicate with a CM server using a WebSocket. For other platform types, a `WebApiTransport` will be used to interact with the Steam login server using api.steampowered.com. **It is very likely that you won't need to mess with this.**

Everything in this category is TypeScript interfaces, so even if you're implementing a custom transport, you don't need these unless you're using TypeScript.

const {ITransport, ApiRequest, ApiResponse} = require('steam-session');
import {ITransport, ApiRequest, ApiResponse} from 'steam-session';

[View on GitHub](https://github.com/DoctorMcKay/node-steam-session/blob/master/src/transports/ITransport.ts)

## LoginSession

[](https://github.com/DoctorMcKay/node-steam-session#loginsession)

const {LoginSession} = require('steam-session');
import {LoginSession} from 'steam-session';

The `LoginSession` class is the primary way to interact with steam-session.

## Properties

[](https://github.com/DoctorMcKay/node-steam-session#properties)
### steamID

[](https://github.com/DoctorMcKay/node-steam-session#steamid)
**Read-only.** A [`SteamID`](https://www.npmjs.com/package/steamid) instance containing the SteamID for the currently-authenticated account. Populated immediately after [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails) resolves, or immediately after [`accessToken`](https://github.com/DoctorMcKay/node-steam-session#accesstoken) or [`refreshToken`](https://github.com/DoctorMcKay/node-steam-session#refreshtoken) are set (meaning that this is always populated when [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) fires).

### loginTimeout

[](https://github.com/DoctorMcKay/node-steam-session#logintimeout)
A `number` specifying the time, in milliseconds, before a login attempt will [`timeout`](https://github.com/DoctorMcKay/node-steam-session#timeout). The timer begins after [`polling`](https://github.com/DoctorMcKay/node-steam-session#polling) begins.

If you attempt to set this property after [`polling`](https://github.com/DoctorMcKay/node-steam-session#polling) has already been emitted, an Error will be thrown since setting this property after that point has no effect.

### accountName

[](https://github.com/DoctorMcKay/node-steam-session#accountname)
**Read-only.** A `string` containing your account name. This is populated just before the [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) event is fired.

### accessToken

[](https://github.com/DoctorMcKay/node-steam-session#accesstoken)
A `string` containing your access token.

~~As of 2023-09-12, Steam does not return an access token in response to successful authentication, so this won't be set when the [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) event is fired.~~ (this behavior has been reverted)

This will be set after you call [`refreshAccessToken()`](https://github.com/DoctorMcKay/node-steam-session#refreshaccesstoken) or [`renewRefreshToken()`](https://github.com/DoctorMcKay/node-steam-session#renewrefreshtoken). Also, since [`getWebCookies()`](https://github.com/DoctorMcKay/node-steam-session#getwebcookies) calls `refreshAccessToken()` internally for EAuthTokenPlatformType SteamClient or MobileApp, this will also be set after calling `getWebCookies()` for those platform types.

You can also assign an access token to this property if you already have one, although at present that wouldn't do anything useful.

Setting this property will throw an Error if:

*   You set it to a token that isn't well-formed, or
*   You set it to a refresh token rather than an access token, or
*   You have already called [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails) and you set it to a token that doesn't belong to the same account, or
*   You have already set [`refreshToken`](https://github.com/DoctorMcKay/node-steam-session#refreshtoken) and you set this to a token that doesn't belong to the same account as the refresh token

Access tokens can't be used for much. You can use them with a few undocumented WebAPIs like [IFriendsListService/GetFriendsList](https://steamapi.xpaw.me/#IFriendsListService/GetFriendsList) by passing the access token as an access_token query string parameter. For example:

```
https://api.steampowered.com/IFriendsListService/GetFriendsList/v1/?access_token=eyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyJpc3MiOiJ...
```

As of time of writing (2023-04-24), it appears that you can also use access tokens with regular published API methods, for example:

```
https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid=440&access_token=eyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyJpc3MiOiJ...
```

### refreshToken

[](https://github.com/DoctorMcKay/node-steam-session#refreshtoken)
A `string` containing your refresh token. This is populated just before the [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) event is fired. You can also assign a refresh token to this property if you already have one.

Setting this property will throw an Error if:

*   You set it to a token that isn't well-formed, or
*   You set it to an access token rather than a refresh token, or
*   You have already called [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails) and you set it to a token that doesn't belong to the same account, or
*   You have already set [`accessToken`](https://github.com/DoctorMcKay/node-steam-session#accesstoken) and you set this to a token that doesn't belong to the same account as the access token

### steamGuardMachineToken

[](https://github.com/DoctorMcKay/node-steam-session#steamguardmachinetoken)
**Read-only.** A `string` containing your Steam Guard machine token. This is populated when you pass a `steamGuardMachineToken` to [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails), or just before the [`steamGuardMachineToken`](https://github.com/DoctorMcKay/node-steam-session#steamguardmachinetoken-1) event is emitted.

## Methods

[](https://github.com/DoctorMcKay/node-steam-session#methods)
### Constructor(platformType[, options])

[](https://github.com/DoctorMcKay/node-steam-session#constructorplatformtype-options)
*   `platformType` - A value from [`EAuthTokenPlatformType`](https://github.com/DoctorMcKay/node-steam-session#eauthtokenplatformtype). You should set this to the appropriate platform type for your desired usage.
*   `options` - An object with zero or more of these properties: 
    *   `userAgent` - Pass a user-agent string if you want to override the [default user-agent](https://github.com/DoctorMcKay/node-user-agents/blob/master/index.js). This is only effective when using EAuthTokenPlatformType.WebBrowser.
    *   `transport` - An `ITransport` instance, if you need to specify a [custom transport](https://github.com/DoctorMcKay/node-steam-session#custom-transports). If omitted, defaults to a `WebSocketCMTransport` instance for `SteamClient` platform types, and a `WebApiTransport` instance for all other platform types. In all likelihood, you don't need to use this.
    *   `localAddress` - A string containing the local IP address you want to use. For example, `11.22.33.44`
    *   `httpProxy` - A string containing a URI for an HTTP proxy. For example, `http://user:pass@1.2.3.4:80`
    *   `socksProxy` - A string containing a URI for a SOCKS proxy. For example, `socks5://user:pass@1.2.3.4:1080`
    *   `agent` - An `https.Agent` instance to use for requests. If omitted, a new `https.Agent` will be created internally.
    *   `machineId` - Only applicable when using EAuthTokenPlatformType.SteamClient. Pass a `Buffer` containing a valid Steam machine ID. Pass `true` to have steam-session internally generate a machine ID using the [same format that steam-user uses](https://github.com/DoctorMcKay/node-steam-user#machineidformat). Pass `false`, `null`, or omit this property to not send a machine ID (not sending a machine ID may cause problems in the future).
    *   `machineFriendlyName` - Only applicable when using EAuthTokenPlatformType.SteamClient. Pass a `string` containing the machine name that you want to report to Steam when logging on. If omitted, a machine name will automatically be generated in the format `DESKTOP-ABCDEFG`. Auto-generated machine IDs are always the same on the same machine (it's based on the hash of your actual machine's hostname)

You can only use one of `localAddress`, `httpProxy`, `socksProxy` or `agent` at the same time. If you try to use more than one of them, an Error will be thrown.

If you specify a custom transport, then you are responsible for handling proxy or agent usage in your transport.

Constructs a new `LoginSession` instance. Example usage:

import {LoginSession, EAuthTokenPlatformType} from 'steam-session';

let session = new LoginSession(EAuthTokenPlatformType.WebBrowser);

### startWithCredentials(details)

[](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails)
*   `details` - An object with these properties: 
    *   `accountName` - Your account's login name, as a string
    *   `password` - Your account's password, as a string
    *   `persistence` - Optional. A value from [ESessionPersistence](https://github.com/DoctorMcKay/node-steam-session#esessionpersistence). Defaults to `Persistent`.
    *   `steamGuardMachineToken` - Optional. If you have a valid Steam Guard machine token, supplying it here will allow you to bypass email code verification.
    *   `steamGuardCode` - Optional. If you have a valid Steam Guard code (either email or TOTP), supplying it here will attempt to use it during login.

Starts a new login attempt using your account credentials. Returns a Promise.

If you're logging in with `EAuthTokenPlatformType.SteamClient`, you can supply a Buffer containing the SHA-1 hash of your sentry file for `steamGuardMachineToken`. For example:

import {createHash} from 'crypto';
import {readFileSync} from 'fs';
import {LoginSession, EAuthTokenPlatformType} from 'steam-session';

let hash = createHash('sha1');
hash.update(readFileSync('ssfn1234567890'));
let buffer = hash.digest(); // buffer contains a Buffer

let session = new LoginSession(EAuthTokenPlatformType.SteamClient);
session.startWithCredentials({
	accountName: 'johndoe',
	password: 'h3ll0wor1d',
	steamGuardMachineToken: buffer
});

If you supply a `steamGuardCode` here and you're using email-based Steam Guard, Steam will send you a new Steam Guard email if you're using EAuthTokenPlatformType = SteamClient or MobileApp. You would ideally keep your LoginSession active that generated your first email, and pass the code using [`submitSteamGuardCode`](https://github.com/DoctorMcKay/node-steam-session#submitsteamguardcodeauthcode) instead of creating a new LoginSession and supplying the code to `startWithCredentials`.

On failure, the Promise will be rejected with its message being equal to the string representation of an [EResult](https://github.com/DoctorMcKay/node-steam-session#eresult) value. There will also be an `eresult` property on the Error object equal to the numeric representation of the relevant EResult value. For example:

```
Error: InvalidPassword
  eresult: 5
```

On success, the Promise will be resolved with an object containing these properties:

*   `actionRequired` - A boolean indicating whether action is required from you to continue this login attempt. If false, you should expect for [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) to be emitted shortly.
*   `validActions` - If `actionRequired` is true, this is an array of objects indicating which actions you could take to continue this login attempt. Each object has these properties: 
    *   `type` - A value from [EAuthSessionGuardType](https://github.com/DoctorMcKay/node-steam-session#eauthsessionguardtype)
    *   `detail` - An optional string containing more details about this guard option. Right now, the only known use for this is that it contains your email address' domain for `EAuthSessionGuardType.EmailCode`.

Here's a list of which guard types might be present in this method's response, and how you should proceed:

*   `EmailCode`: An email was sent to you containing a code (`detail` contains your email address' domain, e.g. `gmail.com`). You should get that code and either call [`submitSteamGuardCode`](https://github.com/DoctorMcKay/node-steam-session#submitsteamguardcodeauthcode), or create a new `LoginSession` and supply that code to the `steamGuardCode` property when calling [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails).
*   `DeviceCode`: You need to supply a TOTP code from your mobile authenticator (or by using [steam-totp](https://www.npmjs.com/package/steam-totp)). Get that code and either call [`submitSteamGuardCode`](https://github.com/DoctorMcKay/node-steam-session#submitsteamguardcodeauthcode), or create a new `LoginSession` and supply that code to the `steamGuardCode` property when calling [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails).
*   `DeviceConfirmation`: You need to approve the confirmation prompt in your Steam mobile app. If this guard type is present, [polling](https://github.com/DoctorMcKay/node-steam-session#polling) will start and [`loginTimeout`](https://github.com/DoctorMcKay/node-steam-session#logintimeout) will be in effect.
*   `EmailConfirmation`: You need to approve the confirmation email sent to you. If this guard type is present, [polling](https://github.com/DoctorMcKay/node-steam-session#polling) will start and [`loginTimeout`](https://github.com/DoctorMcKay/node-steam-session#logintimeout) will be in effect.

Note that multiple guard types might be available; for example both `DeviceCode` and `DeviceConfirmation` can be available at the same time.

When this method resolves, [`steamID`](https://github.com/DoctorMcKay/node-steam-session#steamid) will be populated.

### startWithQR()

[](https://github.com/DoctorMcKay/node-steam-session#startwithqr)
Starts a new QR login attempt. Returns a Promise.

On failure, the Promise will be rejected with its message being equal to the string representation of an [EResult](https://github.com/DoctorMcKay/node-steam-session#eresult) value. There will also be an `eresult` property on the Error object equal to the numeric representation of the relevant EResult value. Realistically, failures should never happen unless Steam is having problems or you're having network issues.

On success, the Promise will be resolved with an object containing these properties:

*   `actionRequired` - Always true.
*   `validActions` - Same as `validActions` for [`startWithCredentials`](https://github.com/DoctorMcKay/node-steam-session#startwithcredentialsdetails). `DeviceConfirmation` should always be present. `DeviceCode` has also been observed, even though at this point Steam doesn't even know what account you intend to log into.
*   `qrChallengeUrl` - A string containing the URL that should be encoded into a QR code and then scanned with the Steam mobile app.

[`steamID`](https://github.com/DoctorMcKay/node-steam-session#steamid) will not be populated when this method resolves, since at this point we don't know which account we're going to log into. It will be populated after you successfully [authenticate](https://github.com/DoctorMcKay/node-steam-session#authenticated).

Immediately after this resolves, LoginSession will start [polling](https://github.com/DoctorMcKay/node-steam-session#polling) to determine when authentication has succeeded.

### submitSteamGuardCode(authCode)

[](https://github.com/DoctorMcKay/node-steam-session#submitsteamguardcodeauthcode)
*   `authCode` - Your Steam Guard code, as a string

If a Steam Guard code is needed, you can supply it using this method. Returns a Promise.

On failure, the Promise will be rejected with its message being equal to the string representation of an [EResult](https://github.com/DoctorMcKay/node-steam-session#eresult) value. There will also be an `eresult` property on the Error object equal to the numeric representation of the relevant EResult value. For example:

```
Error: TwoFactorCodeMismatch
  eresult: 88
```

Note that an incorrect email code will fail with EResult value InvalidLoginAuthCode (65), and an incorrect TOTP code will fail with EResult value TwoFactorCodeMismatch (88).

On success, the Promise will be resolved with no value. In this case, you should expect for [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) to be emitted shortly.

### forcePoll()

[](https://github.com/DoctorMcKay/node-steam-session#forcepoll)
Forces an immediate polling attempt. This will throw an `Error` if you call it before the [`polling`](https://github.com/DoctorMcKay/node-steam-session#polling) event is emitted, after [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated) is emitted, or after you call [`cancelLoginAttempt`](https://github.com/DoctorMcKay/node-steam-session#cancelloginattempt).

### cancelLoginAttempt()

[](https://github.com/DoctorMcKay/node-steam-session#cancelloginattempt)
Cancels [polling](https://github.com/DoctorMcKay/node-steam-session#polling) for an ongoing login attempt. Once canceled, you should no longer interact with this `LoginSession` object, and you should create a new one if you want to start a new attempt.

### getWebCookies()

[](https://github.com/DoctorMcKay/node-steam-session#getwebcookies)
Once successfully [authenticated](https://github.com/DoctorMcKay/node-steam-session#authenticated), you can call this method to get cookies for use on the Steam websites. You can also manually set [`refreshToken`](https://github.com/DoctorMcKay/node-steam-session#refreshtoken) and then call this method without going through another login attempt if you already have a valid refresh token. Returns a Promise.

On failure, the Promise will be rejected. Depending on the nature of the failure, an EResult may or may not be available.

On success, the Promise will be resolved with an array of strings. Each string contains a cookie, e.g.

`steamLoginSecure=blahblahblahblah`

or

`steamLoginSecure=blahblahblahblah; Path=/; Secure; HttpOnly; SameSite=None; Domain=steamcommunity.com`

Here's an example of how you can get new web cookies when you already have a valid refresh token:

import {LoginSession, EAuthTokenPlatformType} from 'steam-session';

let session = new LoginSession(EAuthTokenPlatformType.WebBrowser);
session.refreshToken = 'eyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyJpc3MiOiJ...';
let cookies = await session.getWebCookies();

As of 2025-04-30, this method works for EAuthTokenPlatformType WebBrowser and MobileApp, but using SteamClient will fail with response `AccessDenied` unless sent over an authenticated CM session. When using a SteamClient refresh token, you should use node-steam-user's [`webLogOn()`](https://github.com/DoctorMcKay/node-steam-user?tab=readme-ov-file#weblogon) method and [`webSession`](https://github.com/DoctorMcKay/node-steam-user?tab=readme-ov-file#websession) event.

### refreshAccessToken()

[](https://github.com/DoctorMcKay/node-steam-session#refreshaccesstoken)
As long as a [`refreshToken`](https://github.com/DoctorMcKay/node-steam-session#refreshtoken) is set, you can call this method to obtain a new access token. Returns a Promise.

On failure, the Promise will be rejected. An EResult will be available under the `eresult` property of the Error object.

On success, the Promise will be resolved with no value. You can then read the access token from the LoginSession's accessToken property.

import {LoginSession, EAuthTokenPlatformType} from 'steam-session';

let session = new LoginSession(EAuthTokenPlatformType.SteamClient);
session.refreshToken = 'eyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyJpc3MiOiJ...';
await session.refreshAccessToken();

console.log(`New access token: ${session.accessToken}`);

As of 2025-04-30, this method works only for EAuthTokenPlatformType MobileApp, but using WebBrowser will fail with response `AccessDenied`, and SteamClient tokens will fail with the same response unless sent over an authenticated CM session. When using a SteamClient refresh token, you should use node-steam-user's [`webLogOn()`](https://github.com/DoctorMcKay/node-steam-user?tab=readme-ov-file#weblogon) method and [`webSession`](https://github.com/DoctorMcKay/node-steam-user?tab=readme-ov-file#websession) event to get web cookies (which is the same as an access token).

### renewRefreshToken()

[](https://github.com/DoctorMcKay/node-steam-session#renewrefreshtoken)
Does the same thing as [`refreshAccessToken()`](https://github.com/DoctorMcKay/node-steam-session#refreshaccesstoken), while also attempting to renew your refresh token.

Whether a new refresh token will actually be issued is at the discretion of the Steam backend. This method will return true if a new refresh token was issued (which can be accessed using the [`refreshToken`](https://github.com/DoctorMcKay/node-steam-session#refreshtoken) property), or false if no new refresh token was issued. Regardless of the return value, the [`accessToken`](https://github.com/DoctorMcKay/node-steam-session#accesstoken) property is always updated with a fresh access token (unless there was an error).

**Important:** If a refresh token is successfully renewed (e.g. this method returns true), the old refresh token will become invalid, even if it is not yet expired.

import {LoginSession, EAuthTokenPlatformType} from 'steam-session';

let session = new LoginSession(EAuthTokenPlatformType.SteamClient);
session.refreshToken = 'eyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyJpc3MiOiJ...';
let renewed = await session.renewRefreshToken();

console.log(`New access token: ${session.accessToken}`);
if (renewed) {
	console.log(`New refresh token: ${session.refreshToken}`);
} else {
	console.log('No new refresh token was issued');
}

As of 2025-04-30, this method works only for EAuthTokenPlatformType MobileApp, but using WebBrowser will fail with response `AccessDenied`, and SteamClient tokens will fail with the same response unless sent over an authenticated CM session. When using a SteamClient refresh token, you should use node-steam-user's [`renewRefreshTokens`](https://github.com/DoctorMcKay/node-steam-user?tab=readme-ov-file#renewrefreshtokens) option and [`refreshToken`](https://github.com/DoctorMcKay/node-steam-user?tab=readme-ov-file#refreshtoken) event to renew refresh tokens.

## Events

[](https://github.com/DoctorMcKay/node-steam-session#events)
### polling

[](https://github.com/DoctorMcKay/node-steam-session#polling)
This event is emitted once we start polling Steam to periodically check if the login attempt has succeeded or not. Polling starts when any of these conditions are met:

*   A login session is successfully started with credentials and no guard is required (e.g. Steam Guard is disabled)*
*   A login session is successfully started with credentials and you supplied a valid code to `steamGuardCode`*
*   A login session is successfully started with credentials, you're using email Steam Guard, and you supplied a valid `steamGuardMachineToken`*
*   A login session is successfully started with credentials, then you supplied a valid code to [`submitSteamGuardCode`](https://github.com/DoctorMcKay/node-steam-session#submitsteamguardcodeauthcode)*
*   A login session is successfully started, and `DeviceConfirmation` or `EmailConfirmation` are among the valid guards 
    *   This case covers [QR logins](https://github.com/DoctorMcKay/node-steam-session#startwithqrdetails), since a QR login is a device confirmation under the hood

* = in these cases, we expect to only have to poll once before login succeeds.

After this event is emitted, if your [`loginTimeout`](https://github.com/DoctorMcKay/node-steam-session#logintimeout) elapses and the login attempt has not yet succeeded, [`timeout`](https://github.com/DoctorMcKay/node-steam-session#timeout) is emitted and the login attempt is abandoned. You would then need to start a new login attempt using a fresh `LoginSession` object.

### timeout

[](https://github.com/DoctorMcKay/node-steam-session#timeout)
This event is emitted when the time specified by [`loginTimeout`](https://github.com/DoctorMcKay/node-steam-session#logintimeout) elapses after [polling](https://github.com/DoctorMcKay/node-steam-session#polling) begins, and the login attempt has not yet succeeded. When `timeout` is emitted, [`cancelLoginAttempt`](https://github.com/DoctorMcKay/node-steam-session#cancelloginattempt) is called internally.

### remoteInteraction

[](https://github.com/DoctorMcKay/node-steam-session#remoteinteraction)
This event is emitted when Steam reports a "remote interaction" via [polling](https://github.com/DoctorMcKay/node-steam-session#polling). This is observed to happen when the approval prompt is viewed in the Steam mobile app for the `DeviceConfirmation` guard. For a [QR login](https://github.com/DoctorMcKay/node-steam-session#startwithqrdetails), this would be after you scan the code, but before you tap approve or deny.

### steamGuardMachineToken

[](https://github.com/DoctorMcKay/node-steam-session#steamguardmachinetoken-1)
This event is emitted when Steam sends us a new Steam Guard machine token. Machine tokens are only relevant when logging into an account that has email-based Steam Guard enabled. Thus, this will only be emitted after successfully logging into such an account.

At this time, this event is only emitted when logging in using EAuthTokenPlatformType = SteamClient. It's not presently possible to get a machine token for the WebBrowser platform (and MobileApp platform doesn't support machine tokens at all).

When this event is emitted, the [`steamGuardMachineToken`](https://github.com/DoctorMcKay/node-steam-session#steamguardmachinetoken) property contains your new machine token.

### authenticated

[](https://github.com/DoctorMcKay/node-steam-session#authenticated)
This event is emitted when we successfully authenticate with Steam. At this point, [`accountName`](https://github.com/DoctorMcKay/node-steam-session#accountname) and [`refreshToken`](https://github.com/DoctorMcKay/node-steam-session#refreshtoken) are populated. If the [EAuthTokenPlatformType](https://github.com/DoctorMcKay/node-steam-session#eauthtokenplatformtype) passed to the [constructor](https://github.com/DoctorMcKay/node-steam-session#constructorplatformtype-transport) is appropriate, you can now safely call [`getWebCookies`](https://github.com/DoctorMcKay/node-steam-session#getwebcookies).

### error

[](https://github.com/DoctorMcKay/node-steam-session#error)
This event is emitted if we encounter an error while [polling](https://github.com/DoctorMcKay/node-steam-session#polling). The first argument to the event handler is an Error object. If this happens, the login attempt has failed and will need to be retried.

Node.js will crash if this event is emitted and not handled.

## LoginApprover

[](https://github.com/DoctorMcKay/node-steam-session#loginapprover)

const {LoginApprover} = require('steam-session');
import {LoginApprover} from 'steam-session';

This class can be used to approve a login attempt that was started with a QR code. [See the approve-qr example.](https://github.com/DoctorMcKay/node-steam-session/blob/master/examples/approve-qr.ts)

## Properties

[](https://github.com/DoctorMcKay/node-steam-session#properties-1)
### steamID

[](https://github.com/DoctorMcKay/node-steam-session#steamid-1)
**Read-only.** A [`SteamID`](https://www.npmjs.com/package/steamid) instance containing the SteamID for the account to which the provided [`accessToken`](https://github.com/DoctorMcKay/node-steam-session#accesstoken-1) belongs. Populated immediately after [`accessToken`](https://github.com/DoctorMcKay/node-steam-session#accesstoken-1) is set.

### accessToken

[](https://github.com/DoctorMcKay/node-steam-session#accesstoken-1)
A `string` containing your access token. This is automatically set by the constructor, but you can also manually assign it if you need to set a new access token.

An Error will be thrown when you set this property if you set it to a value that isn't a well-formed JWT, if you set it to a refresh token rather than an access token, or if you set it to an access token that was not generated using `EAuthTokenPlatformType.MobileApp`.

### sharedSecret

[](https://github.com/DoctorMcKay/node-steam-session#sharedsecret)
A `string` or `Buffer` containing your shared secret. This is automatically set by the constructor, but you can also manually assign it if you need to set a new shared secret.

If this is a `string`, it must be either hex- or base64-encoded.

## Methods

[](https://github.com/DoctorMcKay/node-steam-session#methods-1)
### Constructor(accessToken, sharedSecret[, transport])

[](https://github.com/DoctorMcKay/node-steam-session#constructoraccesstoken-sharedsecret-transport)
*   `accessToken` - A `string` containing a valid access token for the account you want to approve logins for. This access token (**not refresh token**) must have been created using the `MobileApp` platform type.
*   `sharedSecret` - A `string` or `Buffer` containing your account's TOTP shared secret. If this is a string, it must be hex- or base64-encoded.
*   `options` - An object with zero or more of these properties: 
    *   `transport` - An `ITransport` instance, if you need to specify a [custom transport](https://github.com/DoctorMcKay/node-steam-session#custom-transports). If omitted, defaults to a `WebApiTransport` instance. In all likelihood, you don't need to use this.
    *   `localAddress` - A string containing the local IP address you want to use. For example, `11.22.33.44`
    *   `httpProxy` - A string containing a URI for an HTTP proxy. For example, `http://user:pass@1.2.3.4:80`
    *   `socksProxy` A string containing a URI for a SOCKS proxy. For example, `socks5://user:pass@1.2.3.4:1080`
    *   `agent` - An `https.Agent` instance to use for requests. If omitted, a new `https.Agent` will be created internally.

You can only use one of `localAddress`, `httpProxy`, `socksProxy` or `agent` at the same time. If you try to use more than one of them, an Error will be thrown.

If you specify a custom transport, then you are responsible for handling proxy or agent usage in your transport.

Constructs a new `LoginApprover` instance. Example usage:

import {LoginApprover} from 'steam-session';

let approver = new LoginApprover('eyAid...', 'oTVMfZJ9uHXo3m9MwTD9IOEWQaw=');

An Error will be thrown if your `accessToken` isn't a well-formed JWT, if it's a refresh token rather than an access token, or if it's an access token that was not generated using `EAuthTokenPlatformType.MobileApp`.

### getAuthSessionInfo(qrChallengeUrl)

[](https://github.com/DoctorMcKay/node-steam-session#getauthsessioninfoqrchallengeurl)
*   `qrChallengeUrl` - A `string` containing the QR challenge URL from a [`startWithQR`](https://github.com/DoctorMcKay/node-steam-session#startwithqrdetails) call

Returns a Promise which resolves to an object with these properties:

*   `ip` - The origin IP address of the QR login attempt, as a string
*   `location` - An object 
    *   `geoloc` - A string containing geo coordinates
    *   `city` - String
    *   `state` - String

*   `platformType` - The [`EAuthTokenPlatformType`](https://github.com/DoctorMcKay/node-steam-session#eauthtokenplatformtype) provided for the QR code
*   `deviceFriendlyName` - The device name provided when the QR code was generated (likely a browser user-agent)
*   `version` - A number containing the version from the QR code, probably not useful to you
*   `loginHistory` - [`EAuthSessionSecurityHistory`](https://github.com/DoctorMcKay/node-steam-session#eauthsessionsecurityhistory)
*   `locationMismatch` - A boolean indicating whether the location you requested the auth session info from doesn't match the location where the QR code was generated
*   `highUsageLogin` - A boolean indicating "whether this login has seen high usage recently"
*   `requestedPersistence` - The [`ESessionPersistence`](https://github.com/DoctorMcKay/node-steam-session#esessionpersistence) requested for this login

### approveAuthSession(details)

[](https://github.com/DoctorMcKay/node-steam-session#approveauthsessiondetails)
*   `details` - An object with these properties: 
    *   `qrChallengeUrl` - A `string` containing the QR challenge URL from a [`startWithQR`](https://github.com/DoctorMcKay/node-steam-session#startwithqrdetails) call
    *   `approve` - `true` to approve the login or `false` to deny
    *   `persistence` - An option value from [`ESessionPersistence`](https://github.com/DoctorMcKay/node-steam-session#esessionpersistence)

Approves or denies an auth session from a QR URL. If you pass `true` for `approve`, then the next poll from the `LoginSession` will return access tokens. If you pass `false`, then the `LoginSession` will emit an [`error`](https://github.com/DoctorMcKay/node-steam-session#error) event with [EResult](https://github.com/DoctorMcKay/node-steam-session#eresult)`FileNotFound` (9).

Returns a Promise which resolves with no value. Once this Promise resolves, you could call [`forcePoll`](https://github.com/DoctorMcKay/node-steam-session#forcepoll), and the `LoginSession` should then immediately emit [`authenticated`](https://github.com/DoctorMcKay/node-steam-session#authenticated).
