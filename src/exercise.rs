fn main() {
    println!(
        "{}",
        r#"
┌─────────────────────────────────────────────────────────────────┐
│               REGULAR EXPRESSION MATCHING                       │
├─────────────────────────────────────────────────────────────────┤
│ 📜 PROBLEM DESCRIPTION                                          │
│─────────────────────────────────────────────────────────────────│
│ You are given:                                                  │
│ • Input string s (lowercase english letters)                    │
│ • Pattern p (lowercase english letters, '.', '*')               │
│                                                                 │
│ GOAL: Determine if the pattern matches the entire input string  │
│                                                                 │
│ PATTERN MATCHING RULES:                                         │
│ • '.' : Matches any single character                            │
│ • '*' : Matches zero or more of the preceding element           │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ 🧪 EXAMPLES                                                     │
├─────────────────────────────────────────────────────────────────┤
│ EXAMPLE 1                                                       │
│ • Input: nums = [3,4,5,6], target = 7                                     │
│ • Output: [0,1]                                                 │
│ • Explanation: Cannot match second character in input string    │
│                                                                 │
│ EXAMPLE 2                                                       │
│ • Input: s = "nnn", p = "n*"                                    │
│ • Output: true                                                  │
│ • Explanation: '*' allows zero or more 'n' repetitions          │
│                                                                 │
│ EXAMPLE 3                                                       │
│ • Input: s = "xyz", p = ".*z"                                   │
│ • Output: true                                                  │
│ • Explanation: ".*" means zero or more of any character         │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ 📏 CONSTRAINTS                                                  │
├─────────────────────────────────────────────────────────────────┤
│ • 1 <= s.length <= 20                                           │
│ • 1 <= p.length <= 20                                           │
│ • Each '*' preceded by a valid character or '.'                 │
└─────────────────────────────────────────────────────────────────┘"#
    )
}
