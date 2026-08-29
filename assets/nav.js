(function () {
  if (window.__okmateNav) {
    return;
  }

  var sectionState = {};
  var pendingRoute = "";
  var afterPatch = "auto";
  var mountedRoute = normalizeRoute(window.location.pathname);
  if (history.scrollRestoration) {
    history.scrollRestoration = "manual";
  }

  function normalizeRoute(path) {
    var route = (path || "/").split(/[?#]/)[0];
    if (!route || route === "/") {
      return "/";
    }
    return "/" + route.replace(/^\/+|\/+$/g, "") + "/";
  }

  function readSections() {
    return sectionState;
  }

  function persistNav(immediate) {
    var nav = navRoot();
    if (window.__okmateReading && typeof window.__okmateReading.persist === "function") {
      window.__okmateReading.persist(
        {
          nav_sections: sectionState,
          nav_scroll: nav ? nav.scrollTop : 0,
        },
        immediate
      );
    }
  }

  function writeSections(state) {
    sectionState = state;
    persistNav(false);
  }

  function navRoot() {
    return document.getElementById("okmate-nav");
  }

  function rememberScroll() {
    persistNav(false);
  }

  function restoreScroll() {
    var nav = navRoot();
    if (!nav) {
      return;
    }
    var raw = document.documentElement.getAttribute("data-okmate-nav-scroll") || "";
    var top = parseInt(raw, 10);
    if (!isNaN(top)) {
      nav.scrollTop = top;
    }
  }

  function rememberAllSections() {
    var nav = navRoot();
    if (!nav) {
      return;
    }
    var state = readSections();
    nav.querySelectorAll("details[data-okmate-nav-section]").forEach(function (section) {
      var key = section.getAttribute("data-okmate-nav-section");
      if (key) {
        state[key] = !!section.open;
      }
    });
    writeSections(state);
  }

  function restoreSections() {
    var nav = navRoot();
    if (!nav) {
      return;
    }
    var state = readSections();
    nav.querySelectorAll("details[data-okmate-nav-section]").forEach(function (section) {
      var key = section.getAttribute("data-okmate-nav-section");
      if (key && Object.prototype.hasOwnProperty.call(state, key)) {
        section.open = !!state[key];
      }
    });
  }

  function markCurrent(link) {
    link.classList.add("is-current");
    link.setAttribute("aria-current", "page");
  }

  function collectionPath(route) {
    var match = route.match(/^\/@[^/]+\/(.*)$/);
    var inner = match ? match[1] : route.replace(/^\/+/, "");
    return inner.replace(/\/+$/g, "");
  }

  function syncNav(route, expandCurrent) {
    var nav = navRoot();
    if (!nav) {
      return;
    }
    route = normalizeRoute(route || window.location.pathname);
    nav.querySelectorAll("[data-okmate-nav-current]").forEach(function (el) {
      el.removeAttribute("data-okmate-nav-current");
    });
    nav.querySelectorAll(".is-current").forEach(function (el) {
      el.classList.remove("is-current");
    });
    nav.querySelectorAll('[aria-current="page"]').forEach(function (el) {
      el.removeAttribute("aria-current");
    });
    nav.querySelectorAll('a[href="' + route + '"]').forEach(markCurrent);
    var collection = collectionPath(route);
    if (collection) {
      nav.querySelectorAll('a[data-okmate-collection="' + collection + '"]').forEach(markCurrent);
    }
    nav.querySelectorAll("details[data-okmate-nav-section]").forEach(function (section) {
      var key = section.getAttribute("data-okmate-nav-section") || "";
      if (sectionMatches(route, key)) {
        section.setAttribute("data-okmate-nav-current", "");
        if (expandCurrent) {
          section.open = true;
        }
      }
    });
  }

  function sectionMatches(route, key) {
    if (!key) {
      return false;
    }
    var prefix = "/" + key.replace(/^\/+|\/+$/g, "") + "/";
    if (route === prefix || route.indexOf(prefix) === 0) {
      return true;
    }
    var at = route.match(/^\/@([^/]+)\/(.*)$/);
    if (!at) {
      return false;
    }
    var namespaced = "/" + at[1] + "/" + at[2];
    if (namespaced === prefix || namespaced.indexOf(prefix) === 0) {
      return true;
    }
    var inner = "/" + at[2];
    return inner === prefix || inner.indexOf(prefix) === 0;
  }

  function mainEl() {
    return document.getElementById("okmate-main");
  }

  function mainScroll() {
    var main = mainEl();
    return main ? Math.max(0, Math.round(main.scrollTop)) : 0;
  }

  function applyMainScroll(top) {
    var main = mainEl();
    if (main && typeof top === "number" && !isNaN(top)) {
      main.scrollTop = Math.max(0, top);
    }
  }

  function historyState(extra) {
    var state = { mainScroll: mainScroll() };
    if (extra) {
      Object.keys(extra).forEach(function (key) {
        state[key] = extra[key];
      });
    }
    return state;
  }

  function replaceHistory(url) {
    try {
      if (url) {
        history.replaceState(historyState(), "", url);
      } else {
        history.replaceState(historyState(), "");
      }
    } catch (err) {}
  }

  function rememberHere() {
    replaceHistory();
  }

  function beginInPage(href) {
    rememberHere();
    try {
      history.pushState(historyState({ mainScroll: 0 }), "", href);
    } catch (err) {}
  }

  function finishInPage(href) {
    replaceHistory(href);
    persistLocation();
  }

  function resetMainScroll() {
    var main = mainEl();
    if (main) {
      main.scrollTop = 0;
    }
    var hash = window.location.hash;
    if (!hash || hash === "#") {
      return;
    }
    var id = decodeURIComponent(hash.slice(1));
    var el = document.getElementById(id);
    if (!el || !main) {
      return;
    }
    var top =
      main.scrollTop + el.getBoundingClientRect().top - main.getBoundingClientRect().top;
    main.scrollTop = Math.max(0, top);
  }

  function applyHistoryScroll(state) {
    if (state && typeof state.mainScroll === "number") {
      applyMainScroll(state.mainScroll);
      return true;
    }
    resetMainScroll();
    return false;
  }

  function sameDocumentUrl(url) {
    return normalizeRoute(url.pathname) === mountedRoute;
  }

  function syncTitle() {
    var crumb = document.querySelector(".okmate-crumb-current");
    var heading = document.querySelector("#okmate-main h1");
    var title = ((crumb && crumb.textContent) || (heading && heading.textContent) || "").trim();
    if (title) {
      document.title = title;
    }
  }

  function persistLocation() {
    var hash = (window.location.hash || "").replace(/^#/, "");
    if (window.__okmateReading && typeof window.__okmateReading.persist === "function") {
      window.__okmateReading.persist({
        open_path: window.location.pathname,
        open_hash: hash || null,
        main_scroll: mainScroll(),
      });
    }
  }

  function restoreMainLocation() {
    if (history.state && typeof history.state.mainScroll === "number") {
      applyMainScroll(history.state.mainScroll);
      return;
    }
    var hash = window.location.hash;
    if (hash && hash !== "#") {
      resetMainScroll();
      return;
    }
    var raw = document.documentElement.getAttribute("data-okmate-main-scroll") || "";
    var top = parseInt(raw, 10);
    if (!isNaN(top) && top > 0) {
      applyMainScroll(top);
    }
  }

  function reportLocation() {
    var href = window.location.href;
    if (window.ipc && window.ipc.postMessage) {
      window.ipc.postMessage("location:" + href);
    }
    persistLocation();
  }

  function afterDocumentPatch() {
    var route = pendingRoute || window.location.pathname + window.location.search;
    if (pendingRoute) {
      if (normalizeRoute(window.location.pathname) !== normalizeRoute(pendingRoute)) {
        try {
          history.pushState({ mainScroll: 0 }, "", pendingRoute);
        } catch (err) {}
      }
      pendingRoute = "";
      afterPatch = window.location.hash && window.location.hash !== "#" ? "hash" : "top";
    }
    var expand = !!pendingRoute || normalizeRoute(route) !== mountedRoute;
    syncNav(route, expand);
    if (expand) {
      rememberAllSections();
    }
    restoreSections();
    if (typeof afterPatch === "number") {
      applyMainScroll(afterPatch);
    } else if (afterPatch === "hash") {
      resetMainScroll();
    } else if (afterPatch === "top") {
      applyMainScroll(0);
    } else if (afterPatch === "auto") {
      restoreMainLocation();
    }
    afterPatch = "auto";
    mountedRoute = normalizeRoute(window.location.pathname);
    replaceHistory();
    syncTitle();
    reportLocation();
    if (window.__okmateToc && typeof window.__okmateToc.enhance === "function") {
      window.__okmateToc.enhance();
    }
    if (window.__okmateResize && typeof window.__okmateResize.enhance === "function") {
      window.__okmateResize.enhance();
    }
    if (window.__okmateReading && typeof window.__okmateReading.enhance === "function") {
      window.__okmateReading.enhance();
    }
    if (window.__okmateTables && typeof window.__okmateTables.enhance === "function") {
      window.__okmateTables.enhance();
    }
    if (window.__okmateMeta && typeof window.__okmateMeta.enhance === "function") {
      window.__okmateMeta.enhance();
    }
  }

  function observeMain() {
    var main = document.getElementById("okmate-main");
    if (!main || main.__okmateNavObserved) {
      return;
    }
    main.__okmateNavObserved = true;
    // Datastar patches #okmate-main (and toc) without replacing #okmate-nav.
    new MutationObserver(function () {
      afterDocumentPatch();
    }).observe(main, { childList: true });
  }

  document.addEventListener(
    "click",
    function (event) {
      var summary = event.target.closest && event.target.closest("details.nav-section > summary");
      if (summary && summary.closest("#okmate-nav") && event.button === 0) {
        event.preventDefault();
        var section = summary.parentElement;
        var key = section.getAttribute("data-okmate-nav-section");
        var opening = !section.open;
        var nav = navRoot();
        if (nav && key) {
          nav.querySelectorAll('details[data-okmate-nav-section="' + key + '"]').forEach(function (copy) {
            copy.open = opening;
          });
        } else {
          section.open = opening;
        }
        rememberAllSections();
        return;
      }
      var link = event.target.closest && event.target.closest("a[href]");
      if (!link || event.button !== 0) {
        return;
      }
      var href = link.getAttribute("href") || "";
      if (!href || href.indexOf("/__okmate/") === 0) {
        return;
      }
      var dest;
      try {
        dest = new URL(link.href, window.location.href);
      } catch (err) {
        return;
      }
      if (dest.origin === window.location.origin && dest.hash && sameDocumentUrl(dest)) {
        if (link.classList.contains("okmate-toc-link") || link.classList.contains("okmate-outline-link")) {
          return;
        }
        event.preventDefault();
        beginInPage(dest.pathname + dest.search + dest.hash);
        resetMainScroll();
        finishInPage(dest.pathname + dest.search + dest.hash);
        return;
      }
      var action =
        link.getAttribute("data-on:click__prevent") || link.getAttribute("data-on:click") || "";
      if (action.indexOf("@get") === -1 && !link.closest("#okmate-nav")) {
        return;
      }
      rememberHere();
      pendingRoute = href;
    },
    true
  );

  function requestDocument(href) {
    if (!href) {
      return;
    }
    var probe = document.createElement("button");
    probe.type = "button";
    probe.hidden = true;
    probe.setAttribute("data-on:click", "@get('" + href.replace(/'/g, "\\'") + "')");
    document.body.appendChild(probe);
    setTimeout(function () {
      probe.click();
      probe.remove();
    }, 0);
  }

  window.addEventListener("popstate", function (event) {
    syncNav(window.location.pathname);
    restoreSections();
    reportLocation();
    if (sameDocumentUrl(window.location)) {
      applyHistoryScroll(event.state);
      return;
    }
    afterPatch =
      event.state && typeof event.state.mainScroll === "number" ? event.state.mainScroll : "hash";
    requestDocument(window.location.pathname + window.location.search);
  });
  window.addEventListener("pagehide", function () {
    rememberAllSections();
    persistNav(true);
    persistLocation();
  });
  window.addEventListener(
    "scroll",
    function (event) {
      var main = document.getElementById("okmate-main");
      var nav = navRoot();
      if (main && event.target === main) {
        persistLocation();
      }
      if (nav && event.target === nav) {
        persistNav(false);
      }
    },
    true
  );

  function placeBlurb(blurb) {
    var summary = blurb.closest("summary");
    var nav = navRoot();
    if (!summary || !nav) {
      return;
    }
    var navBox = nav.getBoundingClientRect();
    var row = summary.getBoundingClientRect();
    var left = Math.round(navBox.right + 8);
    var top = Math.round(row.top);
    var room = window.innerWidth - left - 12;
    blurb.style.left = left + "px";
    blurb.style.top = top + "px";
    blurb.style.maxWidth = Math.max(12 * 16, Math.min(16 * 16, room)) + "px";
    var height = blurb.offsetHeight;
    if (top + height > window.innerHeight - 8) {
      blurb.style.top = Math.max(8, window.innerHeight - height - 8) + "px";
    }
  }

  function bindBlurbs() {
    var nav = navRoot();
    if (!nav || nav.__okmateBlurbs) {
      return;
    }
    nav.__okmateBlurbs = true;
    function onAim(event) {
      var summary = event.target.closest && event.target.closest("details.nav-section > summary");
      if (!summary || !nav.contains(summary)) {
        return;
      }
      var blurb = summary.querySelector(":scope > .okmate-nav-blurb");
      if (blurb) {
        placeBlurb(blurb);
      }
    }
    nav.addEventListener("mouseover", onAim);
    nav.addEventListener("focusin", onAim);
  }

  function enhance() {
    observeMain();
    bindBlurbs();
    syncNav(window.location.pathname);
    restoreSections();
    restoreScroll();
    restoreMainLocation();
    syncTitle();
    reportLocation();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
  window.__okmateNav = {
    enhance: enhance,
    sync: syncNav,
    persistLocation: persistLocation,
    rememberHere: rememberHere,
    beginInPage: beginInPage,
    finishInPage: finishInPage,
  };
})();
