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

  function headerCells(table) {
    var row = table.querySelector("tr");
    return row ? Array.prototype.slice.call(row.children) : [];
  }

  function ensureColgroup(table, count) {
    var group = table.querySelector(":scope > colgroup");
    if (!group) {
      group = document.createElement("colgroup");
      table.insertBefore(group, table.firstChild);
    }
    while (group.children.length < count) {
      group.appendChild(document.createElement("col"));
    }
    while (group.children.length > count) {
      group.removeChild(group.lastChild);
    }
    return group;
  }

  function snapshotWidths(table) {
    return headerCells(table).map(function (cell) {
      return Math.max(minCol(), Math.round(cell.getBoundingClientRect().width));
    });
  }

  function applyFixed(table, widths) {
    var group = ensureColgroup(table, widths.length);
    var total = 0;
    for (var i = 0; i < group.children.length; i++) {
      var width = widths[i] || minCol();
      group.children[i].style.width = width + "px";
      total += width;
    }
    table.style.tableLayout = "fixed";
    table.style.width = total + "px";
    table.style.minWidth = total + "px";
  }

  function resetTable(table) {
    table.style.tableLayout = "";
    table.style.width = "";
    table.style.minWidth = "";
    var group = table.querySelector(":scope > colgroup");
    if (group) {
      group.remove();
    }
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
      applyFixed(table, widths);
      dragging = {
        table: table,
        index: index,
        startX: event.clientX,
        startW: widths[index],
        widths: widths,
      };
      handle.classList.add("is-active");
      document.body.classList.add("is-col-resizing");
    });
    handle.addEventListener("pointermove", function (event) {
      if (!dragging || dragging.table !== table || dragging.index !== index) {
        return;
      }
      var next = dragging.widths.slice();
      next[index] = Math.max(minCol(), dragging.startW + (event.clientX - dragging.startX));
      applyFixed(table, next);
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
        widths[index] = Math.max(minCol(), widths[index] + delta);
        applyFixed(table, widths);
        placeHandles(table);
      } else if (event.key === "Home") {
        event.preventDefault();
        resetTable(table);
      }
    });
  }

  function mount(table) {
    if (table.__okmateCols) {
      placeHandles(table);
      return;
    }
    var wrapper = wrapperOf(table);
    var cells = headerCells(table);
    if (!wrapper || cells.length < 2) {
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
