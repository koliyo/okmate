(function () {
  function bindQueue(root) {
    var buttons = root.querySelectorAll(".okmate-filter-btn");
    var searchInput = root.querySelector("#okmate-search-input");
    var rows = root.querySelectorAll(".okmate-row");
    if (!buttons.length || !rows.length) return;
    if (root.dataset.okmateReviewBound === "1") return;
    root.dataset.okmateReviewBound = "1";
    var currentFilter = "all";
    var searchQuery = "";

    function updateRows() {
      for (var i = 0; i < rows.length; i++) {
        var row = rows[i];
        var status = row.getAttribute("data-status");
        var isAction = row.getAttribute("data-action") === "true";
        var searchData = row.getAttribute("data-search") || "";
        var matchesFilter = true;
        if (currentFilter === "action") matchesFilter = isAction;
        else if (currentFilter === "draft") matchesFilter = status === "draft";
        else if (currentFilter === "stable") matchesFilter = status === "stable";
        var matchesSearch = !searchQuery || searchData.indexOf(searchQuery) !== -1;
        row.hidden = !(matchesFilter && matchesSearch);
      }
    }

    for (var b = 0; b < buttons.length; b++) {
      buttons[b].addEventListener("click", function () {
        currentFilter = this.getAttribute("data-filter") || "all";
        for (var i = 0; i < buttons.length; i++) {
          buttons[i].classList.toggle("is-active", buttons[i] === this);
        }
        updateRows();
      });
    }
    if (searchInput) {
      searchInput.addEventListener("input", function () {
        searchQuery = (this.value || "").toLowerCase();
        updateRows();
      });
    }
  }

  function scan() {
    var queue = document.getElementById("okmate-queue");
    if (queue) bindQueue(queue);
  }

  document.addEventListener("DOMContentLoaded", scan);
  var main = document.getElementById("okmate-main");
  if (main && window.MutationObserver) {
    new MutationObserver(scan).observe(main, { childList: true, subtree: true });
  }
  scan();
})();
