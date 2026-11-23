/* NTC Git — Repository tab.
 *
 * Read-mostly landing page: repository facts from /api/status, quick actions,
 * and the configured remote names. Remote URL management arrives with the
 * /api/remotes endpoints; until then the tab explains the terminal command.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var facts = el("div", { class: "facts" });
  var remotes = ui.list();
  var branchInput = ui.input("new branch name");
  var status = ui.status();

  function factGrid() {
    var repo = store.repo;
    var files = store.status.files.length;
    var conflicts = store.status.conflicts.length;
    var children = [
      ui.fact("Repository", repo.root || "—"),
      ui.fact("HEAD", repo.branch || "detached"),
      ui.fact("Upstream", repo.upstream || "none"),
      ui.fact("Ahead / behind", "↑" + repo.ahead + "  ↓" + repo.behind),
      ui.fact("Working tree", conflicts ? conflicts + " conflicted" : files ? files + " changed" : "clean"),
      ui.fact("Staged", String(store.status.staged)),
      ui.fact("Remotes", repo.remotes.length ? repo.remotes.join(", ") : "none"),
      ui.fact(
        "State",
        repo.rebasing ? "rebasing" : repo.merging ? "merging" : "idle"
      )
    ];
    NTC.clear(facts);
    children.forEach(function (child) {
      facts.appendChild(child);
    });
  }

  function renderRemotes() {
    NTC.clear(remotes);
    var names = store.repo.remotes || [];
    if (!names.length) {
      remotes.appendChild(
        ui.empty("No remotes. Add one in the terminal: git remote add origin <url>")
      );
      return;
    }
    names.forEach(function (name) {
      remotes.appendChild(
        el(
          "div",
          { class: "entry" },
          el("span", { class: "code copy", text: "→" }),
          el("button", {
            class: "name",
            text: name,
            onclick: function () {
              NTC.copy(name);
            },
            title: "Copy the remote name"
          }),
          el(
            "span",
            { class: "flags" },
            el("span", {
              class: "flag",
              text: name === (store.repo.upstream || "").split("/")[0] ? "tracked" : "remote"
            })
          )
        )
      );
    });
  }

  function createBranch() {
    var name = branchInput.value.trim();
    if (!name) {
      status.set("Enter a branch name.", "err");
      return;
    }
    NTC.mutate("/api/branch/create", { name: name }, "Created and switched to " + name + ".", [
      "status",
      "log",
      "branches"
    ]).then(function (data) {
      if (data) branchInput.value = "";
    });
  }

  function fetchAll() {
    var names = store.repo.remotes || [];
    if (!names.length) {
      status.set("No remotes configured.", "err");
      return;
    }
    status.set("Fetching " + names[0] + "…");
    NTC.mutate("/api/fetch", { remote: names[0] }, "Fetched " + names[0] + ".", ["status", "log"])
      .then(function (data) {
        if (data) status.set("Fetched " + names[0] + ".", "ok");
      });
  }

  function mount(root) {
    var overview = ui.panel("Overview");
    overview.body.appendChild(facts);

    branchInput.classList.add("grow");
    var quick = ui.panel("Quick actions");
    quick.body.appendChild(
      ui.row(
        ui.button(
          "Refresh status",
          function () {
            NTC.reload().then(function () {
              status.set("Status reloaded.", "ok");
            });
          },
          "primary"
        ),
        ui.button("Fetch", fetchAll, null, "git fetch <first remote>"),
        ui.button("Open Changes", function () {
          NTC.go("changes");
        }),
        ui.button("Open History", function () {
          NTC.go("history");
        })
      )
    );
    quick.body.appendChild(
      ui.row(
        el("span", { class: "muted", text: "New branch from HEAD:" }),
        branchInput,
        ui.button("Create & switch", createBranch)
      )
    );
    branchInput.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        e.preventDefault();
        createBranch();
      }
    });
    quick.body.appendChild(status.el);

    var remotePanel = ui.panel("Remotes");
    remotePanel.body.appendChild(remotes);
    remotePanel.body.appendChild(
      el("div", {
        class: "muted",
        style: "margin-top:8px",
        text: "URLs and remote CRUD are not in this page yet; use git remote -v / git remote set-url in the terminal."
      })
    );

    var about = ui.panel("About this page");
    about.body.appendChild(
      el(
        "div",
        { class: "muted" },
        "Every button runs a real git command in ",
        el("code", { class: "mono", text: store.repo.root || "this repository" }),
        ". The bar at the bottom echoes the last command. ",
        el("a", { href: "/about", target: "_blank", rel: "noopener" }, "Read the rules"),
        "."
      )
    );

    root.appendChild(overview.root);
    root.appendChild(quick.root);
    root.appendChild(remotePanel.root);
    root.appendChild(about.root);
  }

  function refresh() {
    factGrid();
    renderRemotes();
    return Promise.resolve();
  }

  NTC.tabs.register({
    id: "repository",
    label: "Repository",
    icon: "◈",
    hint: "Repository facts and quick actions",
    mount: mount,
    refresh: refresh
  });
})(window);
