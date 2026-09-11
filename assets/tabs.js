(function () {
  if (window.__okmateTabs) {
    return;
  }

  var tabs = [];
  var active = "";
  var closed = [];

  function normalizeRoute(path) {
    var route = (path || "/").split(/[?#]/)[0];
    if (!route || route === "/") {
      return "/";
    }
    return "/" + route.replace(/^\/+|\/+$/g, "") + "/";
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

  function ensureStrip() {
    var el = document.getElementById("okmate-tabs");
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

  function render() {
    var el = ensureStrip();
    if (!el) {
      return;
    }
    if (tabs.length < 2) {
      el.hidden = true;
      el.replaceChildren();
      return;
    }
    el.hidden = false;
    el.replaceChildren();
    tabs.forEach(function (tab) {
      var node = document.createElement("div");
      node.className = "okmate-tab" + (tab.path === active ? " is-current" : "");
      node.setAttribute("role", "tab");
      node.setAttribute("aria-selected", tab.path === active ? "true" : "false");
      node.setAttribute("data-okmate-tab-path", tab.path);
      node.setAttribute("data-okmate-tab-hash", tab.hash || "");
      var open = document.createElement("button");
      open.type = "button";
      open.className = "okmate-tab-open";
      open.textContent = titleFor(tab);
      var close = document.createElement("button");
      close.type = "button";
      close.className = "okmate-tab-close";
      close.setAttribute("aria-label", "Close");
      close.textContent = "×";
      node.appendChild(open);
      node.appendChild(close);
      el.appendChild(node);
    });
  }

  function readDom() {
    var el = document.getElementById("okmate-tabs");
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
    render();
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
    } else {
      render();
      persist();
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
    if (closingActive) {
      var next = tabs[Math.min(index, tabs.length - 1)];
      active = next.path;
      activate(next.path, next.hash);
    } else {
      render();
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
    render();
    persist();
  }

  function onClick(event) {
    var close = event.target.closest && event.target.closest(".okmate-tab-close");
    if (close) {
      event.preventDefault();
      event.stopPropagation();
      var tab = close.closest("[data-okmate-tab-path]");
      if (tab) {
        closePath(tab.getAttribute("data-okmate-tab-path"));
      }
      return;
    }
    var open = event.target.closest && event.target.closest(".okmate-tab-open, .okmate-tab");
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
    render();
    var strip = document.getElementById("okmate-tabs");
    if (strip && !strip.__okmateTabsBound) {
      strip.__okmateTabsBound = true;
      strip.addEventListener("click", onClick);
      strip.addEventListener("auxclick", onAuxClick);
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
