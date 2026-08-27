(function () {
  function bindWindow(root) {
    if (!root || root.dataset.okmateLogWindow === "1") return;
    if (!root.querySelector("[data-okmate-sentinel]")) return;
    root.dataset.okmateLogWindow = "1";
    if (!window.IntersectionObserver) return;
    var observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (!entry.isIntersecting) return;
          var start = entry.target.getAttribute("data-start");
          if (start == null) return;
          fetch("/__okmate/log-window?start=" + encodeURIComponent(start))
            .then(function (response) {
              return response.text();
            })
            .then(function (html) {
              var parsed = new DOMParser().parseFromString(html, "text/html");
              var next = parsed.getElementById("okmate-log-window");
              var current = document.getElementById("okmate-log-window");
              if (!next || !current) return;
              current.replaceWith(next);
              var log = document.getElementById("okmate-log");
              if (log) delete log.dataset.okmateLogWindow;
              bindWindow(log);
            });
        });
      },
      { rootMargin: "120px 0px" }
    );
    var sentinels = root.querySelectorAll("[data-okmate-sentinel]");
    for (var i = 0; i < sentinels.length; i++) observer.observe(sentinels[i]);
  }

  function scan() {
    bindWindow(document.getElementById("okmate-log"));
  }

  document.addEventListener("DOMContentLoaded", scan);
  var main = document.getElementById("okmate-main");
  if (main && window.MutationObserver) {
    new MutationObserver(scan).observe(main, { childList: true, subtree: true });
  }
  scan();
})();
