Title: User Authentication and Ownership (Steamworks Documentation)

URL Source: https://partner.steamgames.com/doc/features/auth

Markdown Content:
## [](https://partner.steamgames.com/doc/features/auth)Overview

Steamworks exposes multiple methods for authenticating a Steam user's identity and verifying ownership of an application. The following document describes each of these authentication methods used in the following scenarios:

*    Between a game client and other clients (P2P) or game servers using [Session Tickets](https://partner.steamgames.com/doc/features/auth#client_to_client)

*    Between a game client and a backend server using:

    *   [Session Tickets and the Steamworks Web API](https://partner.steamgames.com/doc/features/auth#client_to_backend_webapi)

    *   [Encrypted Application Ticket Library](https://partner.steamgames.com/doc/features/auth#encryptedapptickets)

*    When a user is in a web browser using [OpenID and Steamworks Web API](https://partner.steamgames.com/doc/features/auth#website)

## [](https://partner.steamgames.com/doc/features/auth)Identifying a user within Steam

Every Steam user can be uniquely identified by a 64-bit numeric ID, known as the user's `Steam ID`. In the Steamworks C++ APIs, a user's SteamID is contained within a [CSteamID](https://partner.steamgames.com/doc/api/steam_api#CSteamID) object. You can retrieve the current user's SteamID by calling [ISteamUser::GetSteamID](https://partner.steamgames.com/doc/api/ISteamUser#GetSteamID) and then retrieve the 64-bit ID by calling `CSteamID.ConvertToUint64()` on the returned value.

The following authentication methods can be used to securely verify a user's Steam ID.

## [](https://partner.steamgames.com/doc/features/auth)APIs covered in this document

#### Session Tickets

Session Tickets are signed tickets that can be used to verify a user's identity between the user's game client and any number of other game clients (such as in a peer-to-peer multiplayer session) or to a listen/dedicated game server (using the [ISteamGameServer](https://partner.steamgames.com/doc/api/ISteamGameServer) API). These tickets can also be used to verify ownership of the current game and related downloadable content, and determine if the user has been VAC-banned (See [Anti-cheat and Game Bans](https://partner.steamgames.com/doc/features/anticheat)).

Session Tickets can also be used to verify a user's identity between a game client and a secure, backend server using the [Steamworks Web API](https://partner.steamgames.com/doc/webapi_overview). Requires that the secure server can make HTTPS requests to `partner.steam-api.com`.

#### Encrypted Application Tickets

Encrypted Application Tickets can be used to verify a user's identity between a game client and a secure, backend server. Unlike Session Tickets, verifying Encrypted Application Tickets does _not_ require that the secure server can make HTTPS requests. Instead, a C++ library and a private, symmetric key are used by the secure server to verify the ticket. The Steamworks SDK includes 32-bit and 64-bit versions of this library for Windows and Linux under the `public/steam/lib` directory.

Before using Encrypted Application Tickets, you must generate a private key for each title. You can do this by navigating to Edit Steamworks Settings for your application and selecting 'SDK Auth' from the 'Security' drop-down. This key will be associated with your title's AppID and any downloadable content for that title. In order to access this section of Steamworks, a user must have the "Manage Signing" permission for the relevant Application.

NOTE: These keys must be stored securely, and must not be distributed within your application in any way!

#### Steamworks Web API

Steam exposes an HTTP based Web API which can be used to access many Steamworks features. The API contains public methods that can be accessed from any application capable of making an HTTP request, such as game client or server. The API also contains protected methods that require authentication and are intended to be accessed from trusted back-end applications. More details on the Web API can be found [here](https://partner.steamgames.com/doc/webapi_overview).

## [](https://partner.steamgames.com/doc/features/auth)P2P or Game Servers

## [](https://partner.steamgames.com/doc/features/auth)Session Tickets

#### User Authentication

The following steps detail how to use Session Tickets to verify a user's identity between the user's game client (client A) and another client or game server (client B):

*    Client A must retrieve a session ticket by calling [ISteamUser::GetAuthSessionTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthSessionTicket).

*    Client A must send its session ticket to client B.

*    Client B must pass client A's ticket to [ISteamUser::BeginAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#BeginAuthSession), which will perform a quick validity check. If the ticket is valid, then [ISteamUser::BeginAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#BeginAuthSession) will forward the ticket to then the Steam backend to verify that the ticket has not been reused and was issued by the account owner of client A. The result of this verification will be returned in a [ISteamUser::ValidateAuthTicketResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#ValidateAuthTicketResponse_t) callback.

*    When the multiplayer session terminates:

    *    Client A must pass the handle initially returned from [ISteamUser::GetAuthSessionTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthSessionTicket) to [ISteamUser::CancelAuthTicket](https://partner.steamgames.com/doc/api/ISteamUser#CancelAuthTicket).

    *    Client B must pass the SteamID of client A to [ISteamUser::EndAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#EndAuthSession).

A few important notes about Session Tickets:

*    Session Tickets must only be used once. [ISteamUser::GetAuthSessionTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthSessionTicket) must be called for every client in the mulitplayer session who requests a ticket.

*    When used to authenticate players within a peer-to-peer multiplayer session, each game client should verify the identity of every other game client in the multiplayer session.

*    When finished with a Session Ticket, [ISteamUser::CancelAuthTicket](https://partner.steamgames.com/doc/api/ISteamUser#CancelAuthTicket) must be called for every handle returned by [ISteamUser::GetAuthSessionTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthSessionTicket).

*    When client A calls [ISteamUser::CancelAuthTicket](https://partner.steamgames.com/doc/api/ISteamUser#CancelAuthTicket), client B will receive a [ISteamUser::ValidateAuthTicketResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#ValidateAuthTicketResponse_t) callback stating that client A's ticket is no longer valid.

*    When client A leaves a game with client B, if client A's call of [ISteamUser::CancelAuthTicket](https://partner.steamgames.com/doc/api/ISteamUser#CancelAuthTicket) is processed before client B call's of [ISteamUser::EndAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#EndAuthSession), then client B may receive a [ISteamUser::ValidateAuthTicketResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#ValidateAuthTicketResponse_t) callback stating that the ticket was cancelled. Because there is mutual agreement that client A is leaving, this callback can be ignored.

*    Network conditions may prevent the Steam backend from providing a callback to the caller of [ISteamUser::BeginAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#BeginAuthSession) for an indefinite period of time. The caller of [ISteamUser::BeginAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#BeginAuthSession) ( client B ) should not assume that he knows the true identity of client A until this callback has been received, but should allow the multiplayer session to continue.

*    If the caller of [ISteamUser::BeginAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#BeginAuthSession) receives a [ISteamUser::ValidateAuthTicketResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#ValidateAuthTicketResponse_t) callback stating that the ticket for client A is invalid, the caller must refuse to continue the multiplayer session with client A. If the other peers in the game do not also refuse to play with client A, the caller should leave the multiplayer session.

*   [ISteamGameServer](https://partner.steamgames.com/doc/api/ISteamGameServer) exposes the same Session Ticket methods to perform authentication between a game client and game server.

#### Ownership Verification

When using Session Tickets, Steam will automatically verify ownership of the current AppID. If the user does not own the current AppID, then `m_eAuthSessionResponse` field of the [ISteamUser::ValidateAuthTicketResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#ValidateAuthTicketResponse_t) will be set to [k_EAuthSessionResponseNoLicenseOrExpired](https://partner.steamgames.com/doc/api/steam_api#k_EAuthSessionResponseNoLicenseOrExpired). After receiving a user's Session Ticket and passing it to [ISteamUser::BeginAuthSession](https://partner.steamgames.com/doc/api/ISteamUser#BeginAuthSession) then, [ISteamUser::UserHasLicenseForApp](https://partner.steamgames.com/doc/api/ISteamUser#UserHasLicenseForApp) can be used to determine if the user owns a specific piece of downloadable content.

## [](https://partner.steamgames.com/doc/features/auth)Backend Server

## [](https://partner.steamgames.com/doc/features/auth)Session Tickets and the Steamworks Web API

#### User Authentication

The following steps detail how to use Session Tickets to verify a user's identity between the user's game client and a secure server:

*    The client must retrieve a session ticket by calling [ISteamUser::GetAuthTicketForWebApi](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthTicketForWebApi).

*    To guarantee a valid ticket, the client must wait for the [ISteamUser::GetTicketForWebApiResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#GetTicketForWebApiResponse_t) callback.

*    The client must send its session ticket to the secure server.

*    The secure server must make an HTTPS request to `partner.steam-api.com` and call the [ISteamUserAuth/AuthenticateUserTicket](https://partner.steamgames.com/doc/webapi/ISteamUserAuth#AuthenticateUserTicket) web method, passing the user's session ticket as a hex encoded UTF-8 string. Please note that this method allows either a [Steam Web API Key](https://steamcommunity.com/dev) or a [Web API Publisher Key](https://partner.steamgames.com/doc/webapi_overview/auth#publisher-keys) that is associated with the AppID for the provided ticket to be passed in. A future update to this API may return more information to the caller when a Web API Publisher key is supplied.

*    If the user's ticket is valid, then [ISteamUserAuth/AuthenticateUserTicket](https://partner.steamgames.com/doc/webapi/ISteamUserAuth#AuthenticateUserTicket) will return the user's 64-bit SteamID.

#### Ownership Verification

Once a user's identity has been verified, a secure server can use the [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) Web API method to check if the user owns a particular AppID, or call [ISteamUser/GetPublisherAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#GetPublisherAppOwnership) to retrieve a list of all user owned AppIDs that are associated with the provided [Publisher Key](https://partner.steamgames.com/doc/webapi_overview/auth#publisher-keys).

## [](https://partner.steamgames.com/doc/features/auth)Encrypted Application Tickets

#### User Authentication

The following steps detail how to use Encrypted Application Tickets to verify a user's identity between the user's game client and a secure server:

*    The client must call [ISteamUser::RequestEncryptedAppTicket](https://partner.steamgames.com/doc/api/ISteamUser#RequestEncryptedAppTicket) and wait for the [ISteamUser::EncryptedAppTicketResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#EncryptedAppTicketResponse_t) call result.

*    The client must then call [ISteamUser::GetEncryptedAppTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetEncryptedAppTicket) to retrieve the user's encrypted ticket and send that ticket to the secured server.

*    Using the Encrypted Application Ticket library, the secure server must then:

    *    Call [SteamEncryptedAppTicket::BDecryptTicket](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BDecryptTicket) to decrypt the user's ticket

    *    Call [SteamEncryptedAppTicket::BIsTicketForApp](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BIsTicketForApp) to verify that the ticket is for the expected application

    *    Call [SteamEncryptedAppTicket::GetTicketIssueTime](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#GetTicketIssueTime) to verify that the ticket has not expired. Tickets will expire 21 days after they are issued

    *    Call [SteamEncryptedAppTicket::GetTicketSteamID](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#GetTicketSteamID) to retrieve the user's SteamID

An example implementation can be found in the [Steamworks API Example Application (SpaceWar)](https://partner.steamgames.com/doc/sdk/api/example) project in the SDK. Specifically `CSpaceWarClient::RetrieveEncryptedAppTicket` and `CSpaceWarClient::OnRequestEncryptedAppTicket`.

#### Ownership Verification

Steam will only create Encrypted Application Tickets for users who own the AppID for which the ticket was created. After decrypting an Encrypted Application Ticket, the secure server can use [SteamEncryptedAppTicket::BIsTicketForApp](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BIsTicketForApp) to verify the AppID of the ticket matches the title's AppID. The server can also use [SteamEncryptedAppTicket::BUserOwnsAppInTicket](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BUserOwnsAppInTicket) to determine if the user owns a specific piece of [Downloadable Content (DLC)](https://partner.steamgames.com/doc/store/application/dlc).

## [](https://partner.steamgames.com/doc/features/auth)Web Browser based authentication with OpenID

Steam is an [OpenID](http://openid.net/) Provider, as described in the OpenID 2.0 specification. Inside a web browser, a third-party website can use OpenID to obtain a user's SteamID which can be used as the login credentials for the 3rd party website, or linked to an existing account on that website.

When using OpenID, the user begins in a web browser at the third-party website. When the user wishes to login/link their account to that website, using OpenID, the site directs the user to a login form on the Steam Community website. Once the user has entered their Steam login credentials, the user's web browser is automatically redirected back to the 3rd party website with some additional OpenID specific data appended to the return URL. The site's OpenID library can then use this data to verify and obtain the user's SteamID.

Steam provides the following images which may be used by 3rd party sites when linking to the Steam sign in page:

![Image 1: sits_large_border.png](https://steamcdn-a.akamaihd.net/steamcommunity/public/images/steamworks_docs/english/sits_large_border.png)

![Image 2: sits_large_noborder.png](https://steamcdn-a.akamaihd.net/steamcommunity/public/images/steamworks_docs/english/sits_large_noborder.png)

![Image 3: sits_small.png](https://steamcdn-a.akamaihd.net/steamcommunity/public/images/steamworks_docs/english/sits_small.png)

#### User Authentication

Steam's OpenID 2.0 implementation can be used to link a users Steam account to their account on the third-party website.

A list of open source OpenID libraries can be found at the [OpenID website](http://openid.net/developers/libraries/). To use OpenID to verify a user's identity:

*    Configure your OpenID library to use the following URL as Steam's OP Endpoint URL: `https://steamcommunity.com/openid/`

*    After a user has been authenticated, the user's Claimed ID will contain the user's SteamID. The Steam Claimed ID format is: `http://steamcommunity.com/openid/id/<steamid>`.

#### Ownership Verification

Once a user's identity has been verified, a secure server can use the [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) Web API method to check if the user owns a particular AppID, or call [ISteamUser/GetPublisherAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#GetPublisherAppOwnership) to retrieve a list of all user owned AppIDs that are associated with the provided [Web API Publisher Key](https://partner.steamgames.com/doc/webapi_overview/auth#publisher-keys).

## [](https://partner.steamgames.com/doc/features/auth)Examples

## [](https://partner.steamgames.com/doc/features/auth)Linking third-party accounts to Steam accounts

Third-party accounts can be linked to Steam accounts by associating a user's SteamID with the 3rd party account.

A user's SteamID can be securely retrieved either in-game or through a web browser and once the initial association has occurred, you can safely allow access to the 3rd party account by merely verifying a user's SteamID. This eliminates the need for Steam users to do any sort of secondary login to 3rd party account systems. Additionally, if new 3rd party accounts can be automatically created and linked when a new SteamID is encountered, the Steam user will never have to be aware that a secondary authentication is taking place at all. Instead, their single Steam account can grant access to all of their games, streamlining the user experience and removing potential barriers to installing and trying new games.

## [](https://partner.steamgames.com/doc/features/auth)Linking From In-game

Session Tickets can be used to verify a user's identity between a game client and a secure, backend server using the Steamworks Web API:

*    The client must retrieve a session ticket by calling [ISteamUser::GetAuthTicketForWebApi](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthTicketForWebApi).

*    To guarantee a valid ticket, the client must wait for the [ISteamUser::GetTicketForWebApiResponse_t](https://partner.steamgames.com/doc/api/ISteamUser#GetTicketForWebApiResponse_t) callback.

*    The client must send its session ticket to the secure server.

*    The secure server must make an HTTPS request to `api.steampowered.com` and call the [ISteamUserAuth/AuthenticateUserTicket](https://partner.steamgames.com/doc/webapi/ISteamUserAuth#AuthenticateUserTicket) web method, passing the user's session ticket as a hex encoded UTF-8 string. Please note that this method requires a [Web API Publisher Key](https://partner.steamgames.com/doc/webapi_overview/auth#publisher-keys) that is associated with the AppID for the provided ticket.

*    If the user's ticket is valid, [ISteamUserAuth/AuthenticateUserTicket](https://partner.steamgames.com/doc/webapi/ISteamUserAuth#AuthenticateUserTicket) will return the user's 64-bit SteamID.

## [](https://partner.steamgames.com/doc/features/auth)Linking From a Web Browser

Steam supports the OpenID 2.0 specification so that you can allow users to securly log into their Steam accounts from your website and retrieve their SteamID. For details on how to use OpenID with Steam go to [Using OpenID](https://partner.steamgames.com/doc/features/auth#website)

## [](https://partner.steamgames.com/doc/features/auth)Ownership Verification

Once a user's identity has been verified, a secure server can use the [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) Web API method to check if the user owns a particular AppID, or call [ISteamUser/GetPublisherAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#GetPublisherAppOwnership) to retrieve a list of all user owned AppIDs that are associated with the provided [Web API Publisher Key](https://partner.steamgames.com/doc/webapi_overview/auth#publisher-keys).

## [](https://partner.steamgames.com/doc/features/auth)Migrating from Third-Party CD Keys to native Steam Ownership Checks

Steam itself has a number of ways a title can authenticate a user with, removing the need for a third-party CD key. We've compiled a list of common use cases for CD Keys and how you might implement each case natively with Steam:

#### Private Forum Access

You'll want to have users login directly with their Steam account using OpenID. OpenID will return the user's 64bit SteamID which can then be used with [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) to verify the user owns your appid. More details can be found above at [Linking 3rd party accounts to Steam accounts](https://partner.steamgames.com/doc/features/auth#account_linking).

#### Unlocking a non-Steam, DRM-free build of a game

Use OpenID and [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) ([documented above](https://partner.steamgames.com/doc/features/auth#account_linking)) to unlock the content on your own site. Alternatively, you could upload the DRM-free build as optional, free DLC.

#### The software is sold on my own web site and unlocks with a key

You'll want to have users login directly with their Steam account using OpenID. OpenID will return the user's 64bit SteamID which can then be used with [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) to verify the user owns your appid. More details can be found in [Linking third-party accounts to Steam accounts](https://partner.steamgames.com/doc/features/auth#account_linking) above.

#### Dropping in-game items for registering your third-party key

If you are using the [Steam Inventory Service](https://partner.steamgames.com/doc/features/inventory), make sure the item's itemdef is configured correctly as a promo item and call [ISteamInventory::AddPromoItem](https://partner.steamgames.com/doc/api/ISteamInventory#AddPromoItem) from the client.

If you have your own item backend, you can call [ISteamUser::GetAuthSessionTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthSessionTicket) from the game client and then use [ISteamUserAuth/AuthenticateUserTicket](https://partner.steamgames.com/doc/webapi/ISteamUserAuth#AuthenticateUserTicket) with [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) to verify ownership. [More Information...](https://partner.steamgames.com/doc/features/auth#client_to_backend_webapi)

#### The key controls which version of the software is unlocked

Each version of your game should have its own AppID. From the game client, call [ISteamUser::GetAuthSessionTicket](https://partner.steamgames.com/doc/api/ISteamUser#GetAuthSessionTicket) and then use [ISteamUserAuth/AuthenticateUserTicket](https://partner.steamgames.com/doc/webapi/ISteamUserAuth#AuthenticateUserTicket) with [ISteamUser/CheckAppOwnership](https://partner.steamgames.com/doc/webapi/ISteamUser#CheckAppOwnership) to verify ownership. [More Information...](https://partner.steamgames.com/doc/features/auth#client_to_backend_webapi)
