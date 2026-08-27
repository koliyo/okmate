(function () {
  if (window.__okmateNav) {
    return;
  }

  var SECTIONS_KEY = "okmate-nav-sections";
  var SCROLL_KEY = "okmate-nav-scroll";
  var pendingRoute = "";

  function normalizeRoute(path) {
    var route = (path || "/").split(/[?#]/)[0];
    if (!route || route === "/") {
      return "/";
    }
    return "/" + route.replace(/^\/+|\/+$/g, "") + "/";
  }

  function readSections() {
    try {
      return JSON.parse(sessionStorage.getItem(SECTIONS_KEY) || "{}");
    } catch (err) {
      return {};
    }
  }

  function writeSections(state) {
    try {
      sessionStorage.setItem(SECTIONS_KEY, JSON.stringify(state));
    } catch (err) {}
  }

  function navRoot() {
    return document.getElementById("okmate-nav");
  }

  function rememberScroll() {
    var nav = navRoot();
    if (!nav) {
      return;
    }
    try {
      sessionStorage.setItem(SCROLL_KEY, String(nav.scrollTop));
    } catch (err) {}
  }

  function restoreScroll() {
    var nav = navRoot();
    if (!nav) {
      return;
    }
    var raw = "";
    try {
      raw = sessionStorage.getItem(SCROLL_KEY) || "";
    } catch (err) {}
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
      if (section.hasAttribute("data-okmate-nav-current") || (key && state[key])) {
        section.open = true;
      } else if (key && Object.prototype.hasOwnProperty.call(state, key)) {
        section.open = !!state[key];
      }
    });
  }

  function syncNav(route) {
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
    nav.querySelectorAll('a[href="' + route + '"]').forEach(function (link) {
      link.classList.add("is-current");
      link.setAttribute("aria-current", "page");
    });
    nav.querySelectorAll("details[data-okmate-nav-section]").forEach(function (section) {
      var key = section.getAttribute("data-okmate-nav-section") || "";
      if (sectionMatches(route, key)) {
        section.setAttribute("data-okmate-nav-current", "");
        section.open = true;
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

  function resetMainScroll() {
    var main = document.getElementById("okmate-main");
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

  function afterDocumentPatch() {
    var route = pendingRoute || window.location.pathname;
    if (pendingRoute) {
      if (normalizeRoute(window.location.pathname) !== normalizeRoute(pendingRoute)) {
        try {
          history.pushState(null, "", pendingRoute);
        } catch (err) {}
      }
      pendingRoute = "";
    }
    syncNav(route);
    restoreSections();
    resetMainScroll();
    if (window.__okmateToc && typeof window.__okmateToc.enhance === "function") {
      window.__okmateToc.enhance();
    }
    if (window.__okmateResize && typeof window.__okmateResize.enhance === "function") {
      window.__okmateResize.enhance();
    }
    if (window.__okmateReading && typeof window.__okmateReading.enhance === "function") {
      window.__okmateReading.enhance();
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
        if (section.hasAttribute("data-okmate-nav-current") && section.open) {
          return;
        }
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
      var link = event.target.closest && event.target.closest("#okmate-nav a[href]");
      if (!link || event.button !== 0) {
        return;
      }
      var href = link.getAttribute("href") || "";
      if (!href || href.charAt(0) === "#" || href.indexOf("/__okmate/") === 0) {
        return;
      }
      pendingRoute = href;
    },
    true
  );

  window.addEventListener("popstate", function () {
    syncNav(window.location.pathname);
    restoreSections();
    resetMainScroll();
  });
  window.addEventListener("pagehide", function () {
    rememberAllSections();
    rememberScroll();
  });

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
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
  window.__okmateNav = { enhance: enhance, sync: syncNav };
})();
