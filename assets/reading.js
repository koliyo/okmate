(function () {
  if (window.__okmateReading) {
    return;
  }

  var WIDTH_KEY = "okmate-main-width";
  var WRAP_KEY = "okmate-main-wrap";
  var MIN_CH = 45;
  var MAX_CH = 90;
  var DEFAULT_CH = 66;

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
    syncControls();
  }

  function widthLabel(ch) {
    return (ch == null ? DEFAULT_CH : ch) + "ch";
  }

  function syncControls() {
    var slider = document.getElementById("okmate-main-width");
    var output = document.getElementById("okmate-main-width-value");
    var reset = document.getElementById("okmate-main-width-reset");
    var wrap = document.getElementById("okmate-main-wrap");
    var ch = parseCh(readStore(WIDTH_KEY));
    if (slider) {
      slider.value = String(ch == null ? DEFAULT_CH : ch);
      slider.setAttribute("aria-valuetext", widthLabel(ch));
    }
    if (output) {
      output.textContent = widthLabel(ch);
    }
    if (reset) {
      reset.hidden = ch == null || ch === DEFAULT_CH;
    }
    if (wrap) {
      wrap.checked = readStore(WRAP_KEY) !== "off";
    }
  }

  function bind() {
    var slider = document.getElementById("okmate-main-width");
    var reset = document.getElementById("okmate-main-width-reset");
    var wrap = document.getElementById("okmate-main-wrap");
    if (slider && slider.dataset.bound !== "1") {
      slider.dataset.bound = "1";
      slider.addEventListener("input", function () {
        writeStore(WIDTH_KEY, widthCss(parseInt(slider.value, 10)));
        apply();
      });
    }
    if (reset && reset.dataset.bound !== "1") {
      reset.dataset.bound = "1";
      reset.addEventListener("click", function () {
        writeStore(WIDTH_KEY, "");
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
  }

  function enhance() {
    apply();
    bind();
  }

  apply();
  window.__okmateReading = { enhance: enhance };
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
})();
