(function () {
  if (window.__okmateReload) {
    return;
  }
  var dialog = document.getElementById("okmate-reload");
  var message = document.getElementById("okmate-reload-message");
  var later = document.getElementById("okmate-reload-later");
  var confirm = document.getElementById("okmate-reload-confirm");
  if (!dialog || !message || !later || !confirm) {
    return;
  }

  function show(text) {
    message.textContent = text || "Knowledge roots changed. Reload the workspace?";
    if (typeof dialog.showModal === "function" && !dialog.open) {
      dialog.showModal();
    } else if (!dialog.open) {
      dialog.setAttribute("open", "");
    }
  }

  later.addEventListener("click", function () {
    dialog.close();
  });
  confirm.addEventListener("click", function () {
    fetch("/__okmate/reload-workspace", { method: "POST" }).then(function (response) {
      if (response.ok) {
        window.location.reload();
        return;
      }
      message.textContent = "Workspace reload failed.";
    });
  });

  var es = null;
  function connect() {
    if (es) {
      return;
    }
    es = new EventSource("/__okmate/events");
    es.addEventListener("config-reload", function (event) {
      show(event.data);
    });
    es.onerror = function () {
      if (es) {
        es.close();
      }
      es = null;
      setTimeout(connect, 1000);
    };
  }
  connect();
  window.__okmateReload = { show: show };
})();
