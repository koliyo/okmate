(function () {
  if (window.__okmatePeek) {
    return;
  }

  var DWELL_MS = 450;
  var timer = 0;
  var activeLink = null;
  var card = null;
  var abort = null;

  function normalizeRoute(path) {
    var route = (path || "/").split(/[?#]/)[0];
    if (!route || route === "/") {
      return "/";
    }
    return "/" + route.replace(/^\/+|\/+$/g, "") + "/";
  }

  function footnoteLabel(hash) {
    var id = (hash || "").replace(/^#/, "");
    if (id.indexOf("fn-") === 0 && id.length > 3) {
      return id.slice(3);
    }
    if (id.indexOf("fnref-") === 0 && id.length > 6) {
      var rest = id.slice(6);
      var dash = rest.lastIndexOf("-");
      if (dash > 0) {
        var suffix = rest.slice(dash + 1);
        if (suffix && /^\d+$/.test(suffix)) {
          rest = rest.slice(0, dash);
        }
      }
      return rest || null;
    }
    return null;
  }

  function classifyHref(href, currentPath) {
    if (!href || href.indexOf("/__okmate") === 0) {
      return null;
    }
    var dest;
    try {
      dest = new URL(href, window.location.origin + (currentPath || "/"));
    } catch (err) {
      return null;
    }
    if (dest.protocol === "http:" || dest.protocol === "https:") {
      if (typeof window !== "undefined" && window.location && dest.origin !== window.location.origin) {
        return null;
      }
    }
    if (dest.protocol !== "http:" && dest.protocol !== "https:" && dest.protocol !== "") {
      if (href.indexOf("mailto:") === 0 || href.indexOf("okf:") === 0) {
        return null;
      }
    }
    var path = dest.pathname || "";
    if (path.indexOf("/assets/") === 0 || /\.(png|jpe?g|gif|svg|webp|pdf)$/i.test(path)) {
      return null;
    }
    var hash = (dest.hash || "").replace(/^#/, "");
    var route = normalizeRoute(path || currentPath || "/");
    var current = normalizeRoute(currentPath || "/");
    var sameDoc = route === current;
    var footnote = !!footnoteLabel(hash);
    return {
      path: route,
      hash: hash,
      sameDoc: sameDoc,
      footnoteRef: footnote && sameDoc,
    };
  }

  function skipLink(link) {
    if (!link || link.classList.contains("is-broken")) {
      return true;
    }
    if (link.closest("details.nav-section > summary")) {
      return true;
    }
    if (link.closest(".okmate-nav-blurb")) {
      return true;
    }
    return false;
  }

  function ensureCard() {
    if (card) {
      return card;
    }
    card = document.createElement("div");
    card.id = "okmate-peek";
    card.className = "okmate-peek";
    card.setAttribute("role", "tooltip");
    card.hidden = true;
    card.innerHTML =
      '<div class="okmate-peek-type"></div>' +
      '<div class="okmate-peek-kicker"></div>' +
      '<div class="okmate-peek-title"></div>' +
      '<p class="okmate-peek-desc"></p>' +
      '<p class="okmate-peek-excerpt"></p>' +
      '<p class="okmate-peek-hint"></p>';
    document.body.appendChild(card);
    return card;
  }

  function hide() {
    if (timer) {
      window.clearTimeout(timer);
      timer = 0;
    }
    if (abort) {
      abort.abort();
      abort = null;
    }
    activeLink = null;
    if (card) {
      card.hidden = true;
      card.style.maxWidth = "";
    }
  }

  function place(anchor) {
    var el = ensureCard();
    var box = anchor.getBoundingClientRect();
    var nav = document.getElementById("okmate-nav");
    var width;
    var height;
    var left;
    var top;
    if (nav && nav.contains(anchor)) {
      var navBox = nav.getBoundingClientRect();
      left = Math.round(navBox.right + 8);
      el.style.maxWidth = Math.max(0, window.innerWidth - left - 8) + "px";
      height = el.offsetHeight || 120;
      top = Math.round(box.top);
      if (top + height > window.innerHeight - 8) {
        top = Math.max(8, window.innerHeight - height - 8);
      }
    } else {
      el.style.maxWidth = "";
      width = el.offsetWidth || 280;
      height = el.offsetHeight || 120;
      left = Math.round(box.left);
      top = Math.round(box.bottom + 8);
      if (left + width > window.innerWidth - 8) {
        left = Math.max(8, window.innerWidth - width - 8);
      }
      if (top + height > window.innerHeight - 8) {
        top = Math.max(8, Math.round(box.top - height - 8));
      }
    }
    el.style.left = left + "px";
    el.style.top = top + "px";
  }

  function fill(data, footnoteRef) {
    var el = ensureCard();
    var type = el.querySelector(".okmate-peek-type");
    var kicker = el.querySelector(".okmate-peek-kicker");
    var title = el.querySelector(".okmate-peek-title");
    var desc = el.querySelector(".okmate-peek-desc");
    var excerpt = el.querySelector(".okmate-peek-excerpt");
    var hint = el.querySelector(".okmate-peek-hint");
    var kind = data.kind || "document";
    type.textContent = data.type || "";
    type.hidden = !data.type;
    if (kind === "heading" || kind === "footnote") {
      kicker.textContent = data.document_title || "";
      kicker.hidden = !data.document_title;
    } else {
      kicker.textContent = "";
      kicker.hidden = true;
    }
    title.textContent = data.title || data.document_title || "";
    desc.textContent = data.description || "";
    desc.hidden = !data.description;
    excerpt.textContent = data.excerpt || "";
    excerpt.hidden = !data.excerpt;
    hint.textContent = footnoteRef
      ? "Click to open"
      : "Click to open · ⌘/Ctrl+click: new tab";
    el.hidden = false;
  }

  function load(link, target) {
    if (abort) {
      abort.abort();
    }
    abort = new AbortController();
    var url =
      "/__okmate/peek?path=" +
      encodeURIComponent(target.path) +
      "&hash=" +
      encodeURIComponent(target.hash || "");
    fetch(url, { signal: abort.signal, headers: { Accept: "application/json" } })
      .then(function (response) {
        if (!response.ok) {
          throw new Error("peek");
        }
        return response.json();
      })
      .then(function (data) {
        if (activeLink !== link) {
          return;
        }
        fill(data, target.footnoteRef);
        place(link);
      })
      .catch(function (err) {
        if (err && err.name === "AbortError") {
          return;
        }
        hide();
      });
  }

  function arm(link) {
    var target = classifyHref(link.getAttribute("href") || "", window.location.pathname);
    if (!target || skipLink(link)) {
      return;
    }
    link.removeAttribute("title");
    if (timer) {
      window.clearTimeout(timer);
    }
    timer = window.setTimeout(function () {
      timer = 0;
      activeLink = link;
      load(link, target);
    }, DWELL_MS);
  }

  function onPointerOver(event) {
    var link = event.target.closest && event.target.closest("a[href]");
    if (!link) {
      return;
    }
    if (activeLink && activeLink !== link) {
      hide();
    }
    arm(link);
  }

  function onPointerOut(event) {
    var link = event.target.closest && event.target.closest("a[href]");
    if (!link) {
      return;
    }
    var next = event.relatedTarget;
    if (next && link.contains(next)) {
      return;
    }
    if (activeLink === link || timer) {
      hide();
    }
  }

  function bindTitles(root) {
    (root || document).querySelectorAll("a[href]").forEach(function (link) {
      if (skipLink(link)) {
        return;
      }
      if (classifyHref(link.getAttribute("href") || "", window.location.pathname)) {
        link.removeAttribute("title");
      }
    });
  }

  function enhance() {
    ensureCard();
    bindTitles(document);
    var main = document.getElementById("okmate-main");
    if (main && !main.__okmatePeekObserved) {
      main.__okmatePeekObserved = true;
      new MutationObserver(function () {
        bindTitles(main);
        hide();
      }).observe(main, { childList: true });
    }
  }

  document.addEventListener("pointerover", onPointerOver, true);
  document.addEventListener("pointerout", onPointerOut, true);
  document.addEventListener(
    "click",
    function () {
      hide();
    },
    true
  );
  document.addEventListener(
    "keydown",
    function (event) {
      if (event.key === "Escape") {
        hide();
      }
    },
    true
  );
  window.addEventListener(
    "scroll",
    function () {
      hide();
    },
    true
  );

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }

  window.__okmatePeek = {
    enhance: enhance,
    classifyHref: classifyHref,
    footnoteLabel: footnoteLabel,
  };
})();
