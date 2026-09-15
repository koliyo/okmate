(function () {
  if (window.__okmateMove) {
    return;
  }
  if (!document.documentElement.hasAttribute("data-okmate-live")) {
    return;
  }
  var dialog = document.getElementById("okmate-move");
  var fromEl = document.getElementById("okmate-move-from");
  var toEl = document.getElementById("okmate-move-to");
  var summaryEl = document.getElementById("okmate-move-summary");
  var notesEl = document.getElementById("okmate-move-notes");
  var cancel = document.getElementById("okmate-move-cancel");
  var confirm = document.getElementById("okmate-move-confirm");
  if (!dialog || !fromEl || !toEl || !summaryEl || !notesEl || !cancel || !confirm) {
    return;
  }

  var pending = null;
  var dragRoot = "";
  var dragFrom = "";

  function showModal() {
    if (typeof dialog.showModal === "function" && !dialog.open) {
      dialog.showModal();
    } else if (!dialog.open) {
      dialog.setAttribute("open", "");
    }
  }

  function closeModal() {
    dialog.close();
    pending = null;
  }

  function queryUrl(root, from, to) {
    var params = new URLSearchParams();
    if (root) {
      params.set("root", root);
    }
    params.set("from", from);
    params.set("to", to);
    return "/__okmate/move?" + params.toString();
  }

  function renderPlan(plan) {
    fromEl.textContent = plan.from_path || plan.from || "";
    toEl.value = plan.to || toEl.value;
    var edits = plan.edits || [];
    summaryEl.textContent =
      edits.length === 1 ? "1 file edit" : edits.length + " file edits";
    notesEl.replaceChildren();
    (plan.notes || []).forEach(function (note) {
      var item = document.createElement("li");
      item.textContent = note;
      notesEl.appendChild(item);
    });
  }

  function loadPlan() {
    if (!pending) {
      return Promise.resolve();
    }
    var to = (toEl.value || "").trim();
    pending.to = to;
    return fetch(queryUrl(pending.root, pending.from, to)).then(function (response) {
      return response.text().then(function (text) {
        if (!response.ok) {
          summaryEl.textContent = text || "Move preview failed.";
          notesEl.replaceChildren();
          return;
        }
        var plan = JSON.parse(text);
        renderPlan(plan);
      });
    });
  }

  function collectionTarget(node) {
    while (node && node !== document) {
      if (node.getAttribute && node.getAttribute("data-okmate-drop-collection")) {
        return node;
      }
      node = node.parentNode;
    }
    return null;
  }

  document.addEventListener("dragstart", function (event) {
    var link = event.target.closest && event.target.closest("[data-okmate-concept-id]");
    if (!link || !event.dataTransfer) {
      return;
    }
    dragFrom = link.getAttribute("data-okmate-concept-id") || "";
    dragRoot = link.getAttribute("data-okmate-root") || "";
    event.dataTransfer.setData("text/plain", dragFrom);
    event.dataTransfer.effectAllowed = "move";
  });

  document.addEventListener("dragend", function () {
    dragFrom = "";
    dragRoot = "";
    document.querySelectorAll(".nav-section.is-drop-target").forEach(function (section) {
      section.classList.remove("is-drop-target");
    });
  });

  document.addEventListener("dragover", function (event) {
    var target = collectionTarget(event.target);
    if (!target || !dragFrom) {
      return;
    }
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
    document.querySelectorAll(".nav-section.is-drop-target").forEach(function (section) {
      if (section !== target) {
        section.classList.remove("is-drop-target");
      }
    });
    target.classList.add("is-drop-target");
  });

  document.addEventListener("drop", function (event) {
    var target = collectionTarget(event.target);
    if (!target || !dragFrom) {
      return;
    }
    event.preventDefault();
    var collection = target.getAttribute("data-okmate-drop-collection") || "";
    var dropRoot = target.getAttribute("data-okmate-drop-root") || "";
    if (dropRoot && dragRoot && dropRoot !== dragRoot) {
      return;
    }
    pending = {
      root: dragRoot,
      from: dragFrom,
      to: collection ? collection.replace(/\/?$/, "/") : "",
    };
    fromEl.textContent = pending.from;
    toEl.value = pending.to;
    summaryEl.textContent = "Loading preview…";
    notesEl.replaceChildren();
    showModal();
    loadPlan();
  });

  toEl.addEventListener("change", function () {
    loadPlan();
  });
  cancel.addEventListener("click", closeModal);
  confirm.addEventListener("click", function () {
    if (!pending) {
      return;
    }
    var body = {
      root: pending.root,
      from: pending.from,
      to: (toEl.value || "").trim(),
      apply: true,
    };
    fetch("/__okmate/move", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    }).then(function (response) {
      return response.text().then(function (text) {
        if (!response.ok) {
          summaryEl.textContent = text || "Move failed.";
          return;
        }
        var plan = JSON.parse(text);
        closeModal();
        if (plan.href) {
          window.location.assign(plan.href);
        } else {
          window.location.reload();
        }
      });
    });
  });

  window.__okmateMove = true;
})();
