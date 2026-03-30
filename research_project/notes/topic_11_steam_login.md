# Topic 11: Steam Login Strategies for Automated Multi-Account Management in VM Environments

## Sources

- Steam Partner Documentation: User Authentication and Ownership (partner.steamgames.com/doc/features/auth)
- DoctorMcKay/node-steam-session: Steam Session Manager (github.com/DoctorMcKay/node-steam-session)
- DoctorMcKay/node-steam-totp: Steam TOTP code generation (github.com/DoctorMcKay/node-steam-totp)
- steamguard-cli: Rust CLI for Steam Mobile Authenticator (crates.io/crates/steamguard-cli, github.com/dyc3/steamguard-cli)
- steam-totp Rust crate (crates.io/crates/steam-totp, github.com/saskenuba/SteamHelper-rs)
- Arch Wiki: Steam/Troubleshooting (wiki.archlinux.org)
- SteamDesktopAuthenticator (github.com/Jessecar96/SteamDesktopAuthenticator)

Note: Some sources (SteamCMD wiki, certain GitHub repos) were blocked by anti-bot measures or returned 404. Analysis below is based on successfully retrieved sources and established community knowledge.

---

## 1. Steam Login Flow (Modern Architecture)

Steam authentication is a two-step process as documented in node-steam-session:

1. **Start a login session** using account credentials (username + password) or QR code.
2. **Satisfy Steam Guard requirements** (if any). Steam replies with a list of guard types that must be satisfied. Only one guard needs to be completed.
3. **Receive tokens**: On success, Steam returns an **access token** (short-lived JWT) and a **refresh token** (long-lived JWT). These are used for all subsequent authentication.

### Guard Types (EAuthSessionGuardType)

- **EmailCode**: Steam sends a code to the account's email. Must retrieve and submit it.
- **DeviceCode**: TOTP code from the Steam Mobile Authenticator (or generated programmatically with shared_secret).
- **DeviceConfirmation**: Approve a prompt in the Steam mobile app (polling-based).
- **EmailConfirmation**: Approve via email link (polling-based).
- **None**: No guard required (Steam Guard disabled, or valid code provided upfront).

### Platform Types (EAuthTokenPlatformType)

- **SteamClient**: For the desktop Steam client. Tokens have audiences `['web', 'client']`. Communicates via WebSocket to CM servers.
- **WebBrowser**: For browser-based sessions. Tokens have audience `['web']`. Uses WebAPI transport.
- **MobileApp**: For mobile app sessions. Tokens have audiences `['web', 'mobile']`.

---

## 2. Automating Steam Login for Multiple Accounts

### 2.1 Credential-Based Login (startWithCredentials)

The primary approach for automation. Required parameters:
- `accountName` - Login username
- `password` - Account password
- `steamGuardCode` (optional) - Pre-supply a TOTP code to skip the guard step
- `steamGuardMachineToken` (optional) - Bypass email code verification if you have a valid machine token

**Key insight**: If you supply a valid `steamGuardCode` (TOTP) upfront in `startWithCredentials`, and the code is correct, the login completes immediately with no additional guard interaction needed. This is the optimal path for automation.

### 2.2 Token-Based Re-authentication

Once you have a valid **refresh token**, you can skip the full login flow entirely:
```
session.refreshToken = '<saved_refresh_token>';
let cookies = await session.getWebCookies();
```

This avoids needing the password or Steam Guard code for subsequent sessions. Refresh tokens are long-lived but do eventually expire.

### 2.3 QR Code Login

An alternative that does not require the account password. The server generates a QR URL, which is scanned by the Steam mobile app. Less useful for full automation, but could be used for initial setup.

---

## 3. Steam Guard Handling (TOTP)

### 3.1 How Steam TOTP Works

Steam uses a non-standard TOTP implementation that generates 5-character alphanumeric codes (not the standard 6-digit numeric codes). Key secrets:

- **shared_secret**: Base64-encoded secret used to generate 2FA login codes. This is the primary secret needed for automation.
- **identity_secret**: Used to generate confirmation keys for trade/market confirmations.
- **revocation_code**: Emergency code to remove the authenticator. Must be stored securely as a backup.

### 3.2 Generating TOTP Codes Programmatically

Using `shared_secret`, codes can be generated with:
```javascript
// node-steam-totp
var code = SteamTotp.generateAuthCode('cnOgv/KdpLoP6Nbh0GMkXkPXALQ=');
```

The algorithm:
1. Get current Unix timestamp (optionally adjusted by a time offset from Steam servers)
2. Divide by 30 (standard TOTP period)
3. HMAC-SHA1 with the shared_secret
4. Extract 5-character code from Steam's custom alphabet (2-9, A-Z excluding vowels and similar chars)

### 3.3 Time Synchronization

Steam TOTP codes are time-sensitive. For reliable automation:
- Query Steam's time server to determine clock offset: `getTimeOffset()` returns the difference in seconds between local time and Steam time.
- Apply the offset when generating codes.
- The `getAuthCode(secret, callback)` form automatically handles time sync.

### 3.4 Confirmation Keys

For trade/market confirmations, use `identity_secret`:
```javascript
var confKey = SteamTotp.getConfirmationKey(identitySecret, time, 'conf');
```
Tags: "conf" (load confirmations), "details" (trade details), "allow" (confirm), "cancel" (cancel).

### 3.5 Device ID Generation

Steam expects a consistent device ID per account. The standard algorithm:
1. SHA-1 hash of the SteamID (as string)
2. Truncate to 32 hex characters
3. Format as UUID-like: 8-4-4-4-12
4. Prepend "android:"

### 3.6 Machine Tokens (Bypassing Email Guard)

For accounts using email-based Steam Guard (not TOTP), a **steamGuardMachineToken** can bypass email verification:
- For SteamClient platform: Pass a `Buffer` containing the SHA-1 hash of the ssfn (sentry) file
- These tokens are emitted via the `steamGuardMachineToken` event after successful login
- Store and reuse them to avoid email verification on subsequent logins

---

## 4. Session Persistence and Key Files

### 4.1 Critical Files for Session State

| File | Location | Purpose |
|------|----------|---------|
| `ssfn*` (sentry files) | `~/.steam/steam/` or Steam install dir | Machine authorization tokens. SHA-1 hash serves as steamGuardMachineToken for email-based Steam Guard. |
| `config.vdf` | `~/.steam/steam/config/` | Steam client configuration including cached credentials, install paths, and account settings. |
| `loginusers.vdf` | `~/.steam/steam/config/` | List of accounts that have logged in on this machine. Contains account names, SteamIDs, persona names, and remember-password flags. |
| `maFiles/` | App-specific (e.g. `~/.config/steamguard-cli/maFiles/`) | Mobile authenticator data files containing shared_secret, identity_secret, and other 2FA data. Used by SteamDesktopAuthenticator and steamguard-cli. |
| `manifest.json` | Inside `maFiles/` | Index of all authenticator accounts. |

### 4.2 Token Lifecycle

- **Access tokens**: Short-lived JWTs. Can be refreshed using the refresh token.
- **Refresh tokens**: Long-lived JWTs. Persist across sessions. Eventually expire but last much longer than access tokens.
- **Web cookies**: `steamLoginSecure` cookie. Can be obtained from tokens via `getWebCookies()`.

### 4.3 Persistence Strategy for Automation

For each managed account, persist:
1. **refresh_token** (primary) - Allows re-authentication without password/2FA
2. **shared_secret** - For generating TOTP codes when refresh token expires
3. **identity_secret** - For confirming trades/market actions
4. **steamGuardMachineToken** - For bypassing email guard (if applicable)
5. **account_name** and **password** - As fallback for full re-authentication

---

## 5. Managing Multiple Steam Accounts on One Machine

### 5.1 SteamCMD Approach

SteamCMD is the command-line Steam client, designed for dedicated server management. Key properties:
- Supports non-interactive login: `steamcmd +login <username> <password> +quit`
- Can accept Steam Guard codes via stdin or command line
- Maintains separate session data per invocation path
- Multiple instances can run simultaneously with different install directories

### 5.2 Separate Steam Data Directories

Each account can use an isolated Steam data directory:
- Set `STEAM_HOME` or `HOME` environment variable per instance
- Or use `--steam-dir` / install path options
- Each directory maintains its own ssfn files, config.vdf, and loginusers.vdf

### 5.3 API-Based Multi-Account (Recommended for Automation)

Instead of running multiple Steam client instances, use the programmatic approach:
1. Store credentials centrally (server-side, encrypted)
2. Create separate `LoginSession` instances per account
3. Each session manages its own tokens independently
4. No need for separate file system state per account

---

## 6. VM-Based Multi-Account Architecture with Server-Managed Credentials

### 6.1 Recommended Architecture

```
[Central Credential Server]
    |
    |-- Stores per-account: username, password, shared_secret,
    |   identity_secret, refresh_token, machine_token
    |
    v
[VM Worker Nodes]
    |
    |-- Request credentials from server at startup
    |-- Perform login using startWithCredentials + TOTP code
    |-- Report back new refresh_tokens for persistence
    |-- Run game / perform actions
    |-- Report status back to server
```

### 6.2 Login Flow for Each VM

1. **VM starts** and requests account assignment from the central server.
2. **Server provides**: account_name, password, shared_secret, and optionally a cached refresh_token.
3. **VM attempts token-based login first**: If a valid refresh_token exists, use it directly (no password/2FA needed).
4. **Fallback to credential login**: Generate TOTP code from shared_secret, then call startWithCredentials with the code pre-supplied.
5. **On success**: Report the new refresh_token back to the server for caching.
6. **On failure**: Report error. Server may rotate to a different account or retry with delay.

### 6.3 Key Considerations

- **Rate limiting**: Steam rate-limits login attempts. Space out logins across VMs. Do not retry immediately on failure.
- **IP considerations**: Multiple logins from the same IP may trigger additional verification. Use proxies if needed. node-steam-session supports `httpProxy`, `socksProxy`, and `localAddress` options.
- **Machine ID**: For SteamClient platform, generate consistent machine IDs per VM (or per account). node-steam-session can auto-generate these (`machineId: true`).
- **Machine friendly name**: Can be customized per VM via `machineFriendlyName` option. Auto-generated names use format `DESKTOP-ABCDEFG` based on hostname hash.
- **Token storage security**: shared_secret and identity_secret are the keys to the account. Encrypt at rest. steamguard-cli supports encrypted maFiles with system keyring integration.

### 6.4 Handling Steam Guard in VMs

**Best case (TOTP-enabled accounts)**:
- Store `shared_secret` on the central server
- Generate TOTP code at login time
- Supply code in `startWithCredentials` for immediate, non-interactive login
- No human intervention needed

**Fallback (email-based Steam Guard)**:
- Store `steamGuardMachineToken` per account per VM
- On first login, will need email code (requires human or email API integration)
- Subsequent logins with valid machine token bypass email verification
- Machine tokens are VM-specific (tied to the machine/sentry)

**Accounts without Steam Guard**:
- Simplest to automate but least secure
- Just username + password needed
- Not recommended for accounts with valuable inventories

---

## 7. Rust Crates for Steam Authentication

### 7.1 steamguard-cli / steamguard (Rust) -- MOST MATURE

- **Crate**: `steamguard-cli` (binary) backed by `steamguard` library crate
- **Repository**: github.com/dyc3/steamguard-cli
- **Features**:
  - Generate 2FA codes
  - Respond to trade/market/any confirmations
  - Encrypted storage of 2FA secrets (with system keyring option)
  - Memory-clearing data structures to prevent secret leakage
  - QR code generation for importing/exporting secrets
  - QR code login support (e.g., Steam Deck)
  - Reads SteamDesktopAuthenticator's maFiles format
  - Uses official Steam APIs
- **Status**: Active development, AUR packages available, most feature-complete Rust option
- **Install**: `cargo install steamguard-cli`
- **Lookup paths**: `~/.config/steamguard-cli/maFiles/` (Linux), `%APPDATA%\steamguard-cli\maFiles\` (Windows)

### 7.2 steam-totp (Rust)

- **Crate**: `steam-totp` v0.2.2
- **Repository**: github.com/saskenuba/SteamHelper-rs (monorepo)
- **Features**: Utility crate to generate Steam TOTP codes
- **Size**: ~396 SLoC, lightweight
- **Status**: Last updated ~4 years ago. Functional but not actively maintained.
- **License**: MIT

### 7.3 SteamHelper-rs (Rust)

- **Repository**: github.com/saskenuba/SteamHelper-rs
- **Contains multiple crates**:
  - `steam-totp` - TOTP code generation
  - `steam-auth` - Authentication (available on crates.io)
  - Additional helpers
- **Status**: Limited recent activity

### 7.4 Recommended Rust Approach

For a VM-based automation system in Rust:
1. **Use `steamguard` library crate** (from steamguard-cli) for TOTP generation and confirmation handling. It is the most mature and actively maintained Rust option.
2. **Use `steam-totp` crate** if you only need lightweight TOTP code generation without the full steamguard infrastructure.
3. **Implement the login session management** using Steam's WebAPI directly (the authentication endpoints are HTTP-based), or port the patterns from node-steam-session to Rust using reqwest/tokio.
4. **For the CM protocol** (SteamClient platform type), you would need a protobuf-based WebSocket client to communicate with Steam CM servers. This is more complex but provides full client capabilities.

### 7.5 Key Node.js Libraries (Reference Implementation)

Even if building in Rust, these Node.js libraries serve as excellent reference implementations:

| Library | Purpose | npm |
|---------|---------|-----|
| `steam-session` | Full login session management, token negotiation | DoctorMcKay/node-steam-session |
| `steam-totp` | TOTP code + confirmation key generation | DoctorMcKay/node-steam-totp |
| `steam-user` | Full Steam client (CM connection, game launching) | DoctorMcKay/node-steam-user |
| `steamcommunity` | Steam Community web interactions | DoctorMcKay/node-steamcommunity |

---

## 8. Summary: Optimal Strategy for VM-Based Multi-Account System

1. **Enable TOTP (Steam Mobile Authenticator) on all managed accounts**. Extract and store `shared_secret` and `identity_secret` centrally. This is non-negotiable for reliable automation.

2. **Use a central credential server** that securely stores all account secrets (encrypted at rest). VMs request credentials on-demand and never persist secrets to disk.

3. **Login flow per VM**:
   - Try refresh_token first (fastest, no 2FA needed)
   - Fall back to credential + TOTP login
   - Generate TOTP code server-side and pass to VM, or have VM generate from shared_secret
   - Cache new refresh_token on central server

4. **For Rust implementation**:
   - Use `steamguard` crate for TOTP/confirmation handling
   - Implement HTTP-based login against Steam's auth endpoints (modeled after node-steam-session)
   - For full game client features, implement CM protocol via WebSocket + protobuf

5. **Session management**:
   - Refresh tokens are long-lived; prefer token reuse over fresh logins
   - Implement graceful token refresh before expiry
   - Rate-limit login attempts to avoid triggering Steam's anti-abuse systems

6. **Per-VM isolation**:
   - Each VM gets a unique machine ID and machine name
   - Use proxy support for IP diversity if needed
   - Keep Steam data directories isolated between accounts
