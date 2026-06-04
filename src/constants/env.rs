// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use url::Url;

pub fn whitelisted_users() -> Vec<String> {
    std::env::var("WHITELISTED_USERS")
        .unwrap_or_else(|_| "123456789".to_string()) // Default to a dummy user ID
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}

pub fn teleoxide_webhook_url() -> Url {
    (std::env::var("TELOXIDE_WEBHOOK_URL").expect("TELOXIDE_WEB_HOOK_URL must be set"))
        .parse()
        .expect("Failed to parse TELOXIDE_WEB_HOOK_URL")
}

pub fn teleoxide_secret_token() -> String {
    std::env::var("TELOXIDE_SECRET_TOKEN").expect("TELOXIDE_SECRET_TOKEN must be set")
}
