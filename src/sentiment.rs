use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mood {
    Positive, // Turquoise
    Negative, // Red
    Neutral,  // Green
    Refactor, // Yellow
    Weekend,  // Purple
}

impl Mood {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mood::Positive => "positive",
            Mood::Negative => "negative",
            Mood::Neutral => "neutral",
            Mood::Refactor => "refactor",
            Mood::Weekend => "weekend",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "positive" => Some(Mood::Positive),
            "negative" => Some(Mood::Negative),
            "neutral" => Some(Mood::Neutral),
            "refactor" => Some(Mood::Refactor),
            "weekend" => Some(Mood::Weekend),
            _ => None,
        }
    }

    pub fn to_color_code(&self) -> &'static str {
        match self {
            Mood::Positive => "#00f2fe", // Turquoise
            Mood::Negative => "#ff0844", // Red
            Mood::Refactor => "#f6d365", // Yellow
            Mood::Weekend => "#7f00ff",  // Purple
            Mood::Neutral => "#00cdac",  // Green
        }
    }
}

impl fmt::Display for Mood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn analyze_commit(
    message: &str,
    insertions: usize,
    deletions: usize,
    is_weekend: bool,
) -> Mood {
    let lower_msg = message.to_lowercase();

    // 1. Check for Negative / Stress Keywords (High Priority)
    let negative_keywords = [
        "fuck", "shit", "damn", "hate", "hell", "stuck", "broken", "fail", "error",
        "wrong", "panic", "crash", "issue", "ugly", "mess", "bug", "wtf", "stupid",
        "annoying", "rage", "die", "nonsense", "garbage", "trash", "crap",
    ];
    for kw in &negative_keywords {
        if lower_msg.contains(kw) {
            return Mood::Negative;
        }
    }

    // 2. Check for Refactor / Cleanup (Yellow)
    let total_changes = insertions + deletions;
    let is_refactor_keyword = lower_msg.contains("refactor")
        || lower_msg.contains("cleanup")
        || lower_msg.contains("simplify")
        || lower_msg.contains("remove")
        || lower_msg.contains("delete");

    let is_heavy_deletions = total_changes > 10 && (deletions as f64 / total_changes as f64) >= 0.70;

    if is_refactor_keyword || is_heavy_deletions {
        return Mood::Refactor;
    }

    // 3. Check for Weekend Warrior (Purple)
    if is_weekend && total_changes > 0 {
        return Mood::Weekend;
    }

    // 4. Check for Positive (Turquoise)
    let positive_keywords = [
        "feat", "add", "improve", "perf", "docs", "love", "great", "awesome",
        "easy", "solved", "fixed", "clean", "nice", "super", "happy", "resolve",
        "success", "optimize", "cool", "proud", "perfect", "magic", "beautiful",
    ];
    for kw in &positive_keywords {
        if lower_msg.contains(kw) {
            return Mood::Positive;
        }
    }

    // 5. Default/Neutral (Green)
    Mood::Neutral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_analysis() {
        assert_eq!(analyze_commit("feat: add login page", 100, 0, false), Mood::Positive);
        assert_eq!(analyze_commit("fix: stupid crash on login", 2, 2, false), Mood::Negative);
        assert_eq!(analyze_commit("refactor auth flows", 10, 50, false), Mood::Refactor);
        assert_eq!(analyze_commit("clean up helper functions", 5, 20, false), Mood::Refactor);
        assert_eq!(analyze_commit("change some config", 5, 0, true), Mood::Weekend);
        assert_eq!(analyze_commit("chore: update readme", 1, 1, false), Mood::Neutral);
    }
}
