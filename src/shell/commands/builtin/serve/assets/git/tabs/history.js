/* NTC Git — History tab.
 *
 * The last 30 commits (the server's LOG_LIMIT) with the selected commit's
 * file list in the inspector. Filters and paging need /api/log parameters,
 * which is the next backend step; this tab is deliberately read-only.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var list = el("div", { class: "list tall" });
  var panel = ui.panel("Recent commits");
  var activeHash = "";

  function statusClass(status) {
    if (!status) return "";
    var kind = status.charAt(0);
    if (kind === "A") return "add";
    if (kind === "D") return "del";
    if (kind === "R" || kind === "C") return "copy";
    if (kind === "U") return "conflict";
    return "";
  }

  function markActive(hash) {
    var rows = list.querySelectorAll(".crow");
    for (var i = 0; i < rows.length; i++) {
      rows[i].classList.toggle("active", rows[i].dataset.hash === hash);
    }
  }

  function openCommit(hash) {
    activeHash = hash;
    markActive(hash);
    NTC.inspect.show("Loading " + hash + "…", el("div", { class: "inspector-empty", text: "…" }));
    NTC.api("/api/commit?hash=" + encodeURIComponent(hash))
      .then(function (data) {
        if (activeHash !== hash) return;
        var nodes = [];
        nodes.push(
          el(
            "div",
            { class: "row", style: "margin:0 0 10px" },
            el("span", { class: "mono", text: data.hash }),
            el("span", { class: "spacer" }),
            ui.button(
              "Copy hash",
              function () {
                NTC.copy(data.hash);
              },
              "mini"
            )
          )
        );
        nodes.push(el("div", { class: "muted", text: data.author + " · " + data.when }));
        nodes.push(
          el("div", { class: "mono", style: "margin:8px 0 12px;white-space:pre-wrap", text: data.summary })
        );

        nodes.push(NTC.diff.section("Files", data.files.length + " changed"));
        var box = el("div", { class: "list", style: "max-height:none" });
        if (!data.files.length) {
          box.appendChild(ui.empty("No files reported for this commit."));
        }
        data.files.forEach(function (file) {
          box.appendChild(
            el(
              "div",
              { class: "entry" },
              el("span", { class: "code wide " + statusClass(file.status), text: file.status }),
              el("span", {
                class: "mono",
                style: "flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap",
                text: file.old ? file.old + " → " + file.path : file.path,
                title: file.old ? file.old + " → " + file.path : file.path
              })
            )
          );
        });
        nodes.push(box);
        if (data.truncated) {
          nodes.push(el("div", { class: "muted", style: "margin-top:8px", text: "… more files not shown" }));
        }

        NTC.inspect.show(data.hash.slice(0, 12), nodes);
      })
      .catch(function (err) {
        NTC.inspect.show(hash, el("div", { class: "inspector-empty", text: err.message }));
      });
  }

  function render() {
    var commits = store.log.commits || [];
    NTC.clear(list);
    if (!commits.length) {
      list.appendChild(
        ui.empty(
          store.repo.branch === null && !store.status.files.length
            ? "No commits yet — stage something and commit it."
            : "No commits yet."
        )
      );
      return;
    }
    commits.forEach(function (commit) {
      var row = el(
        "div",
        {
          class: "crow" + (commit.hash === activeHash ? " active" : ""),
          dataset: { hash: commit.hash },
          title: "Show the files changed by " + commit.hash,
          tabindex: "0"
        },
        el("span", { class: "hash", text: commit.hash }),
        el("span", { class: "summary", text: commit.summary, title: commit.summary }),
        el("span", { class: "meta", text: commit.author + " · " + commit.when })
      );
      row.addEventListener("click", function () {
        openCommit(commit.hash);
      });
      row.addEventListener("keydown", function (e) {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          openCommit(commit.hash);
        }
      });
      list.appendChild(row);
    });
  }

  function refresh() {
    return NTC.api("/api/log")
      .then(function (data) {
        store.log.commits = data.commits || [];
        store.log.loaded = true;
        panel.count(
          store.log.commits.length
            ? "newest " + store.log.commits.length
            : "empty"
        );
        render();
      })
      .catch(function (err) {
        NTC.clear(list);
        list.appendChild(ui.empty(err.message));
      });
  }

  function mount(root) {
    panel.body.appendChild(list);
    panel.body.appendChild(
      el("div", {
        class: "muted",
        style: "margin-top:8px",
        text: "Click a commit to list the files it changed. Paging and patch view come with the extended log endpoint."
      })
    );
    root.appendChild(panel.root);
  }

  NTC.tabs.register({
    id: "history",
    label: "History",
    icon: "≣",
    hint: "Recent commits",
    mount: mount,
    refresh: refresh,
    activate: function (args) {
      if (args && args[0]) openCommit(args[0]);
    },
    badge: function () {
      return store.log.loaded ? store.log.commits.length : 0;
    }
  });
})(window);
