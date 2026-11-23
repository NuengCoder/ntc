/* NTC Git — Changes tab.
 *
 * Working-tree workflow in one place: filter, select, stage/unstage/discard,
 * read the diff in the inspector, and commit from the sticky footer.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var MODES = [
    { id: "all", label: "All" },
    { id: "staged", label: "Staged" },
    { id: "unstaged", label: "Unstaged" },
    { id: "untracked", label: "Untracked" },
    { id: "conflicts", label: "Conflicts" }
  ];

  var selected = new Set();
  var anchor = -1;
  var activePath = "";
  var query = "";
  var mode = "all";

  var search = ui.input("Filter paths…", { type: "search" });
  search.classList.add("grow");
  var list = el("div", { class: "list tall" });
  var message = el("textarea", {
    placeholder: "Commit message (staged changes only)",
    spellcheck: "true"
  });
  var panel = ui.panel("Changes");
  var status = ui.status();
  var seg = el("div", { class: "seg" });
  var modeButtons = {};

  var stageButton = ui.button("Stage selected", function () {
    stagePaths(Array.from(selected));
  }, "primary");
  var unstageButton = ui.button("Unstage selected", function () {
    unstagePaths(Array.from(selected));
  });
  var discardButton = ui.button("Discard selected", function () {
    discardPaths(Array.from(selected));
  }, "danger", "git restore -- (keeps staged changes)");
  var commitButton = ui.button("Commit staged", commit, "primary");
  var selectAllButton = ui.button("Select all", toggleAll, "mini");
  var stagedTag = el("span", { class: "count", text: "0 staged" });

  function files() {
    return store.status.files || [];
  }

  function matches(file) {
    if (mode === "staged" && !file.index) return false;
    if (mode === "unstaged" && !file.worktree) return false;
    if (mode === "untracked" && !file.untracked) return false;
    if (mode === "conflicts" && !file.conflict) return false;
    if (query && file.path.toLowerCase().indexOf(query) < 0) return false;
    return true;
  }

  function visible() {
    return files().filter(matches);
  }

  function codeClass(file) {
    if (file.conflict) return "conflict";
    if (file.untracked) return "untracked";
    if (file.code === "A") return "add";
    if (file.code === "D") return "del";
    if (file.code === "?") return "untracked";
    if (file.code === " ") return "clean";
    return "";
  }

  function prune() {
    var paths = {};
    files().forEach(function (file) {
      paths[file.path] = true;
    });
    Array.from(selected).forEach(function (path) {
      if (!paths[path]) selected.delete(path);
    });
    if (activePath && !paths[activePath]) activePath = "";
  }

  function updateControls() {
    var rows = visible();
    var all = rows.length > 0 && rows.every(function (file) {
      return selected.has(file.path);
    });
    selectAllButton.textContent = all ? "Clear" : "Select all";
    selectAllButton.disabled = rows.length === 0;
    stageButton.disabled = selected.size === 0;
    unstageButton.disabled = selected.size === 0;
    discardButton.disabled = selected.size === 0;
    stagedTag.textContent = store.status.staged + " staged";
    panel.count(files().length + (files().length === 1 ? " file" : " files"));
    commitButton.disabled =
      store.repo.rebasing ||
      (!store.status.staged && !store.repo.merging) ||
      !message.value.trim();
    message.placeholder = store.repo.rebasing
      ? "Rebase in progress — continue it from Actions"
      : store.repo.merging
        ? "Merge commit message (merge in progress)"
        : "Commit message (staged changes only)";
  }

  function rowActions(file) {
    var box = el("span", { class: "actions" });
    if (!file.index || file.worktree) {
      box.appendChild(
        el("button", {
          text: "Stage",
          title: "git add -- " + file.path,
          onclick: function (e) {
            e.stopPropagation();
            stagePaths([file.path]);
          }
        })
      );
    }
    if (file.index) {
      box.appendChild(
        el("button", {
          text: "Unstage",
          title: "git restore --staged -- " + file.path,
          onclick: function (e) {
            e.stopPropagation();
            unstagePaths([file.path]);
          }
        })
      );
    }
    if (file.worktree && !file.untracked && !file.conflict) {
      box.appendChild(
        el("button", {
          class: "danger",
          text: "Discard",
          title: "git restore -- " + file.path,
          onclick: function (e) {
            e.stopPropagation();
            discardPaths([file.path]);
          }
        })
      );
    }
    return box;
  }

  function render() {
    prune();
    NTC.clear(list);
    var rows = visible();
    if (!rows.length) {
      list.appendChild(
        ui.empty(
          files().length
            ? "No files match this filter."
            : "Working tree clean — nothing to stage."
        )
      );
      updateControls();
      return;
    }

    rows.forEach(function (file) {
      var row = el("div", {
        class: "entry" + (file.path === activePath ? " active" : ""),
        dataset: { path: file.path }
      });

      var box = el("input", { type: "checkbox" });
      box.checked = selected.has(file.path);
      box.addEventListener("click", function (e) {
        e.stopPropagation();
        var index = rows.indexOf(file);
        if (e.shiftKey && anchor >= 0 && anchor < rows.length) {
          var start = Math.min(anchor, index);
          var end = Math.max(anchor, index);
          for (var i = start; i <= end; i++) {
            if (box.checked) selected.add(rows[i].path);
            else selected.delete(rows[i].path);
          }
          render();
          return;
        }
        if (box.checked) selected.add(file.path);
        else selected.delete(file.path);
        anchor = index;
        updateControls();
      });

      var code = el("span", {
        class: "code " + codeClass(file),
        text: file.conflict ? "U" : file.untracked ? "?" : file.code || "M",
        title: file.conflict
          ? "conflict"
          : file.untracked
            ? "untracked"
            : "index " + (file.index || "-") + " / worktree " + (file.worktree || "-")
      });

      var name = el("button", { class: "name", title: file.path });
      var slash = file.path.lastIndexOf("/");
      if (slash > 0) {
        name.appendChild(el("span", { class: "dir", text: file.path.slice(0, slash + 1) }));
        name.appendChild(document.createTextNode(file.path.slice(slash + 1)));
      } else {
        name.textContent = file.path;
      }
      name.addEventListener("click", function (e) {
        e.stopPropagation();
        openDiff(file.path);
      });

      var flags = el("span", { class: "flags" });
      if (file.index) flags.appendChild(el("span", { class: "flag staged", text: "staged" }));
      if (file.worktree) flags.appendChild(el("span", { class: "flag", text: "worktree" }));

      row.appendChild(box);
      row.appendChild(code);
      row.appendChild(name);
      row.appendChild(flags);
      row.appendChild(rowActions(file));
      row.addEventListener("click", function () {
        openDiff(file.path);
      });
      list.appendChild(row);
    });
    updateControls();
  }

  function toggleAll() {
    var rows = visible();
    var all = rows.length > 0 && rows.every(function (file) {
      return selected.has(file.path);
    });
    if (all) {
      rows.forEach(function (file) {
        selected.delete(file.path);
      });
    } else {
      rows.forEach(function (file) {
        selected.add(file.path);
      });
    }
    anchor = -1;
    render();
  }

  function stagePaths(paths) {
    if (!paths.length) return;
    NTC.mutate("/api/stage", { paths: paths }, "Staged " + paths.length + " file(s).");
  }

  function unstagePaths(paths) {
    if (!paths.length) return;
    NTC.mutate("/api/unstage", { paths: paths }, "Unstaged " + paths.length + " file(s).");
  }

  function discardPaths(paths) {
    if (!paths.length) return;
    NTC.confirm(
      "Discard unstaged changes in " +
        paths.length +
        " file(s)? This runs git restore -- and cannot be undone. Staged changes are kept.",
      "Discard"
    ).then(function (ok) {
      if (!ok) return;
      NTC.mutate("/api/discard", { paths: paths }, "Discarded changes in " + paths.length + " file(s).");
    });
  }

  function commit() {
    var text = message.value.trim();
    if (!text) {
      status.set("Enter a commit message.", "err");
      return;
    }
    commitButton.disabled = true;
    NTC.post("/api/commit", { message: text })
      .then(function (data) {
        NTC.command(data.command, true);
        message.value = "";
        status.set(data.summary || "Commit created.", "ok");
        NTC.emit("git:changed");
      })
      .catch(function (err) {
        status.set(err.message, "err");
        NTC.toast(err.message, "err");
      })
      .then(function () {
        updateControls();
      });
  }

  function openDiff(path) {
    activePath = path;
    render();
    var file = files().filter(function (item) {
      return item.path === path;
    })[0];
    NTC.api("/api/diff?path=" + encodeURIComponent(path))
      .then(function (data) {
        var nodes = [];
        nodes.push(
          el(
            "div",
            { class: "row", style: "margin:0 0 8px" },
            el("span", { class: "code " + (file ? codeClass(file) : ""), text: file ? file.code : "M" }),
            el("span", { class: "mono", text: data.path }),
            el("span", { class: "spacer" }),
            file && file.untracked
              ? el("button", {
                  class: "mini",
                  text: "Stage",
                  onclick: function () {
                    stagePaths([path]);
                  }
                })
              : el("span")
          )
        );
        nodes.push(NTC.diff.section("Staged", data.staged ? "" : "no staged diff"));
        var staged = el("div");
        NTC.diff.render(staged, data.staged, { emptyText: "(nothing staged for this file)" });
        nodes.push(staged);

        nodes.push(NTC.diff.section("Unstaged", data.worktree ? "" : "no unstaged diff"));
        var worktree = el("div");
        NTC.diff.render(worktree, data.worktree, { emptyText: "(no unstaged changes)" });
        nodes.push(worktree);

        var note = "";
        if (data.untracked) note = "Untracked file — stage it to see its diff.";
        else if (data.conflict) note = "Conflicted file — resolve it in Actions, then stage the result.";
        else if (data.truncated) note = "Diff truncated (too large to show in full).";
        if (note) nodes.push(el("div", { class: "muted", style: "margin-top:10px", text: note }));

        NTC.inspect.show(data.path, nodes);
      })
      .catch(function (err) {
        status.set(err.message, "err");
      });
  }

  function focusFilter() {
    search.focus();
    search.select();
  }

  function onKey(e) {
    if (store.ui.tab !== "changes") return;
    var target = e.target || {};
    var tag = (target.tagName || "").toLowerCase();
    if (tag === "textarea" || tag === "select" || (tag === "input" && target.type !== "checkbox")) return;
    if (target.isContentEditable) return;

    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "a") {
      if (!visible().length) return;
      e.preventDefault();
      toggleAll();
      return;
    }
    if (e.ctrlKey || e.metaKey || e.altKey) return;

    var rows = visible();
    if (e.key === "j" || e.key === "ArrowDown") {
      if (!rows.length) return;
      e.preventDefault();
      var down = Math.min(rows.length - 1, indexOfActive(rows) + 1);
      openDiff(rows[down].path);
    } else if (e.key === "k" || e.key === "ArrowUp") {
      if (!rows.length) return;
      e.preventDefault();
      var up = Math.max(0, indexOfActive(rows) - 1);
      openDiff(rows[up].path);
    } else if (e.key === " ") {
      if (!activePath) return;
      e.preventDefault();
      if (selected.has(activePath)) selected.delete(activePath);
      else selected.add(activePath);
      render();
    }
  }

  function indexOfActive(rows) {
    for (var i = 0; i < rows.length; i++) {
      if (rows[i].path === activePath) return i;
    }
    return -1;
  }

  function mount(root) {
    MODES.forEach(function (item) {
      var button = el("button", {
        text: item.label,
        onclick: function () {
          mode = item.id;
          Object.keys(modeButtons).forEach(function (id) {
            modeButtons[id].classList.toggle("on", id === mode);
          });
          render();
        }
      });
      if (item.id === mode) button.classList.add("on");
      modeButtons[item.id] = button;
      seg.appendChild(button);
    });

    var head = el(
      "div",
      { class: "row", style: "margin:0 0 10px" },
      search,
      seg,
      selectAllButton
    );
    panel.head.appendChild(stagedTag);
    panel.body.appendChild(head);
    panel.body.appendChild(list);
    panel.body.appendChild(ui.row(stageButton, unstageButton, discardButton));

    var footer = el(
      "div",
      { class: "commit-bar" },
      el(
        "div",
        { class: "row", style: "margin:0 0 8px" },
        el("span", { class: "muted", text: "Commit staged changes" }),
        el("span", { class: "spacer" }),
        el("span", { class: "hint", text: "Ctrl+Enter to commit" })
      ),
      message,
      ui.row(
        commitButton,
        ui.button(
          "Push",
          function () {
            NTC.go("remotes");
          },
          null,
          "Go to Remotes to push or publish"
        ),
        el("span", { class: "spacer" }),
        el("span", { class: "hint", text: store.repo.upstream ? "upstream " + store.repo.upstream : "no upstream yet" })
      ),
      status.el
    );

    root.appendChild(panel.root);
    root.appendChild(footer);

    search.addEventListener("input", function () {
      query = search.value.trim().toLowerCase();
      render();
    });
    message.addEventListener("input", updateControls);
    message.addEventListener("keydown", function (e) {
      if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
        e.preventDefault();
        if (!commitButton.disabled) commit();
      }
    });
    document.addEventListener("keydown", onKey);
  }

  NTC.tabs.register({
    id: "changes",
    label: "Changes",
    icon: "±",
    hint: "Status, staging and commits",
    mount: mount,
    refresh: function () {
      render();
      return Promise.resolve();
    },
    activate: function (args) {
      if (args && args[0] === "file" && args[1]) openDiff(args[1]);
    },
    focusFilter: focusFilter,
    badge: function () {
      var conflicts = store.status.conflicts.length;
      if (conflicts) return { value: conflicts, kind: "err" };
      var count = files().length;
      return count ? { value: count, kind: "warn" } : 0;
    }
  });
})(window);
