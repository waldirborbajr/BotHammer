pub fn help() -> String {
    format!(
        r#"🤖 *BotHammer v{}*

Automatically detects:
- Pornography
- Sales / spam
- Gambling
- Child exploitation / CSAM
- Suspicious links

Commands:
/help — this message
/status — bot status
/stats — group moderation stats
/language <pt|en|es> — sets the bot's language for this group (admins only)
/reload — reloads moderation.toml without restarting the bot (admins only)
/unban <user_id> — removes a user's ban (admins only)
/blockdomain <domain> — blocks a domain instantly (admins only)
/trainspam — in reply to a message, teaches the Bayesian classifier that it's spam (admins only)
/trainham — in reply to a message, teaches the Bayesian classifier that it's legitimate (admins only)"#,
        env!("CARGO_PKG_VERSION")
    )
}

pub const STATUS: &str = "✅ BotHammer is online and protecting the group!";

pub const VIOLATION_GENERIC: &str = "🚫 Prohibited content detected and removed.";

pub fn banned(username: &str) -> String {
    format!("🚫 @{username} has been banned for violating the rules.")
}

pub fn warned(username: &str, count: i64) -> String {
    format!(
        "⚠️ @{username}, your message was removed for violating the group rules. \
         Warning {count} — repeated violations lead to being muted and then banned."
    )
}

pub fn muted(username: &str, minutes: i64) -> String {
    format!("🔇 @{username} has been muted for {minutes} minute(s) after repeated rule violations.")
}

pub fn kicked(username: &str) -> String {
    format!(
        "👢 @{username} was removed from the group for repeated violations. \
         They may rejoin, but the next violation results in a permanent ban."
    )
}

pub const LANG_SET: &str = "✅ Bot language set to English.";

pub const LANG_INVALID: &str =
    "⚠️ Invalid language. Use `/language pt`, `/language en` or `/language es`.";

pub const LANG_NO_PERMISSION: &str = "⚠️ Only administrators can change the bot's language.";

pub const STATS_TITLE: &str = "📊 *Group statistics*";
pub const STATS_TOTAL: &str = "Total violations";
pub const STATS_24H: &str = "Last 24h";
pub const STATS_BY_TYPE: &str = "By category";
pub const STATS_TOP: &str = "Top offenders";
pub const STATS_EMPTY: &str = "✅ No violations recorded in this group yet.";
pub const STATS_BAYES_LEARNED: &str = "Examples taught to the Bayesian classifier";

pub const RELOAD_SUCCESS: &str = "✅ Moderation config reloaded successfully.";
pub const RELOAD_ERROR: &str =
    "⚠️ Failed to reload moderation.toml. Previous rules are still active. Check the bot logs.";
pub const RELOAD_NO_PERMISSION: &str = "⚠️ Only administrators can reload the configuration.";

pub const UNBAN_NO_PERMISSION: &str = "⚠️ Only administrators can remove bans.";
pub const UNBAN_INVALID_ID: &str = "⚠️ Usage: `/unban <user_id>` — the ID must be numeric.";

pub fn unban_success(user_id: u64) -> String {
    format!("✅ User `{user_id}` has been unbanned.")
}

pub const UNBAN_ERROR: &str =
    "⚠️ Failed to unban the user. Check that the ID is correct and the bot has admin permission.";

#[allow(non_snake_case)]
pub fn BLOCKDOMAIN_SUCCESS(domain: &str) -> String {
    format!("✅ Domain `{domain}` blocked successfully.")
}

pub const BLOCKDOMAIN_NO_PERMISSION: &str = "⚠️ Only administrators can block domains.";
pub const BLOCKDOMAIN_INVALID: &str =
    "⚠️ Usage: `/blockdomain <domain>` (e.g. /blockdomain spam-site.com).";
pub const BLOCKDOMAIN_ERROR: &str = "⚠️ Failed to block the domain. Check the bot logs.";

pub const TRAIN_NO_PERMISSION: &str = "⚠️ Only administrators can train the spam classifier.";
pub const TRAIN_MISSING_REPLY: &str =
    "⚠️ Use `/trainspam` or `/trainham` in *reply* to a text message.";
pub const TRAIN_ERROR: &str = "⚠️ Failed to record the training example. Check the bot logs.";

pub fn train_success(is_spam: bool) -> String {
    if is_spam {
        "✅ Message recorded as *spam*. The classifier was retrained just now.".to_string()
    } else {
        "✅ Message recorded as *legitimate*. The classifier was retrained just now.".to_string()
    }
}
