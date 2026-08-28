(function () {
  if (window.__okmateTables) {
    return;
  }

  var MIN_REM = 4;
  var dragging = null;

  function remPx() {
    var size = parseFloat(window.getComputedStyle(document.documentElement).fontSize);
    return size > 0 ? size : 16;
  }

  function minCol() {
    return Math.round(MIN_REM * remPx());
  }

  function wrapOff() {
    return document.documentElement.getAttribute("data-okmate-wrap") === "off";
  }

  function headerCells(table) {
    var row = table.querySelector("tr");
    return row ? Array.prototype.slice.call(row.children) : [];
  }

  function snapshotWidths(table) {
    return headerCells(table).map(function (cell) {
      return Math.max(minCol(), Math.round(cell.getBoundingClientRect().width));
    });
  }

  function defaultCols(count) {
    if (wrapOff()) {
      return Array(count)
        .fill("max-content")
        .join(" ");
    }
    return "repeat(" + count + ", minmax(0, 1fr))";
  }

  function colsFromWidths(widths) {
    if (wrapOff()) {
      return widths
        .map(function (width) {
          return Math.max(minCol(), width) + "px";
        })
        .join(" ");
    }
    return widths
      .map(function (width) {
        return "minmax(" + MIN_REM + "rem, " + Math.max(1, width) + "fr)";
      })
      .join(" ");
  }

  function applyCols(table, value) {
    table.style.setProperty("--okmate-cols", value);
  }

  function resizeFrom(start, index, delta) {
    var next = start.slice();
    var right = index + 1;
    if (right < next.length) {
      var pair = start[index] + start[right];
      var left = Math.max(minCol(), Math.min(pair - minCol(), start[index] + delta));
      next[index] = left;
      next[right] = pair - left;
    } else {
      next[index] = Math.max(minCol(), start[index] + delta);
    }
    return next;
  }

  function resetTable(table) {
    applyCols(table, defaultCols(headerCells(table).length));
    placeHandles(table);
  }

  function wrapperOf(table) {
    return table.closest(".okmate-md-table");
  }

  function placeHandles(table) {
    var wrapper = wrapperOf(table);
    var cells = headerCells(table);
    if (!wrapper || !cells.length) {
      return;
    }
    var wrapBox = wrapper.getBoundingClientRect();
    var tableBox = table.getBoundingClientRect();
    wrapper.querySelectorAll(".okmate-table-resizer").forEach(function (handle) {
      var index = parseInt(handle.getAttribute("data-okmate-col"), 10);
      var cell = cells[index];
      if (!cell) {
        handle.style.display = "none";
        return;
      }
      var box = cell.getBoundingClientRect();
      handle.style.display = "";
      handle.style.left =
        Math.round(box.right - wrapBox.left - handle.offsetWidth / 2 + wrapper.scrollLeft) + "px";
      handle.style.top = Math.round(tableBox.top - wrapBox.top + wrapper.scrollTop) + "px";
      handle.style.height = Math.round(tableBox.height) + "px";
    });
  }

  function bindHandle(handle, table, index) {
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
      var widths = snapshotWidths(table);
      applyCols(table, colsFromWidths(widths));
      dragging = {
        table: table,
        index: index,
        startX: event.clientX,
        widths: widths,
      };
      handle.classList.add("is-active");
      document.body.classList.add("is-col-resizing");
    });
    handle.addEventListener("pointermove", function (event) {
      if (!dragging || dragging.table !== table || dragging.index !== index) {
        return;
      }
      applyCols(table, colsFromWidths(resizeFrom(dragging.widths, index, event.clientX - dragging.startX)));
      placeHandles(table);
    });
    handle.addEventListener("pointerup", function (event) {
      if (!dragging || dragging.table !== table) {
        return;
      }
      try {
        handle.releasePointerCapture(event.pointerId);
      } catch (err) {}
      dragging = null;
      handle.classList.remove("is-active");
      document.body.classList.remove("is-col-resizing");
      placeHandles(table);
    });
    handle.addEventListener("dblclick", function (event) {
      event.preventDefault();
      resetTable(table);
    });
    handle.addEventListener("keydown", function (event) {
      var step = event.shiftKey ? 32 : 16;
      if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
        event.preventDefault();
        var widths = snapshotWidths(table);
        var delta = event.key === "ArrowRight" ? step : -step;
        applyCols(table, colsFromWidths(resizeFrom(widths, index, delta)));
        placeHandles(table);
      } else if (event.key === "Home") {
        event.preventDefault();
        resetTable(table);
      }
    });
  }

  function mount(table) {
    var wrapper = wrapperOf(table);
    var cells = headerCells(table);
    if (!wrapper || !cells.length) {
      return;
    }
    if (!table.style.getPropertyValue("--okmate-cols")) {
      applyCols(table, defaultCols(cells.length));
    }
    if (table.__okmateCols) {
      placeHandles(table);
      return;
    }
    if (cells.length < 2) {
      return;
    }
    table.__okmateCols = true;
    for (var i = 0; i < cells.length - 1; i++) {
      var handle = document.createElement("div");
      handle.className = "okmate-table-resizer";
      handle.setAttribute("data-okmate-col", String(i));
      handle.setAttribute("role", "separator");
      handle.setAttribute("aria-orientation", "vertical");
      handle.setAttribute("aria-label", "Resize column");
      handle.tabIndex = 0;
      wrapper.appendChild(handle);
      bindHandle(handle, table, i);
    }
    if (!wrapper.__okmateTableScroll) {
      wrapper.__okmateTableScroll = true;
      wrapper.addEventListener("scroll", function () {
        wrapper.querySelectorAll("table").forEach(placeHandles);
      });
    }
    placeHandles(table);
  }

  function enhance() {
    document.querySelectorAll("#okmate-main .okmate-md-table table").forEach(mount);
  }

  function onResize() {
    document.querySelectorAll("#okmate-main .okmate-md-table table").forEach(placeHandles);
  }

  window.__okmateTables = { enhance: enhance };
  window.addEventListener("resize", onResize);
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance);
  } else {
    enhance();
  }
})();
