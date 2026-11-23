/* NTC Git — Branches tab.
 *
 * Switch, create, rename, delete, merge and rebase. The conflict resolver
 * lives in Actions; this tab only points at it, so destructive verbs are not
 * next to a branch list.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var list = el("div", { class: "list tall" });
  var panel = ui.panel("Local branches");
  var status = ui.status();
  var selected = "";
  var newName = ui.input("new branch name");
  newName.classList.add("grow");
  var renameName = ui.input("new name for the selected branch");
  renameName.classList.add("grow");

  var switchButton = ui.button("Switch", doSwitch, "primary");
  var deleteButton = ui.button("Delete", doDelete, "danger", "git branch -d (refuses unmerged branches)");
  var mergeButton = ui.button("Merge into current", doMerge, null, "git merge --no-edit <selected>");
  var rebaseButton = ui.button("Rebase onto selected", doRebase, null, "git rebase <selected>");
  var renameButton = ui.button("Rename", doRename);
  var conflictNote = el("div", { class: "muted", style: "margin-top:10px" });
  var selectedLabel = el("span", { class: "mono", text: "—" });

  function current() {
    return store.branches.current || "";
  }

  function updateControls() {
    var isCurrent = selected && selected === current();
    switchButton.disabled = !selected || isCurrent;
    deleteButton.disabled = !selected || isCurrent;
    mergeButton.disabled = !selected || isCurrent;
    rebaseButton.disabled = !selected || isCurrent;
    renameButton.disabled = !selected;
  }

  function markActive() {
    var rows = list.querySelectorAll(".crow");
    for (var i = 0; i < rows.length; i++) {
      rows[i].classList.toggle("active", rows[i].dataset.name === selected);
    }
    selectedLabel.textContent = selected || "—";
    updateControls();
  }

  function render() {
    var branches = store.branches.list || [];
    NTC.clear(list);
    panel.count(branches.length + (branches.length === 1 ? " branch" : " branches"));
    if (!branches.length) {
      list.appendChild(ui.empty("No local branches yet."));
    }
    branches.forEach(function (name) {
      var row = el(
        "div",
        {
          class: "crow" + (name === selected ? " active" : ""),
          dataset: { name: name },
          title: name === current() ? "Current branch" : "Select " + name,
          tabindex: "0"
        },
        el("span", { class: "name", text: name }),
        el("span", { class: "summary" }),
        name === current() ? el("span", { class: "badge current", text: "current" }) : null
      );
      row.addEventListener("click", function () {
        selected = name;
        markActive();
      });
      row.addEventListener("dblclick", function () {
        if (name !== current()) doSwitch();
      });
      row.addEventListener("keydown", function (e) {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          selected = name;
          markActive();
        }
      });
      list.appendChild(row);
    });
    markActive();

    var conflicts = store.status.conflicts.length;
    if (conflicts) {
      NTC.clear(conflictNote);
      conflictNote.appendChild(
        el("span", {
          class: "status err",
          text: conflicts + " conflicted file(s) — resolve them in "
        })
      );
      conflictNote.appendChild(
        ui.button("Actions", function () {
          NTC.go("actions");
        }, "mini")
      );
    } else if (store.repo.rebasing) {
      conflictNote.textContent = "Rebase in progress — finish it from Actions.";
    } else {
      conflictNote.textContent = "";
    }
  }

  function doSwitch() {
    if (!selected) return;
    NTC.mutate("/api/branch/switch", { name: selected }, "Switched to " + selected + ".", [
      "status",
      "log",
      "branches"
    ]);
  }

  function doCreate() {
    var name = newName.value.trim();
    if (!name) {
      status.set("Enter a branch name.", "err");
      return;
    }
    NTC.mutate("/api/branch/create", { name: name }, "Created and switched to " + name + ".", [
      "status",
      "log",
      "branches"
    ]).then(function (data) {
      if (data) newName.value = "";
    });
  }

  function doRename() {
    var target = renameName.value.trim();
    if (!selected || !target) {
      status.set("Select a branch and enter its new name.", "err");
      return;
    }
    NTC.mutate(
      "/api/branch/rename",
      { name: selected, new_name: target },
      "Renamed " + selected + " to " + target + ".",
      ["status", "branches"]
    ).then(function (data) {
      if (data) {
        selected = target;
        renameName.value = "";
      }
    });
  }

  function doDelete() {
    if (!selected) return;
    NTC.confirm(
      'Delete branch "' + selected + '"? This runs git branch -d, which refuses unmerged branches.',
      "Delete"
    ).then(function (ok) {
      if (!ok) return;
      NTC.mutate("/api/branch/delete", { name: selected }, "Deleted " + selected + ".", [
        "status",
        "branches"
      ]);
    });
  }

  function doMerge() {
    if (!selected) return;
    var into = current() || "the current branch";
    NTC.mutate("/api/branch/merge", { name: selected }, "Merged " + selected + " into " + into + ".", [
      "status",
      "log"
    ]);
  }

  function doRebase() {
    if (!selected) return;
    var from = current() || "the current branch";
    NTC.confirm(
      "Rebase " + from + " onto " + selected + "? This rewrites the local commits of " + from + ".",
      "Rebase"
    ).then(function (ok) {
      if (!ok) return;
      NTC.mutate("/api/branch/rebase", { name: selected }, "Rebased " + from + " onto " + selected + ".", [
        "status",
        "log",
        "branches"
      ]);
    });
  }

  function refresh() {
    return NTC.api("/api/branches")
      .then(function (data) {
        store.branches.current = data.current || "";
        store.branches.detached = !!data.detached;
        store.branches.list = data.branches || [];
        if (selected && store.branches.list.indexOf(selected) < 0) selected = "";
        if (!selected) selected = store.branches.current || "";
        render();
        if (data.detached) {
          status.set("Detached HEAD — create a branch to keep new commits.", "err");
        }
      })
      .catch(function (err) {
        status.set(err.message, "err");
      });
  }

  function mount(root) {
    panel.body.appendChild(list);
    panel.body.appendChild(
      ui.row(
        el("span", { class: "muted", text: "Selected:" }),
        selectedLabel,
        el("span", { class: "spacer" }),
        switchButton,
        deleteButton
      )
    );
    panel.body.appendChild(ui.row(mergeButton, rebaseButton));

    var create = ui.panel("Create branch");
    create.body.appendChild(ui.row(el("span", { class: "muted", text: "Name:" }), newName, ui.button("Create & switch", doCreate, "primary")));
    newName.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        e.preventDefault();
        doCreate();
      }
    });

    var rename = ui.panel("Rename selected");
    rename.body.appendChild(ui.row(renameName, renameButton));
    renameName.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        e.preventDefault();
        doRename();
      }
    });

    root.appendChild(panel.root);
    root.appendChild(create.root);
    root.appendChild(rename.root);
    panel.body.appendChild(status.el);
    panel.body.appendChild(conflictNote);
  }

  NTC.tabs.register({
    id: "branches",
    label: "Branches",
    icon: "⑂",
    hint: "Switch, create, merge and rebase",
    mount: mount,
    refresh: refresh,
    badge: function () {
      return store.branches.list.length;
    }
  });
})(window);
