(function () {
  if (window.__okmateTabs) {
    return;
  }

  var tabs = [];
  var active = "";
  var closed = [];
  var bound = false;

  function normalizeRoute(path) {
    var route = (path || "/").split(/[?#]/)[0];
    if (!route || route === "/") {
      return "/";
    }
    return "/" + route.replace(/^\/+|\/+$/g, "") + "/";
  }

  function hrefFor(tab) {
    return tab.hash ? tab.path + "#" + tab.hash : tab.path;
  }

  function titleFor(tab) {
    return tab.title || tab.path;
  }

  function hasIpc() {
    return !!(window.ipc && typeof window.ipc.postMessage === "function");
  }

  function persist() {
    if (!window.__okmateReading || typeof window.__okmateReading.persist !== "function") {
      return;
    }
    window.__okmateReading.persist({
      open_path: active || window.location.pathname,
      open_hash: (window.location.hash || "").replace(/^#/, "") || null,
      tabs: tabs.map(function (tab) {
        return { path: tab.path, hash: tab.hash || null, title: tab.title || "" };
      }),
    });
  }

  function stripEl() {
    return document.getElementById("okmate-tabs");
  }

  function ensureStrip() {
    var el = stripEl();
    if (el) {
      return el;
    }
    var app = document.querySelector(".okmate-app");
    var shell = document.querySelector(".okmate-shell");
    if (!app || !shell) {
      return null;
    }
    el = document.createElement("nav");
    el.id = "okmate-tabs";
    el.className = "okmate-tabs";
    el.setAttribute("aria-label", "Open documents");
    app.insertBefore(el, shell);
    return el;
  }

  function fillTabNode(node, tab, isCurrent) {
    node.classList.toggle("is-current", !!isCurrent);
    node.setAttribute("role", "tab");
    node.setAttribute("aria-selected", isCurrent ? "true" : "false");
    node.setAttribute("data-okmate-tab-path", tab.path);
    node.setAttribute("data-okmate-tab-hash", tab.hash || "");
    node.setAttribute("data-okmate-tab-href", hrefFor(tab));
    var open = node.querySelector(".okmate-tab-open");
    if (open) {
      open.textContent = titleFor(tab);
    }
  }

  function cloneTab(tab, isCurrent) {
    var tpl = document.getElementById("okmate-tab-template");
    var source = tpl && tpl.content && tpl.content.firstElementChild;
    if (!source) {
      return null;
    }
    var node = source.cloneNode(true);
    fillTabNode(node, tab, isCurrent);
    return node;
  }

  function findNode(el, path) {
    return el.querySelector('[data-okmate-tab-path="' + path + '"]');
  }

  function syncVisibility(el) {
    if (!el) {
      return;
    }
    el.hidden = tabs.length < 2;
  }

  function insertTabNode(tab, isCurrent) {
    var el = ensureStrip();
    if (!el || findNode(el, tab.path)) {
      return;
    }
    var node = cloneTab(tab, isCurrent);
    if (node) {
      el.appendChild(node);
    }
  }

  function syncStrip() {
    var el = tabs.length >= 2 ? ensureStrip() : stripEl();
    if (!el) {
      return;
    }
    tabs.forEach(function (tab) {
      var node = findNode(el, tab.path);
      if (node) {
        fillTabNode(node, tab, tab.path === active);
      } else {
        insertTabNode(tab, tab.path === active);
      }
    });
    el.querySelectorAll("[data-okmate-tab-path]").forEach(function (node) {
      var path = normalizeRoute(node.getAttribute("data-okmate-tab-path") || "");
      var keep = tabs.some(function (tab) {
        return tab.path === path;
      });
      if (!keep) {
        node.remove();
      }
    });
    syncVisibility(el);
  }

  function readDom() {
    var el = stripEl();
    if (!el) {
      return;
    }
    tabs = [];
    el.querySelectorAll("[data-okmate-tab-path]").forEach(function (node) {
      var path = normalizeRoute(node.getAttribute("data-okmate-tab-path") || "");
      tabs.push({
        path: path,
        hash: node.getAttribute("data-okmate-tab-hash") || "",
        title: ((node.querySelector(".okmate-tab-open") || {}).textContent || "").trim(),
      });
      if (node.classList.contains("is-current")) {
        active = path;
      }
    });
  }

  function findTab(path) {
    path = normalizeRoute(path);
    for (var i = 0; i < tabs.length; i += 1) {
      if (tabs[i].path === path) {
        return tabs[i];
      }
    }
    return null;
  }

  function rememberClosed(tab) {
    if (!tab) {
      return;
    }
    closed.push({ path: tab.path, hash: tab.hash || "", title: tab.title || "" });
    if (closed.length > 16) {
      closed.shift();
    }
  }

  function upsert(path, hash, title, activateTab) {
    path = normalizeRoute(path);
    var tab = findTab(path);
    if (!tab) {
      tab = { path: path, hash: "", title: "" };
      tabs.push(tab);
    }
    if (hash != null && hash !== "") {
      tab.hash = String(hash).replace(/^#/, "");
    }
    if (title) {
      tab.title = title;
    }
    if (activateTab) {
      active = path;
    }
    syncStrip();
    persist();
  }

  function activate(path, hash) {
    path = normalizeRoute(path);
    upsert(path, hash, "", true);
    var tab = findTab(path);
    if (window.__okmateNav && typeof window.__okmateNav.openRoute === "function") {
      window.__okmateNav.openRoute(path, hash != null && hash !== "" ? hash : tab && tab.hash);
    }
  }

  function openHref(href, options) {
    options = options || {};
    var dest;
    try {
      dest = new URL(href, window.location.href);
    } catch (err) {
      return;
    }
    if (dest.origin !== window.location.origin) {
      return;
    }
    var path = normalizeRoute(dest.pathname);
    var hash = (dest.hash || "").replace(/^#/, "");
    var activateTab = !!options.activate;
    upsert(path, hash, "", activateTab);
    if (activateTab) {
      activate(path, hash);
    }
  }

  function closePath(path) {
    path = normalizeRoute(path);
    if (tabs.length < 2) {
      return false;
    }
    var index = -1;
    tabs.forEach(function (tab, i) {
      if (tab.path === path) {
        index = i;
      }
    });
    if (index < 0) {
      return false;
    }
    rememberClosed(tabs[index]);
    var closingActive = tabs[index].path === active;
    tabs.splice(index, 1);
    var el = stripEl();
    var node = el && findNode(el, path);
    if (node) {
      node.remove();
    }
    syncVisibility(el);
    if (closingActive) {
      var next = tabs[Math.min(index, tabs.length - 1)];
      active = next.path;
      activate(next.path, next.hash);
    } else {
      persist();
    }
    return true;
  }

  function closeCurrent() {
    if (tabs.length >= 2) {
      return closePath(active);
    }
    if (hasIpc()) {
      window.ipc.postMessage("close-window");
    }
    return false;
  }

  function newTab() {
    var home = findTab("/");
    if (home) {
      activate("/", home.hash);
      return;
    }
    openHref("/", { activate: true });
  }

  function reopenClosed() {
    var tab = closed.pop();
    if (!tab) {
      return;
    }
    if (findTab(tab.path)) {
      activate(tab.path, tab.hash);
      return;
    }
    upsert(tab.path, tab.hash, tab.title, true);
    activate(tab.path, tab.hash);
  }

  function cycle(delta) {
    if (tabs.length < 2) {
      return;
    }
    var index = -1;
    tabs.forEach(function (tab, i) {
      if (tab.path === active) {
        index = i;
      }
    });
    if (index < 0) {
      index = 0;
    }
    var next = tabs[(index + delta + tabs.length) % tabs.length];
    activate(next.path, next.hash);
  }

  function jump(n) {
    if (!tabs.length) {
      return;
    }
    var tab = n === 9 ? tabs[tabs.length - 1] : tabs[n - 1];
    if (tab) {
      activate(tab.path, tab.hash);
    }
  }

  function afterPatch() {
    var path = normalizeRoute(window.location.pathname);
    var hash = (window.location.hash || "").replace(/^#/, "");
    var title = (document.title || "").trim();
    var existing = findTab(path);
    if (existing) {
      if (hash) {
        existing.hash = hash;
      }
      if (title) {
        existing.title = title;
      }
      active = path;
    } else {
      var current = findTab(active);
      if (current) {
        current.path = path;
        current.hash = hash;
        if (title) {
          current.title = title;
        }
        active = path;
      } else {
        upsert(path, hash, title, true);
        return;
      }
    }
    syncStrip();
    persist();
  }

  function onClick(event) {
    if (!event.target.closest || !event.target.closest("#okmate-tabs")) {
      return;
    }
    var close = event.target.closest(".okmate-tab-close");
    if (close) {
      event.preventDefault();
      event.stopPropagation();
      var tab = close.closest("[data-okmate-tab-path]");
      if (tab) {
        closePath(tab.getAttribute("data-okmate-tab-path"));
      }
      return;
    }
    var open = event.target.closest(".okmate-tab-open, .okmate-tab");
    if (!open) {
      return;
    }
    var node = open.closest("[data-okmate-tab-path]");
    if (!node || event.button !== 0) {
      return;
    }
    event.preventDefault();
    activate(node.getAttribute("data-okmate-tab-path"), node.getAttribute("data-okmate-tab-hash"));
  }

  function onAuxClick(event) {
    if (event.button !== 1) {
      return;
    }
    var tab = event.target.closest && event.target.closest("[data-okmate-tab-path]");
    if (tab && tab.closest("#okmate-tabs")) {
      event.preventDefault();
      closePath(tab.getAttribute("data-okmate-tab-path"));
    }
  }

  function typingTarget(event) {
    var target = event.target;
    if (!target) {
      return false;
    }
    var tag = (target.tagName || "").toLowerCase();
    if (tag === "input" || tag === "textarea" || tag === "select" || target.isContentEditable) {
      return true;
    }
    var gotoDialog = document.getElementById("okmate-goto");
    return !!(gotoDialog && gotoDialog.open);
  }

  function onKey(event) {
    if (typingTarget(event)) {
      return;
    }
    var desktop = hasIpc();
    if (event.key === "Tab" && event.ctrlKey && !event.metaKey && !event.altKey) {
      if (!desktop) {
        return;
      }
      event.preventDefault();
      cycle(event.shiftKey ? -1 : 1);
      return;
    }
    var meta = event.metaKey || event.ctrlKey;
    if (!meta || event.altKey || !desktop) {
      return;
    }
    var key = event.key;
    if (event.shiftKey && key.toLowerCase() === "t") {
      event.preventDefault();
      reopenClosed();
      return;
    }
    if (!event.shiftKey && key.toLowerCase() === "t") {
      event.preventDefault();
      newTab();
      return;
    }
    if (!event.shiftKey && key.toLowerCase() === "w") {
      event.preventDefault();
      closeCurrent();
      return;
    }
    if (!event.shiftKey && /^[1-9]$/.test(key)) {
      event.preventDefault();
      jump(parseInt(key, 10));
    }
  }

  function enhance() {
    readDom();
    if (!active) {
      active = normalizeRoute(window.location.pathname);
    }
    if (!tabs.length) {
      tabs.push({
        path: active,
        hash: (window.location.hash || "").replace(/^#/, ""),
        title: (document.title || "").trim(),
      });
    }
    syncVisibility(stripEl());
    if (!bound) {
      bound = true;
      document.addEventListener("click", onClick);
      document.addEventListener("auxclick", onAuxClick);
    }
    if (!window.__okmateTabsKeysBound) {
      window.__okmateTabsKeysBound = true;
      window.addEventListener("keydown", onKey);
    }
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }

  window.__h35NewTab = newTab;
  window.__h35CloseTab = function () {
    if (tabs.length < 2) {
      return false;
    }
    closePath(active);
    return true;
  };

  window.__okmateTabs = {
    enhance: enhance,
    afterPatch: afterPatch,
    openHref: openHref,
    persist: persist,
    newTab: newTab,
    closeCurrent: closeCurrent,
  };
})();
