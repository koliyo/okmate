(() => {
  const dialog = document.getElementById("okmate-goto");
  const input = document.getElementById("okmate-goto-input");
  const list = document.getElementById("okmate-goto-list");
  if (!dialog || !input || !list) {
    return;
  }

  const LIMIT = 12;
  const TIER_EXACT = 100;
  const TIER_PREFIX = 80;
  const TIER_ACRONYM = 70;
  const TIER_SUBSTRING = 50;
  const TIER_SUBSEQUENCE = 20;

  let pages = [];
  let hits = [];
  let active = 0;

  function filenameStem(path) {
    const file = String(path || "").split("/").pop() || "";
    return file.endsWith(".md") ? file.slice(0, -3) : file;
  }

  function words(text) {
    const chars = Array.from(String(text || ""));
    const out = [];
    let cur = "";
    let prev = null;
    const flush = () => {
      if (cur) {
        out.push(cur);
        cur = "";
      }
    };
    for (let i = 0; i < chars.length; i += 1) {
      const ch = chars[i];
      if (!/[a-zA-Z0-9]/.test(ch)) {
        flush();
        prev = null;
        continue;
      }
      const next = chars[i + 1];
      if (prev) {
        const camel = /[a-z]/.test(prev) && /[A-Z]/.test(ch);
        const acronym = /[A-Z]/.test(prev) && /[A-Z]/.test(ch) && next && /[a-z]/.test(next);
        if (camel || acronym) {
          flush();
        }
      }
      cur += ch.toLowerCase();
      prev = ch;
    }
    flush();
    return out;
  }

  function isSubsequence(word, token) {
    let from = 0;
    for (const needle of token) {
      const at = word.indexOf(needle, from);
      if (at < 0) {
        return false;
      }
      from = at + 1;
    }
    return true;
  }

  function acronymIndex(fieldWords, token) {
    const initials = fieldWords.map((word) => word.charAt(0)).join("");
    const at = initials.indexOf(token);
    return at < 0 ? null : at;
  }

  function matchWords(fieldWords, token) {
    if (!token || !fieldWords.length) {
      return null;
    }
    let best = null;
    const consider = (tier, index) => {
      if (!best || tier > best.tier || (tier === best.tier && index < best.index)) {
        best = { tier, index };
      }
    };
    fieldWords.forEach((word, index) => {
      if (word === token) {
        consider(TIER_EXACT, index);
      } else if (word.startsWith(token)) {
        consider(TIER_PREFIX, index);
      } else if (word.includes(token)) {
        consider(TIER_SUBSTRING, index);
      } else if (isSubsequence(word, token)) {
        consider(TIER_SUBSEQUENCE, index);
      }
    });
    const acronym = acronymIndex(fieldWords, token);
    if (acronym !== null) {
      consider(TIER_ACRONYM, acronym);
    }
    return best;
  }

  function scorePage(page, tokens) {
    const stem = filenameStem(page.path);
    const fields = [
      { words: words(stem), weight: 8, isStem: true },
      { words: words(page.title), weight: 5, isStem: false },
      { words: words(page.path), weight: 3, isStem: false },
      { words: words(page.route), weight: 2, isStem: false },
      { words: words(page.collection), weight: 2, isStem: false },
      { words: words(page.description), weight: 1, isStem: false },
      { words: words(page.root), weight: 1, isStem: false },
    ];
    let total = 0;
    let stemMatchIndex = Number.MAX_SAFE_INTEGER;
    for (const token of tokens) {
      let best = 0;
      let bestStemIndex = Number.MAX_SAFE_INTEGER;
      for (const field of fields) {
        const hit = matchWords(field.words, token);
        if (!hit) {
          continue;
        }
        const points = hit.tier * field.weight;
        if (points > best) {
          best = points;
        }
        if (field.isStem) {
          bestStemIndex = Math.min(bestStemIndex, hit.index);
        }
      }
      if (best === 0) {
        return null;
      }
      total += best;
      stemMatchIndex = Math.min(stemMatchIndex, bestStemIndex);
    }
    return { total, stemMatchIndex, stemLen: stem.length };
  }

  function rankPages(catalog, query) {
    const tokens = String(query || "")
      .trim()
      .toLowerCase()
      .split(/\s+/)
      .filter(Boolean);
    if (!tokens.length) {
      return catalog.slice(0, LIMIT);
    }
    return catalog
      .map((page, index) => ({ page, index, score: scorePage(page, tokens) }))
      .filter((row) => row.score)
      .sort((left, right) => {
        if (right.score.total !== left.score.total) {
          return right.score.total - left.score.total;
        }
        if (left.score.stemMatchIndex !== right.score.stemMatchIndex) {
          return left.score.stemMatchIndex - right.score.stemMatchIndex;
        }
        if (left.score.stemLen !== right.score.stemLen) {
          return left.score.stemLen - right.score.stemLen;
        }
        return (
          (left.page.route || "").localeCompare(right.page.route || "") ||
          (left.page.path || "").localeCompare(right.page.path || "") ||
          (left.page.root || "").localeCompare(right.page.root || "") ||
          left.index - right.index
        );
      })
      .slice(0, LIMIT)
      .map((row) => row.page);
  }

  function secondaryText(page) {
    const path = page.path || page.route || "";
    return page.root ? `${path} · ${page.root}` : path;
  }

  function render() {
    hits = rankPages(pages, input.value);
    if (active >= hits.length) {
      active = 0;
    }
    list.replaceChildren(
      ...hits.map((page, index) => {
        const item = document.createElement("li");
        item.className = index === active ? "is-active" : "";
        item.append(page.title || "");
        const secondary = document.createElement("span");
        secondary.textContent = secondaryText(page);
        item.appendChild(secondary);
        item.addEventListener("mousedown", (event) => {
          event.preventDefault();
          go(page.route);
        });
        return item;
      }),
    );
  }

  function go(route) {
    dialog.close();
    window.location.assign(route);
  }

  async function openPalette() {
    if (!pages.length) {
      const response = await fetch("/pages.json");
      if (response.ok) {
        pages = await response.json();
      }
    }
    input.value = "";
    active = 0;
    render();
    dialog.showModal();
    input.focus();
  }

  window.addEventListener("keydown", (event) => {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      openPalette();
      return;
    }
    if (!dialog.open) {
      return;
    }
    if (event.key === "Escape") {
      dialog.close();
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      active = hits.length ? (active + 1) % hits.length : 0;
      render();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      active = hits.length ? (active - 1 + hits.length) % hits.length : 0;
      render();
    } else if (event.key === "Enter" && hits[active]) {
      event.preventDefault();
      go(hits[active].route);
    }
  });

  input.addEventListener("input", () => {
    active = 0;
    render();
  });
})();
