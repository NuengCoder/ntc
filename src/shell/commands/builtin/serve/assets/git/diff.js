/* NTC Git — shared unified-diff renderer.
 *
 * Parses `@@ -a,b +c,d @@` headers to put old/new line numbers in a gutter,
 * and colours added/removed/hunk/meta lines. No split view yet.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC || (global.NTC = {});

  function cell(className, text) {
    var node = document.createElement("span");
    node.className = className;
    node.textContent = text;
    return node;
  }

  function line(oldNo, newNo, kind, text) {
    var row = document.createElement("div");
    row.className = "dl" + (kind ? " " + kind : "");
    row.appendChild(cell("g", oldNo === null ? "" : String(oldNo)));
    row.appendChild(cell("g", newNo === null ? "" : String(newNo)));
    row.appendChild(cell("t", text === "" ? " " : text));
    return row;
  }

  function render(container, text, opts) {
    opts = opts || {};
    container.innerHTML = "";
    container.className = "diff" + (opts.wrap ? " wrap" : "");
    if (!text) {
      container.appendChild(line(null, null, "meta", opts.emptyText || "(no changes)"));
      return container;
    }

    var lines = text.split("\n");
    if (lines.length && lines[lines.length - 1] === "") lines.pop();

    var oldNo = 0;
    var newNo = 0;
    var inHunk = false;

    for (var i = 0; i < lines.length; i++) {
      var raw = lines[i];
      var kind = "";
      var left = null;
      var right = null;

      if (raw.indexOf("diff --git") === 0 || raw.indexOf("index ") === 0) {
        inHunk = false;
        kind = "meta";
      } else if (!inHunk && (raw.indexOf("--- ") === 0 || raw.indexOf("+++ ") === 0)) {
        kind = "meta";
      } else if (raw.indexOf("@@") === 0) {
        var match = /^@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@/.exec(raw);
        if (match) {
          oldNo = parseInt(match[1], 10);
          newNo = parseInt(match[2], 10);
        }
        inHunk = true;
        kind = "hunk";
      } else if (raw.indexOf("\\ No newline") === 0) {
        kind = "meta";
      } else if (inHunk && raw.charAt(0) === "+") {
        kind = "add";
        right = newNo++;
      } else if (inHunk && raw.charAt(0) === "-") {
        kind = "del";
        left = oldNo++;
      } else if (inHunk) {
        left = oldNo++;
        right = newNo++;
      } else if (raw.indexOf("…") === 0) {
        kind = "meta";
      }

      container.appendChild(line(left, right, kind, raw));
    }
    return container;
  }

  NTC.diff = {
    render: render,
    /* Small labelled section used by the inspector and Changes tab. */
    section: function (label, note) {
      return NTC.el(
        "div",
        { class: "diffhead" },
        NTC.el("span", { text: label }),
        NTC.el("span", { text: note || "" })
      );
    }
  };
})(window);
