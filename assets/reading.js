(function () {
  if (window.__okmateReading) {
    return;
  }

  var LEGACY_WIDTH_KEY = "okmate-main-width";
  var LEGACY_WRAP_KEY = "okmate-main-wrap";
  var LEGACY_NAV_KEY = "okmate-nav-visible";
  var LEGACY_TOC_KEY = "okmate-toc-visible";
  var LEGACY_FONT_KEY = "okmate-font-size";
  var MIN_CH = 45;
  var MAX_CH = 100;
  var DEFAULT_CH = 66;
  var MIN_FONT = 80;
  var MAX_FONT = 160;
  var FONT_STEP = 10;
  var DEFAULT_FONT = 100;
  var persistTimer = null;
  var state = {
    font: DEFAULT_FONT,
    width: null,
    wrap: true,
    full: false,
    nav: true,
    toc: true,
    navWidth: "",
    outlineWidth: "",
  };

  function readStore(key) {
    try {
      return window.localStorage.getItem(key) || "";
    } catch (err) {
      return "";
    }
  }

  function clearStore(key) {
    try {
      window.localStorage.removeItem(key);
    } catch (err) {}
  }

  function parseCh(raw) {
    if (!raw) {
      return null;
    }
    var match = String(raw).match(/^(\d{1,3})ch$/);
    if (!match) {
      return null;
    }
    var value = parseInt(match[1], 10);
    if (isNaN(value)) {
      return null;
    }
    return Math.min(MAX_CH, Math.max(MIN_CH, value));
  }

  function widthCss(ch) {
    return ch + "ch";
  }

  function parseFont(raw) {
    var value = parseInt(raw, 10);
    if (isNaN(value)) {
      return DEFAULT_FONT;
    }
    return Math.min(MAX_FONT, Math.max(MIN_FONT, Math.round(value / FONT_STEP) * FONT_STEP));
  }

  function currentFont() {
    return state.font;
  }

  function setFont(percent) {
    state.font = parseFont(String(percent));
  }

  function hasTocLinks() {
    return !!document.querySelector("#okmate-toc .okmate-toc-link");
  }

  function persist(extra) {
    var body = {
      font_size: state.font,
      main_width: state.width,
      wrap: state.wrap,
      full_width: state.full,
      nav_visible: state.nav,
      toc_visible: state.toc,
    };
    if (extra) {
      Object.keys(extra).forEach(function (key) {
        body[key] = extra[key];
      });
    }
    clearTimeout(persistTimer);
    persistTimer = setTimeout(function () {
      fetch("/__okmate/prefs", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(body),
      }).catch(function () {});
    }, 200);
  }

  function apply() {
    var root = document.documentElement;
    if (state.width == null) {
      root.style.removeProperty("--okmate-main-max-width");
    } else {
      root.style.setProperty("--okmate-main-max-width", widthCss(state.width));
    }
    if (!state.wrap) {
      root.setAttribute("data-okmate-wrap", "off");
    } else {
      root.removeAttribute("data-okmate-wrap");
    }
    if (state.full) {
      root.setAttribute("data-okmate-full", "on");
    } else {
      root.removeAttribute("data-okmate-full");
    }
    if (!state.nav) {
      root.setAttribute("data-okmate-nav", "off");
    } else {
      root.removeAttribute("data-okmate-nav");
    }
    if (!state.toc) {
      root.setAttribute("data-okmate-toc", "off");
    } else {
      root.removeAttribute("data-okmate-toc");
    }
    if (state.font === DEFAULT_FONT) {
      root.style.removeProperty("font-size");
    } else {
      root.style.fontSize = state.font + "%";
    }
    syncControls();
    if (window.__okmateResize && typeof window.__okmateResize.enhance === "function") {
      window.__okmateResize.enhance();
    }
  }

  function widthLabel(ch) {
    return (ch == null ? DEFAULT_CH : ch) + "ch";
  }

  function syncControls() {
    var slider = document.getElementById("okmate-main-width");
    var output = document.getElementById("okmate-main-width-value");
    var wrap = document.getElementById("okmate-main-wrap");
    var full = document.getElementById("okmate-main-full");
    var navToggle = document.getElementById("okmate-nav-toggle");
    var tocToggle = document.getElementById("okmate-toc-toggle");
    var fontValue = document.getElementById("okmate-font-value");
    if (slider) {
      slider.value = String(state.width == null ? DEFAULT_CH : state.width);
      slider.setAttribute("aria-valuetext", state.full ? "full" : widthLabel(state.width));
      slider.disabled = state.full;
    }
    if (output) {
      output.textContent = state.full ? "full" : widthLabel(state.width);
    }
    if (wrap) {
      wrap.checked = state.wrap;
    }
    if (full) {
      full.checked = state.full;
    }
    if (navToggle) {
      navToggle.setAttribute("aria-pressed", state.nav ? "true" : "false");
    }
    if (tocToggle) {
      var tocOn = hasTocLinks();
      tocToggle.hidden = !tocOn;
      tocToggle.disabled = !tocOn;
      tocToggle.setAttribute("aria-pressed", state.toc ? "true" : "false");
    }
    if (fontValue) {
      fontValue.textContent = state.font + "%";
    }
  }

  function bind() {
    var slider = document.getElementById("okmate-main-width");
    var wrap = document.getElementById("okmate-main-wrap");
    var full = document.getElementById("okmate-main-full");
    var navToggle = document.getElementById("okmate-nav-toggle");
    var tocToggle = document.getElementById("okmate-toc-toggle");
    var fontSmaller = document.getElementById("okmate-font-smaller");
    var fontLarger = document.getElementById("okmate-font-larger");
    var fontValue = document.getElementById("okmate-font-value");
    if (slider && slider.dataset.bound !== "1") {
      slider.dataset.bound = "1";
      slider.addEventListener("input", function () {
        state.width = parseInt(slider.value, 10);
        apply();
        persist();
      });
    }
    if (wrap && wrap.dataset.bound !== "1") {
      wrap.dataset.bound = "1";
      wrap.addEventListener("change", function () {
        state.wrap = wrap.checked;
        apply();
        persist();
      });
    }
    if (full && full.dataset.bound !== "1") {
      full.dataset.bound = "1";
      full.addEventListener("change", function () {
        state.full = full.checked;
        apply();
        persist();
      });
    }
    if (navToggle && navToggle.dataset.bound !== "1") {
      navToggle.dataset.bound = "1";
      navToggle.addEventListener("click", function () {
        state.nav = !state.nav;
        apply();
        persist();
      });
    }
    if (tocToggle && tocToggle.dataset.bound !== "1") {
      tocToggle.dataset.bound = "1";
      tocToggle.addEventListener("click", function () {
        state.toc = !state.toc;
        apply();
        persist();
      });
    }
    if (fontSmaller && fontSmaller.dataset.bound !== "1") {
      fontSmaller.dataset.bound = "1";
      fontSmaller.addEventListener("click", function () {
        setFont(currentFont() - FONT_STEP);
        apply();
        persist();
      });
    }
    if (fontLarger && fontLarger.dataset.bound !== "1") {
      fontLarger.dataset.bound = "1";
      fontLarger.addEventListener("click", function () {
        setFont(currentFont() + FONT_STEP);
        apply();
        persist();
      });
    }
    if (fontValue && fontValue.dataset.bound !== "1") {
      fontValue.dataset.bound = "1";
      fontValue.addEventListener("click", function () {
        setFont(DEFAULT_FONT);
        apply();
        persist();
      });
    }
  }

  function onZoomKey(event) {
    if (!(event.metaKey || event.ctrlKey) || event.altKey) {
      return;
    }
    var code = event.code;
    var key = event.key;
    if (code === "Equal" || code === "NumpadAdd" || key === "+" || key === "=") {
      event.preventDefault();
      setFont(currentFont() + FONT_STEP);
      apply();
      persist();
    } else if (code === "Minus" || code === "NumpadSubtract" || key === "-" || key === "_") {
      event.preventDefault();
      setFont(currentFont() - FONT_STEP);
      apply();
      persist();
    } else if (code === "Digit0" || code === "Numpad0" || key === "0") {
      event.preventDefault();
      setFont(DEFAULT_FONT);
      apply();
      persist();
    }
  }

  function seedFromDom() {
    var root = document.documentElement;
    var fontRaw = root.style.fontSize;
    if (fontRaw) {
      state.font = parseFont(fontRaw);
    }
    state.width = parseCh(root.style.getPropertyValue("--okmate-main-max-width"));
    state.wrap = root.getAttribute("data-okmate-wrap") !== "off";
    state.full = root.getAttribute("data-okmate-full") === "on";
    state.nav = root.getAttribute("data-okmate-nav") !== "off";
    state.toc = root.getAttribute("data-okmate-toc") !== "off";
    state.navWidth = root.style.getPropertyValue("--okmate-nav-width").trim();
    state.outlineWidth = root.style.getPropertyValue("--okmate-outline-width").trim();
  }

  function migrateLegacy() {
    var seeded =
      state.font !== DEFAULT_FONT ||
      state.width != null ||
      !state.wrap ||
      !state.nav ||
      !state.toc ||
      state.navWidth ||
      state.outlineWidth;
    if (seeded) {
      return;
    }
    var width = parseCh(readStore(LEGACY_WIDTH_KEY));
    var fontRaw = readStore(LEGACY_FONT_KEY);
    var wrapOff = readStore(LEGACY_WRAP_KEY) === "off";
    var navOff = readStore(LEGACY_NAV_KEY) === "off";
    var tocOff = readStore(LEGACY_TOC_KEY) === "off";
    var navWidth = readStore("okmate-nav-width");
    var outlineWidth = readStore("okmate-outline-width");
    if (
      width == null &&
      !fontRaw &&
      !wrapOff &&
      !navOff &&
      !tocOff &&
      !navWidth &&
      !outlineWidth
    ) {
      return;
    }
    if (width != null) {
      state.width = width;
    }
    if (fontRaw) {
      state.font = parseFont(fontRaw);
    }
    state.wrap = !wrapOff;
    state.nav = !navOff;
    state.toc = !tocOff;
    persist({
      nav_width: navWidth || null,
      outline_width: outlineWidth || null,
    });
    [
      LEGACY_WIDTH_KEY,
      LEGACY_WRAP_KEY,
      LEGACY_NAV_KEY,
      LEGACY_TOC_KEY,
      LEGACY_FONT_KEY,
      "okmate-nav-width",
      "okmate-outline-width",
    ].forEach(clearStore);
  }

  function enhance() {
    apply();
    bind();
  }

  seedFromDom();
  migrateLegacy();
  apply();
  window.__okmateReading = { enhance: enhance, persist: persist };
  window.addEventListener("keydown", onZoomKey);
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
})();
