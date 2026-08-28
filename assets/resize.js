(function () {
  if (window.__okmateResize) {
    return;
  }
  var NAV_HOST = "#okmate-nav";
  var OUTLINE_HOST = "#okmate-toc";
  var SHELL = ".okmate-shell";
  var dragging = null;

  function remPx() {
    var size = parseFloat(window.getComputedStyle(document.documentElement).fontSize);
    return size > 0 ? size : 16;
  }

  function persistWidths() {
    var nav = document.documentElement.style.getPropertyValue("--okmate-nav-width").trim();
    var outline = document.documentElement.style.getPropertyValue("--okmate-outline-width").trim();
    if (window.__okmateReading && typeof window.__okmateReading.persist === "function") {
      window.__okmateReading.persist({
        nav_width: nav || null,
        outline_width: outline || null,
      });
    }
  }

  function visible(el) {
    if (!el) {
      return false;
    }
    var style = window.getComputedStyle(el);
    return style.display !== "none" && style.visibility !== "hidden" && el.clientWidth > 0;
  }

  function clamp(kind, px) {
    var rem = remPx();
    if (kind === "nav") {
      return Math.round(Math.min(Math.max(px, 12 * rem), Math.max(12 * rem, window.innerWidth * 0.42)));
    }
    return Math.round(Math.min(Math.max(px, 10 * rem), Math.max(10 * rem, window.innerWidth * 0.36)));
  }

  function setWidth(kind, px, persist) {
    var value = clamp(kind, px) + "px";
    var prop = kind === "nav" ? "--okmate-nav-width" : "--okmate-outline-width";
    document.documentElement.style.setProperty(prop, value);
    if (persist) {
      persistWidths();
    }
    placeAll();
  }

  function hostWidth(kind) {
    var host = document.querySelector(kind === "nav" ? NAV_HOST : OUTLINE_HOST);
    return host && visible(host) ? host.getBoundingClientRect().width : 0;
  }

  function placeHandle(handle) {
    var kind = handle.getAttribute("data-okmate-resize");
    var host = document.querySelector(kind === "nav" ? NAV_HOST : OUTLINE_HOST);
    var shell = handle.parentElement;
    if (!host || !visible(host) || !shell) {
      handle.style.display = "none";
      return;
    }
    handle.style.display = "";
    var shellBox = shell.getBoundingClientRect();
    var hostBox = host.getBoundingClientRect();
    var edge = kind === "nav" ? hostBox.right : hostBox.left;
    handle.style.left = Math.round(edge - shellBox.left - handle.offsetWidth / 2) + "px";
    handle.style.right = "auto";
  }

  function placeAll() {
    document.querySelectorAll(".okmate-col-resizer").forEach(placeHandle);
  }

  function bindHandle(handle, kind) {
    if (handle.__okmateBound) {
      return;
    }
    handle.__okmateBound = true;
    handle.addEventListener("pointerdown", function (event) {
      if (event.button !== 0) {
        return;
      }
      event.preventDefault();
      handle.setPointerCapture(event.pointerId);
      dragging = {
        kind: kind,
        startX: event.clientX,
        startW: hostWidth(kind),
      };
      handle.classList.add("is-active");
      document.body.classList.add("is-col-resizing");
    });
    handle.addEventListener("pointermove", function (event) {
      if (!dragging || dragging.kind !== kind) {
        return;
      }
      var delta = event.clientX - dragging.startX;
      if (kind === "outline") {
        delta = -delta;
      }
      setWidth(kind, dragging.startW + delta, false);
    });
    handle.addEventListener("pointerup", function (event) {
      if (!dragging || dragging.kind !== kind) {
        return;
      }
      try {
        handle.releasePointerCapture(event.pointerId);
      } catch (err) {}
      dragging = null;
      handle.classList.remove("is-active");
      document.body.classList.remove("is-col-resizing");
      persistWidths();
    });
    handle.addEventListener("keydown", function (event) {
      var step = event.shiftKey ? 32 : 16;
      if (event.key === "ArrowLeft") {
        event.preventDefault();
        setWidth(kind, hostWidth(kind) + (kind === "nav" ? -step : step), true);
      } else if (event.key === "ArrowRight") {
        event.preventDefault();
        setWidth(kind, hostWidth(kind) + (kind === "nav" ? step : -step), true);
      } else if (event.key === "Home") {
        event.preventDefault();
        setWidth(kind, kind === "nav" ? 12 * remPx() : 10 * remPx(), true);
      }
    });
  }

  function mount(host, kind) {
    var shell = host.closest(SHELL);
    if (!shell || !visible(host)) {
      return;
    }
    if (shell.querySelector(':scope > .okmate-col-resizer[data-okmate-resize="' + kind + '"]')) {
      return;
    }
    var handle = document.createElement("div");
    handle.className = "okmate-col-resizer";
    handle.setAttribute("data-okmate-resize", kind);
    handle.setAttribute("role", "separator");
    handle.setAttribute("aria-orientation", "vertical");
    handle.setAttribute("aria-label", kind === "nav" ? "Resize navigation" : "Resize outline");
    handle.tabIndex = 0;
    shell.appendChild(handle);
    bindHandle(handle, kind);
  }

  function enhance() {
    var nav = document.querySelector(NAV_HOST);
    var toc = document.querySelector(OUTLINE_HOST);
    if (nav) {
      mount(nav, "nav");
    }
    if (toc) {
      mount(toc, "outline");
    }
    placeAll();
  }

  window.__okmateResize = { enhance: enhance };
  window.addEventListener("resize", placeAll);
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
})();
