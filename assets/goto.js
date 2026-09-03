(() => {
  const dialog = document.getElementById("okmate-goto");
  const chipsEl = document.getElementById("okmate-goto-chips");
  const input = document.getElementById("okmate-goto-input");
  const list = document.getElementById("okmate-goto-list");
  if (!dialog || !chipsEl || !input || !list) {
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
  let chips = [];
  let mode = "pages";
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

  function catalogRoots() {
    const roots = [];
    const seen = new Set();
    for (const page of pages) {
      const root = page.root || "";
      if (!root || seen.has(root)) {
        continue;
      }
      seen.add(root);
      roots.push(root);
    }
    roots.sort();
    return roots;
  }

  function exactRoot(prefix, roots) {
    const needle = prefix.toLowerCase();
    return roots.find((root) => root.toLowerCase() === needle) || null;
  }

  function matchingRoots(prefix, roots) {
    const needle = String(prefix || "").toLowerCase();
    return roots.filter((root) => root.toLowerCase().startsWith(needle));
  }

  function commonPrefix(names) {
    if (!names.length) {
      return "";
    }
    let end = names[0].length;
    for (let i = 1; i < names.length; i += 1) {
      let shared = 0;
      const left = Array.from(names[0]);
      const right = Array.from(names[i]);
      while (
        shared < left.length &&
        shared < right.length &&
        left[shared].toLowerCase() === right[shared].toLowerCase()
      ) {
        shared += 1;
      }
      end = Math.min(end, shared);
    }
    return names[0].slice(0, end);
  }

  function completeRoot(prefix, roots) {
    const matches = matchingRoots(prefix, roots);
    if (matches.length === 1) {
      return matches[0];
    }
    if (!matches.length) {
      return null;
    }
    const common = commonPrefix(matches);
    return common.length > prefix.length ? common : null;
  }

  function parseQuery(query, roots) {
    const raw = String(query || "").trim().split(/\s+/).filter(Boolean);
    const trailingWs = /\s$/.test(query);
    let completing = null;
    const last = raw[raw.length - 1];
    if (last && last.startsWith("@")) {
      const prefix = last.slice(1);
      if (!prefix || (!trailingWs && !exactRoot(prefix, roots))) {
        completing = prefix;
      }
    }
    const selected = [...chips];
    const textParts = [];
    let unmatchedRoot = false;
    raw.forEach((token, index) => {
      const isLast = index + 1 === raw.length;
      if (token.startsWith("@")) {
        if (completing !== null && isLast) {
          return;
        }
        const match = exactRoot(token.slice(1), roots);
        if (match) {
          if (!selected.includes(match)) {
            selected.push(match);
          }
        } else {
          unmatchedRoot = true;
        }
      } else {
        textParts.push(token.toLowerCase());
      }
    });
    return {
      roots: selected,
      text: textParts.join(" "),
      completing,
      unmatchedRoot,
    };
  }

  function pullExactRoots(query, roots) {
    const parts = String(query || "").split(/(\s+)/);
    const kept = [];
    const added = [];
    parts.forEach((part, index) => {
      if (/^\s+$/.test(part)) {
        if (kept.length) {
          kept.push(part);
        }
        return;
      }
      if (!part.startsWith("@") || part.length === 1) {
        kept.push(part);
        return;
      }
      const isLast = parts.slice(index + 1).every((next) => /^\s*$/.test(next));
      const match = exactRoot(part.slice(1), roots);
      if (match && !isLast) {
        if (!chips.includes(match) && !added.includes(match)) {
          added.push(match);
        }
        return;
      }
      kept.push(part);
    });
    return { text: kept.join("").replace(/^\s+/, ""), added };
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

  function replaceAtToken(value) {
    const trimmed = value.replace(/\s+$/, "");
    const start = Math.max(trimmed.lastIndexOf(" ") + 1, 0);
    if (!trimmed.slice(start).startsWith("@")) {
      return value;
    }
    return `${value.slice(0, start)}${value.slice(trimmed.length)}`;
  }

  function commitRoot(root) {
    if (!root || chips.includes(root)) {
      input.value = replaceAtToken(input.value);
      render();
      return;
    }
    chips.push(root);
    input.value = replaceAtToken(input.value).replace(/^\s+/, "");
    active = 0;
    render();
  }

  function removeChip(root) {
    chips = chips.filter((item) => item !== root);
    active = 0;
    render();
    input.focus();
  }

  function renderChips() {
    chipsEl.replaceChildren(
      ...chips.map((root) => {
        const chip = document.createElement("span");
        chip.className = "okmate-badge okmate-root okmate-goto-chip";
        chip.textContent = `@${root}`;
        chip.title = "Remove bundle filter";
        chip.addEventListener("mousedown", (event) => {
          event.preventDefault();
          removeChip(root);
        });
        return chip;
      }),
    );
  }

  function render() {
    const roots = catalogRoots();
    const pulled = pullExactRoots(input.value, roots);
    if (pulled.added.length) {
      chips.push(...pulled.added);
      input.value = pulled.text;
    }
    renderChips();
    const parsed = parseQuery(input.value, roots);
    if (parsed.completing !== null) {
      mode = "bundles";
      hits = matchingRoots(parsed.completing, roots).map((root) => ({ root }));
    } else if (parsed.unmatchedRoot) {
      mode = "pages";
      hits = [];
    } else {
      mode = "pages";
      const catalog = parsed.roots.length
        ? pages.filter((page) => parsed.roots.includes(page.root))
        : pages;
      hits = rankPages(catalog, parsed.text);
    }
    if (active >= hits.length) {
      active = 0;
    }
    list.replaceChildren(
      ...hits.map((item, index) => {
        const row = document.createElement("li");
        row.className = index === active ? "is-active" : "";
        if (mode === "bundles") {
          row.classList.add("okmate-goto-bundle");
          const chip = document.createElement("span");
          chip.className = "okmate-badge okmate-root";
          chip.textContent = `@${item.root}`;
          row.appendChild(chip);
          row.addEventListener("mousedown", (event) => {
            event.preventDefault();
            commitRoot(item.root);
          });
          return row;
        }
        row.append(item.title || "");
        const secondary = document.createElement("span");
        secondary.textContent = secondaryText(item);
        row.appendChild(secondary);
        row.addEventListener("mousedown", (event) => {
          event.preventDefault();
          go(item.route);
        });
        return row;
      }),
    );
  }

  function go(route) {
    dialog.close();
    window.location.assign(route);
  }

  function acceptTab(event) {
    const roots = catalogRoots();
    const parsed = parseQuery(input.value, roots);
    if (parsed.completing === null) {
      const last = input.value.trim().split(/\s+/).pop() || "";
      const exact = last.startsWith("@") ? exactRoot(last.slice(1), roots) : null;
      if (exact) {
        event.preventDefault();
        commitRoot(exact);
      }
      return;
    }
    event.preventDefault();
    const completed = completeRoot(parsed.completing, roots);
    if (!completed) {
      return;
    }
    if (matchingRoots(completed, roots).length === 1) {
      commitRoot(completed);
      return;
    }
    const start = input.value.search(/@\S*$/);
    if (start >= 0) {
      input.value = `${input.value.slice(0, start)}@${completed}`;
    }
    render();
  }

  async function openPalette() {
    try {
      const response = await fetch("/pages.json", { cache: "no-store" });
      if (response.ok) {
        const next = await response.json();
        if (Array.isArray(next)) {
          pages = next;
        }
      }
    } catch (_error) {}
    chips = [];
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
    } else if (event.key === "Tab") {
      acceptTab(event);
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      active = hits.length ? (active + 1) % hits.length : 0;
      render();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      active = hits.length ? (active - 1 + hits.length) % hits.length : 0;
      render();
    } else if (event.key === "Backspace" && !input.value && chips.length) {
      event.preventDefault();
      chips.pop();
      active = 0;
      render();
    } else if (event.key === "Enter" && hits[active]) {
      event.preventDefault();
      if (mode === "bundles") {
        commitRoot(hits[active].root);
      } else {
        go(hits[active].route);
      }
    }
  });

  input.addEventListener("input", () => {
    active = 0;
    render();
  });
})();
