(function () {
  if (window.__okmateReading) {
    return;
  }

  var WIDTH_KEY = "okmate-main-width";
  var WRAP_KEY = "okmate-main-wrap";
  var NAV_KEY = "okmate-nav-visible";
  var TOC_KEY = "okmate-toc-visible";
  var FONT_KEY = "okmate-font-size";
  var MIN_CH = 45;
  var MAX_CH = 90;
  var DEFAULT_CH = 66;
  var MIN_FONT = 80;
  var MAX_FONT = 160;
  var FONT_STEP = 10;
  var DEFAULT_FONT = 100;

  function readStore(key) {
    try {
      return window.localStorage.getItem(key) || "";
    } catch (err) {
      return "";
    }
  }

  function writeStore(key, value) {
    try {
      if (value) {
        window.localStorage.setItem(key, value);
      } else {
        window.localStorage.removeItem(key);
      }
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
    var raw = readStore(FONT_KEY);
    return raw ? parseFont(raw) : DEFAULT_FONT;
  }

  function setFont(percent) {
    var value = parseFont(String(percent));
    if (value === DEFAULT_FONT) {
      writeStore(FONT_KEY, "");
      document.documentElement.style.removeProperty("font-size");
    } else {
      writeStore(FONT_KEY, String(value));
      document.documentElement.style.fontSize = value + "%";
    }
  }

  function hasTocLinks() {
    return !!document.querySelector("#okmate-toc .okmate-toc-link");
  }

  function apply() {
    var root = document.documentElement;
    var ch = parseCh(readStore(WIDTH_KEY));
    if (ch == null) {
      root.style.removeProperty("--okmate-main-max-width");
    } else {
      root.style.setProperty("--okmate-main-max-width", widthCss(ch));
    }
    if (readStore(WRAP_KEY) === "off") {
      root.setAttribute("data-okmate-wrap", "off");
    } else {
      root.removeAttribute("data-okmate-wrap");
    }
    if (readStore(NAV_KEY) === "off") {
      root.setAttribute("data-okmate-nav", "off");
    } else {
      root.removeAttribute("data-okmate-nav");
    }
    if (readStore(TOC_KEY) === "off") {
      root.setAttribute("data-okmate-toc", "off");
    } else {
      root.removeAttribute("data-okmate-toc");
    }
    setFont(currentFont());
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
    var navToggle = document.getElementById("okmate-nav-toggle");
    var tocToggle = document.getElementById("okmate-toc-toggle");
    var fontValue = document.getElementById("okmate-font-value");
    var ch = parseCh(readStore(WIDTH_KEY));
    if (slider) {
      slider.value = String(ch == null ? DEFAULT_CH : ch);
      slider.setAttribute("aria-valuetext", widthLabel(ch));
    }
    if (output) {
      output.textContent = widthLabel(ch);
    }
    if (wrap) {
      wrap.checked = readStore(WRAP_KEY) !== "off";
    }
    if (navToggle) {
      navToggle.setAttribute("aria-pressed", readStore(NAV_KEY) === "off" ? "false" : "true");
    }
    if (tocToggle) {
      var tocOn = hasTocLinks();
      tocToggle.hidden = !tocOn;
      tocToggle.disabled = !tocOn;
      tocToggle.setAttribute("aria-pressed", readStore(TOC_KEY) === "off" ? "false" : "true");
    }
    if (fontValue) {
      fontValue.textContent = currentFont() + "%";
    }
  }

  function bind() {
    var slider = document.getElementById("okmate-main-width");
    var wrap = document.getElementById("okmate-main-wrap");
    var navToggle = document.getElementById("okmate-nav-toggle");
    var tocToggle = document.getElementById("okmate-toc-toggle");
    var fontSmaller = document.getElementById("okmate-font-smaller");
    var fontLarger = document.getElementById("okmate-font-larger");
    var fontValue = document.getElementById("okmate-font-value");
    if (slider && slider.dataset.bound !== "1") {
      slider.dataset.bound = "1";
      slider.addEventListener("input", function () {
        writeStore(WIDTH_KEY, widthCss(parseInt(slider.value, 10)));
        apply();
      });
    }
    if (wrap && wrap.dataset.bound !== "1") {
      wrap.dataset.bound = "1";
      wrap.addEventListener("change", function () {
        writeStore(WRAP_KEY, wrap.checked ? "" : "off");
        apply();
      });
    }
    if (navToggle && navToggle.dataset.bound !== "1") {
      navToggle.dataset.bound = "1";
      navToggle.addEventListener("click", function () {
        writeStore(NAV_KEY, readStore(NAV_KEY) === "off" ? "" : "off");
        apply();
      });
    }
    if (tocToggle && tocToggle.dataset.bound !== "1") {
      tocToggle.dataset.bound = "1";
      tocToggle.addEventListener("click", function () {
        writeStore(TOC_KEY, readStore(TOC_KEY) === "off" ? "" : "off");
        apply();
      });
    }
    if (fontSmaller && fontSmaller.dataset.bound !== "1") {
      fontSmaller.dataset.bound = "1";
      fontSmaller.addEventListener("click", function () {
        setFont(currentFont() - FONT_STEP);
        apply();
      });
    }
    if (fontLarger && fontLarger.dataset.bound !== "1") {
      fontLarger.dataset.bound = "1";
      fontLarger.addEventListener("click", function () {
        setFont(currentFont() + FONT_STEP);
        apply();
      });
    }
    if (fontValue && fontValue.dataset.bound !== "1") {
      fontValue.dataset.bound = "1";
      fontValue.addEventListener("click", function () {
        setFont(DEFAULT_FONT);
        apply();
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
    } else if (code === "Minus" || code === "NumpadSubtract" || key === "-" || key === "_") {
      event.preventDefault();
      setFont(currentFont() - FONT_STEP);
      apply();
    } else if (code === "Digit0" || code === "Numpad0" || key === "0") {
      event.preventDefault();
      setFont(DEFAULT_FONT);
      apply();
    }
  }

  function enhance() {
    apply();
    bind();
  }

  apply();
  window.__okmateReading = { enhance: enhance };
  window.addEventListener("keydown", onZoomKey);
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
})();
