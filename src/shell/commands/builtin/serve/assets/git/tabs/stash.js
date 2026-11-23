/* NTC Git — Stash tab.
 *
 * Stash tracked changes, then apply / pop / drop an entry. Untracked files
 * are left alone by `git stash push` without --include-untracked, so the tab
 * says so instead of silently ignoring them.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var panel = ui.panel("Stashes");
  var status = ui.status();
  var list = el("div", { class: "list" });
  var message = ui.input("stash message (optional)", { maxlength: 200 });
  message.classList.add("grow");
  var selected = -1;

  var applyButton = ui.button("Apply", function () {
    if (selected < 0) return;
    act("/api/stash/apply", { index: selected, drop: false }, "Applied stash@{" + selected + "}.");
  }, "primary");
  var popButton = ui.button("Pop", function () {
    if (selected < 0) return;
    act("/api/stash/apply", { index: selected, drop: true }, "Popped stash@{" + selected + "}.");
  });
  var dropButton = ui.button("Drop", function () {
    if (selected < 0) return;
    var index = selected;
    NTC.confirm("Drop stash@{" + index + "}? This cannot be undone.", "Drop").then(function (ok) {
      if (!ok) return;
      act("/api/stash/drop", { index: index }, "Dropped stash@{" + index + "}.");
    });
  }, "danger");

  function updateControls() {
    applyButton.disabled = selected < 0;
    popButton.disabled = selected < 0;
    dropButton.disabled = selected < 0;
  }

  function render() {
    var stashes = store.stash.list || [];
    NTC.clear(list);
    panel.count(stashes.length + (stashes.length === 1 ? " stash" : " stashes"));
    if (!stashes.length) {
      selected = -1;
      list.appendChild(ui.empty("No stashes."));
      updateControls();
      return;
    }
    if (selected >= stashes.length) selected = -1;
    stashes.forEach(function (stash, index) {
      var row = el(
        "div",
        {
          class: "crow" + (index === selected ? " active" : ""),
          title: "Select " + stash.ref,
          tabindex: "0"
        },
        el("span", { class: "ref", text: stash.ref }),
        el("span", { class: "summary", text: stash.message || "(no message)", title: stash.message || "" })
      );
      row.addEventListener("click", function () {
        selected = index;
        render();
      });
      row.addEventListener("keydown", function (e) {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          selected = index;
          render();
        }
      });
      list.appendChild(row);
    });
    updateControls();
  }

  function act(url, payload, okMessage) {
    selected = -1;
    return NTC.mutate(url, payload, okMessage, ["status", "log", "stashes"]).then(function (data) {
      if (data) render();
      return data;
    });
  }

  function push() {
    var text = message.value.trim();
    NTC.mutate("/api/stash", { message: text }, "Stashed changes.", ["status", "log", "stashes"]).then(
      function (data) {
        if (data) {
          message.value = "";
          render();
        }
      }
    );
  }

  function refresh() {
    return NTC.api("/api/stashes")
      .then(function (data) {
        store.stash.list = data.stashes || [];
        store.stash.loaded = true;
        render();
      })
      .catch(function (err) {
        NTC.clear(list);
        list.appendChild(ui.empty(err.message));
      });
  }

  function mount(root) {
    panel.body.appendChild(list);
    panel.body.appendChild(ui.row(applyButton, popButton, dropButton));
    panel.body.appendChild(status.el);

    var pushPanel = ui.panel("Stash current changes");
    pushPanel.body.appendChild(ui.row(message, ui.button("Stash", push, "primary")));
    pushPanel.body.appendChild(
      el("div", {
        class: "muted",
        style: "margin-top:8px",
        text: "Untracked files are not stashed unless you include them from the terminal (git stash push -u)."
      })
    );
    message.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        e.preventDefault();
        push();
      }
    });

    root.appendChild(panel.root);
    root.appendChild(pushPanel.root);
  }

  NTC.tabs.register({
    id: "stash",
    label: "Stash",
    icon: "≡",
    hint: "Stash, apply, pop and drop",
    mount: mount,
    refresh: refresh,
    activate: function (args) {
      if (!args || !args[0]) return;
      var index = parseInt(args[0], 10);
      if (!isNaN(index)) {
        selected = index;
        render();
      }
    },
    badge: function () {
      return store.stash.list.length;
    }
  });
})(window);
