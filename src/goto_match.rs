const LIMIT: usize = 12;

const TIER_EXACT: u32 = 100;
const TIER_PREFIX: u32 = 80;
const TIER_ACRONYM: u32 = 70;
const TIER_SUBSTRING: u32 = 50;
const TIER_SUBSEQUENCE: u32 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoPage {
    pub title: String,
    pub route: String,
    pub path: String,
    pub description: String,
    pub collection: String,
    pub root: String,
}

#[derive(Clone, Copy)]
struct Field {
    weight: u32,
    is_stem: bool,
}

pub fn rank_pages<'a>(pages: &'a [GotoPage], query: &str) -> Vec<&'a GotoPage> {
    let tokens: Vec<String> = query
        .split_whitespace()
        .map(|token| token.to_lowercase())
        .filter(|token| !token.is_empty())
        .collect();
    if tokens.is_empty() {
        return pages.iter().take(LIMIT).collect();
    }

    let mut scored: Vec<(Score, usize)> = pages
        .iter()
        .enumerate()
        .filter_map(|(index, page)| score_page(page, &tokens).map(|score| (score, index)))
        .collect();
    scored.sort_by(|left, right| {
        right
            .0
            .total
            .cmp(&left.0.total)
            .then(left.0.stem_match_index.cmp(&right.0.stem_match_index))
            .then(left.0.stem_len.cmp(&right.0.stem_len))
            .then(pages[left.1].route.cmp(&pages[right.1].route))
            .then(pages[left.1].path.cmp(&pages[right.1].path))
            .then(pages[left.1].root.cmp(&pages[right.1].root))
            .then(left.1.cmp(&right.1))
    });
    scored
        .into_iter()
        .take(LIMIT)
        .map(|(_, index)| &pages[index])
        .collect()
}

struct Score {
    total: u64,
    stem_match_index: u32,
    stem_len: u32,
}

fn score_page(page: &GotoPage, tokens: &[String]) -> Option<Score> {
    let stem = filename_stem(&page.path);
    let fields = [
        (
            words(&stem),
            Field {
                weight: 8,
                is_stem: true,
            },
        ),
        (
            words(&page.title),
            Field {
                weight: 5,
                is_stem: false,
            },
        ),
        (
            words(&page.path),
            Field {
                weight: 3,
                is_stem: false,
            },
        ),
        (
            words(&page.route),
            Field {
                weight: 2,
                is_stem: false,
            },
        ),
        (
            words(&page.collection),
            Field {
                weight: 2,
                is_stem: false,
            },
        ),
        (
            words(&page.description),
            Field {
                weight: 1,
                is_stem: false,
            },
        ),
        (
            words(&page.root),
            Field {
                weight: 1,
                is_stem: false,
            },
        ),
    ];
    let mut total = 0_u64;
    let mut stem_match_index = u32::MAX;
    for token in tokens {
        let mut best = 0_u64;
        let mut best_stem_index = u32::MAX;
        for (field_words, field) in &fields {
            if let Some((tier, word_index)) = match_words(field_words, token) {
                let points = u64::from(tier) * u64::from(field.weight);
                if points > best {
                    best = points;
                }
                if field.is_stem {
                    best_stem_index = best_stem_index.min(word_index as u32);
                }
            }
        }
        if best == 0 {
            return None;
        }
        total += best;
        stem_match_index = stem_match_index.min(best_stem_index);
    }
    Some(Score {
        total,
        stem_match_index,
        stem_len: stem.len() as u32,
    })
}

fn filename_stem(path: &str) -> String {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.strip_suffix(".md").unwrap_or(file).to_string()
}

fn words(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut prev: Option<char> = None;
    for (index, &ch) in chars.iter().enumerate() {
        if !ch.is_ascii_alphanumeric() {
            flush_word(&mut cur, &mut out);
            prev = None;
            continue;
        }
        if let Some(previous) = prev {
            let next = chars.get(index + 1).copied();
            let camel = previous.is_ascii_lowercase() && ch.is_ascii_uppercase();
            let acronym = previous.is_ascii_uppercase()
                && ch.is_ascii_uppercase()
                && next.is_some_and(|next| next.is_ascii_lowercase());
            if camel || acronym {
                flush_word(&mut cur, &mut out);
            }
        }
        cur.extend(ch.to_lowercase());
        prev = Some(ch);
    }
    flush_word(&mut cur, &mut out);
    out
}

fn flush_word(cur: &mut String, out: &mut Vec<String>) {
    if !cur.is_empty() {
        out.push(std::mem::take(cur));
    }
}

fn match_words(words: &[String], token: &str) -> Option<(u32, usize)> {
    if token.is_empty() || words.is_empty() {
        return None;
    }
    let mut best: Option<(u32, usize)> = None;
    for (index, word) in words.iter().enumerate() {
        let tier = if word == token {
            TIER_EXACT
        } else if word.starts_with(token) {
            TIER_PREFIX
        } else if word.contains(token) {
            TIER_SUBSTRING
        } else if is_subsequence(word, token) {
            TIER_SUBSEQUENCE
        } else {
            continue;
        };
        consider(&mut best, tier, index);
    }
    if let Some(index) = acronym_index(words, token) {
        consider(&mut best, TIER_ACRONYM, index);
    }
    best
}

fn consider(best: &mut Option<(u32, usize)>, tier: u32, index: usize) {
    match *best {
        Some((best_tier, best_index))
            if best_tier > tier || (best_tier == tier && best_index <= index) => {}
        _ => *best = Some((tier, index)),
    }
}

fn acronym_index(words: &[String], token: &str) -> Option<usize> {
    let initials: String = words
        .iter()
        .filter_map(|word| word.chars().next())
        .collect();
    initials.find(token)
}

fn is_subsequence(word: &str, token: &str) -> bool {
    let mut chars = word.chars();
    token.chars().all(|needle| chars.any(|hay| hay == needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(title: &str, route: &str, path: &str) -> GotoPage {
        GotoPage {
            title: title.into(),
            route: route.into(),
            path: path.into(),
            description: String::new(),
            collection: String::new(),
            root: String::new(),
        }
    }

    fn catalog() -> Vec<GotoPage> {
        vec![
            page("Dashboard", "/", "index.md"),
            page("Review queue", "/review/", "review"),
            page("Log", "/log/", "log"),
            page("Settings", "/settings/", "settings"),
            page(
                "Knowledge-bundle modelling in OKMate versus okf-gem",
                "/research/okmate/bundle-modelling/",
                "research/okmate/bundle-modelling.md",
            ),
            page(
                "Extended multi-bundle workspace",
                "/research/okmate/extended-multi-bundle/",
                "research/okmate/extended-multi-bundle.md",
            ),
            page(
                "OKF module notes",
                "/research/okf/module/",
                "research/okf/module.md",
            ),
            page("Research", "/research/", "research/index.md"),
        ]
    }

    fn paths<'a>(pages: &'a [&'a GotoPage]) -> Vec<&'a str> {
        pages.iter().map(|page| page.path.as_str()).collect()
    }

    #[test]
    fn empty_query_keeps_catalog_order() {
        let pages = catalog();
        let ranked = rank_pages(&pages, "   ");
        assert_eq!(
            paths(&ranked),
            paths(&pages.iter().take(12).collect::<Vec<_>>())
        );
    }

    #[test]
    fn bund_mod_ranks_bundle_modelling_first() {
        let pages = catalog();
        for query in ["bund mod", "bundle modelling"] {
            let ranked = rank_pages(&pages, query);
            assert_eq!(
                ranked.first().map(|page| page.path.as_str()),
                Some("research/okmate/bundle-modelling.md"),
                "{query}"
            );
        }
    }

    #[test]
    fn path_tokens_find_bundle_modelling() {
        let pages = catalog();
        let ranked = rank_pages(&pages, "res ok bund");
        assert_eq!(
            ranked.first().map(|page| page.path.as_str()),
            Some("research/okmate/bundle-modelling.md")
        );
    }

    #[test]
    fn acronym_finds_bundle_modelling() {
        let pages = catalog();
        let ranked = rank_pages(&pages, "bm");
        assert_eq!(
            ranked.first().map(|page| page.path.as_str()),
            Some("research/okmate/bundle-modelling.md")
        );
    }

    #[test]
    fn review_and_settings_still_resolve() {
        let pages = catalog();
        assert_eq!(
            rank_pages(&pages, "review")
                .first()
                .map(|page| page.path.as_str()),
            Some("review")
        );
        assert_eq!(
            rank_pages(&pages, "sett")
                .first()
                .map(|page| page.path.as_str()),
            Some("settings")
        );
    }

    #[test]
    fn unmatched_token_yields_no_hits() {
        let pages = catalog();
        assert!(rank_pages(&pages, "xyzzy").is_empty());
        assert!(rank_pages(&pages, "bund xyzzy").is_empty());
    }

    #[test]
    fn camel_case_splits_okmate() {
        assert_eq!(words("OKMate"), ["ok", "mate"]);
        assert_eq!(words("bundle-modelling.md"), ["bundle", "modelling", "md"]);
    }
}
