/* NTC Git — Actions tab.
 *
 * Rare and destructive verbs, deliberately away from Commit. Today: the
 * in-progress sequencer state and the conflict resolver (also reachable from
 * the global banner). Reset / revert / cherry-pick / clean need endpoints
 * that do not exist yet, so they are documented, not faked.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var progressPanel = ui.panel("In progress");
  var progressBody = el("div");
  var status = ui.status();
  var conflictPanel = ui.panel("Conflicts");
  var conflictList = el("div", { class: "list" });
  var conflictHelp = el("div", {
    class: "muted",
    text: "Take one side of a file, or edit it yourself in your editor, then mark it resolved."
  });

  function conflictAction(path, side) {
    NTC.mutate(
      "/api/conflict/resolve",
      { path: path, side: side },
      "Took " + side + " for " + path + ".",
      ["status", "log"]
    );
  }

  function markResolved(path) {
    NTC.mutate("/api/stage", { paths: [path] }, "Marked resolved: " + path + ".", ["status", "log"]);
  }

  function viewDiff(path) {
    NTC.api("/api/diff?path=" + encodeURIComponent(path))
      .then(function (data) {
        var nodes = [
          el("div", { class: "row", style: "margin:0 0 8px" },
            el("span", { class: "code conflict", text: "U" }),
            el("span", { class: "mono", text: data.path })),
          NTC.diff.section("Conflict diff", data.truncated ? "truncated" : "")
        ];
        var box = el("div");
        NTC.diff.render(box, data.worktree || data.staged, { emptyText: "(no diff available)" });
        nodes.push(box);
        nodes.push(
          el("div", {
            class: "muted",
            style: "margin-top:10px",
            text: "Resolve by taking a whole side below, or edit the file and stage it."
          })
        );
        NTC.inspect.show(data.path, nodes);
      })
      .catch(function (err) {
        status.set(err.message, "err");
      });
  }

  function renderProgress() {
    var repo = store.repo;
    NTC.clear(progressBody);
    var busy = repo.merging || repo.rebasing || store.status.conflicts.length > 0;
    if (!busy) {
      progressBody.appendChild(
        el("div", { class: "muted", text: "Nothing in progress — no merge, rebase, cherry-pick or revert state." })
      );
      return;
    }
    if (repo.rebasing) {
      progressBody.appendChild(el("div", { class: "mono", text: "rebase in progress" }));
      progressBody.appendChild(
        ui.row(
          ui.button(
            "Continue rebase",
            function () {
              NTC.mutate("/api/branch/rebase/continue", {}, "Rebase continued.", ["status", "log", "branches"]);
            },
            "primary",
            "git rebase --continue (needs every conflict staged)"
          ),
          el("span", { class: "muted", text: "Every conflict must be staged first." })
        )
      );
    }
    if (repo.merging) {
      progressBody.appendChild(el("div", { class: "mono", text: "merge in progress" }));
      progressBody.appendChild(
        ui.row(
          ui.button("Commit the merge", function () {
            NTC.go("changes");
          }, "primary"),
          el("span", { class: "muted", text: "Commit from Changes once every conflict is resolved." })
        )
      );
    }
    if (!repo.merging && !repo.rebasing && store.status.conflicts.length) {
      progressBody.appendChild(
        el("div", { class: "muted", text: "Conflicts left over from an earlier operation; resolve and stage them." })
      );
    }
  }

  function renderConflicts() {
    var conflicts = store.status.conflicts || [];
    NTC.clear(conflictList);
    conflictPanel.count(conflicts.length ? conflicts.length + " file(s)" : "");
    if (!conflicts.length) {
      conflictList.appendChild(ui.empty("No conflicts."));
      conflictHelp.textContent =
        "Take one side of a file, or edit it yourself in your editor, then mark it resolved.";
      return;
    }
    conflictHelp.textContent =
      "Taking a side runs git checkout --ours/--theirs and stages the result. Prefer editing the file when both sides matter.";
    conflicts.forEach(function (path) {
      var row = el(
        "div",
        { class: "entry" },
        el("span", { class: "code conflict", text: "U" }),
        el("button", {
          class: "name",
          text: path,
          title: "Show the conflict in " + path,
          onclick: function () {
            viewDiff(path);
          }
        }),
        el(
          "span",
          { class: "actions", style: "opacity:1" },
          el("button", { text: "Ours", title: "git checkout --ours -- " + path, onclick: function () { conflictAction(path, "ours"); } }),
          el("button", { text: "Theirs", title: "git checkout --theirs -- " + path, onclick: function () { conflictAction(path, "theirs"); } }),
          el("button", { text: "Resolved", title: "git add -- " + path, onclick: function () { markResolved(path); } })
        )
      );
      conflictList.appendChild(row);
    });
  }

  function mount(root) {
    progressPanel.body.appendChild(progressBody);
    progressPanel.body.appendChild(status.el);

    conflictPanel.body.appendChild(conflictHelp);
    conflictPanel.body.appendChild(conflictList);

    var later = ui.panel("Not exposed yet");
    later.body.appendChild(
      el("div", {
        class: "muted",
        text:
          "Reset, revert, cherry-pick, clean and remote URL management are terminal-only for now: the page will not invent a command it cannot show you first. They arrive with the destructive-verb endpoints, behind a typed confirmation."
      })
    );

    root.appendChild(progressPanel.root);
    root.appendChild(conflictPanel.root);
    root.appendChild(later.root);
  }

  function refresh() {
    renderProgress();
    renderConflicts();
    return Promise.resolve();
  }

  NTC.tabs.register({
    id: "actions",
    label: "Actions",
    icon: "⚡",
    hint: "In-progress state and conflicts",
    mount: mount,
    refresh: refresh,
    badge: function () {
      var conflicts = store.status.conflicts.length;
      if (conflicts) return { value: conflicts, kind: "err" };
      return store.repo.merging || store.repo.rebasing ? { value: "!", kind: "warn" } : 0;
    }
  });
})(window);
