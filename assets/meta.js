(function () {
  if (window.__okmateMeta) {
    return;
  }

  var REL_KEY = "okmate-rel-open";

  function readStore(key) {
    try {
      return window.localStorage.getItem(key) || "";
    } catch (err) {
      return "";
    }
  }

  function writeStore(key, value) {
    try {
      window.localStorage.setItem(key, value);
    } catch (err) {}
  }

  function formatStamp(iso) {
    var date = new Date(iso);
    if (isNaN(date.getTime())) {
      return iso;
    }
    return date.toLocaleString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "numeric",
      minute: "2-digit",
      timeZoneName: "short",
    });
  }

  function formatTimes() {
    document.querySelectorAll(".okmate-summary-time[datetime]").forEach(function (el) {
      var iso = el.getAttribute("datetime") || "";
      if (!iso) {
        return;
      }
      el.textContent = formatStamp(iso);
    });
  }

  function relOpenDefault() {
    var raw = readStore(REL_KEY);
    if (raw === "0") {
      return false;
    }
    return true;
  }

  function bindRel() {
    var details = document.getElementById("okmate-rel");
    if (!details) {
      return;
    }
    details.open = relOpenDefault();
    if (details.dataset.bound === "1") {
      return;
    }
    details.dataset.bound = "1";
    details.addEventListener("toggle", function () {
      writeStore(REL_KEY, details.open ? "1" : "0");
    });
  }

  function enhance() {
    formatTimes();
    bindRel();
  }

  window.__okmateMeta = { enhance: enhance };
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
})();
