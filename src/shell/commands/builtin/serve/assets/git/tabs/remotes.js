/* NTC Git — Remotes tab.
 *
 * One place for the network story: pick a remote, fetch, preview incoming
 * commits, pull --ff-only, push, or publish a branch that has no upstream.
 * Remote URL management waits for the /api/remotes endpoints.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var panel = ui.panel("Sync");
  var status = ui.status();
  var remoteSelect = ui.select();
  var incomingInfo = el("div", { class: "muted", text: "" });
  var incomingList = el("div", { class: "list", style: "max-height:34vh" });
  var upstreamTag = el("span", { class: "count", text: "no upstream" });
  var syncLine = el("div", { class: "muted", text: "Checking upstream…" });

  var fetchButton = ui.button("Fetch", doFetch, "primary");
  var pullButton = ui.button("Pull", doPull, null, "git pull --ff-only");
  var pushButton = ui.button("Push", doPush, null, "git push");
  var publishButton = ui.button("Publish branch", doPublish, null, "git push -u <remote> <branch>");

  function remotes() {
    return store.repo.remotes || [];
  }

  function selectedRemote() {
    return remoteSelect.value || "";
  }

  function renderSync() {
    var repo = store.repo;
    upstreamTag.textContent = repo.upstream || "no upstream";
    NTC.clear(remoteSelect);
    remotes().forEach(function (name) {
      remoteSelect.appendChild(ui.option(name));
    });
    var preferred = repo.upstream ? repo.upstream.split("/")[0] : "";
    if (preferred && remotes().indexOf(preferred) >= 0) remoteSelect.value = preferred;
    else if (remotes().length) remoteSelect.value = remotes()[0];
    remoteSelect.disabled = !remotes().length;

    if (!repo.branch) {
      syncLine.textContent = "Detached HEAD — check out a branch to fetch, pull or push.";
    } else if (!remotes().length) {
      syncLine.textContent = "No remotes configured. Add one in the terminal: git remote add origin <url>";
    } else if (!repo.upstream) {
      syncLine.textContent =
        "No upstream. Publish with: git push -u " + selectedRemote() + " " + repo.branch;
    } else {
      syncLine.textContent =
        repo.upstream + " · " + repo.ahead + " ahead, " + repo.behind + " behind";
    }

    var remote = selectedRemote();
    fetchButton.disabled = !remote;
    publishButton.disabled = !remote || !repo.branch || !!repo.upstream;
    pushButton.disabled = !repo.upstream;
    pullButton.disabled = !repo.upstream;
  }

  function renderIncoming(data) {
    NTC.clear(incomingList);
    if (!data) {
      incomingInfo.textContent = "";
      return;
    }
    if (data.missing) {
      incomingInfo.textContent = "No " + data.target + " on the remote yet — publish this branch first.";
      return;
    }
    if (!data.count) {
      incomingInfo.textContent = "Up to date with " + data.target + ".";
      return;
    }
    incomingInfo.textContent =
      data.count + (data.count === 1 ? " incoming commit" : " incoming commits") + " from " + data.target;
    data.incoming.forEach(function (commit) {
      incomingList.appendChild(
        el(
          "div",
          { class: "crow static" },
          el("span", { class: "hash", text: commit.hash }),
          el("span", { class: "summary", text: commit.summary, title: commit.summary }),
          el("span", { class: "meta", text: commit.author + " · " + commit.when })
        )
      );
    });
    if (data.truncated) {
      incomingList.appendChild(
        ui.empty("Showing the newest " + data.incoming.length + " commits.")
      );
    }
  }

  function loadIncoming() {
    var remote = selectedRemote();
    var repo = store.repo;
    if (!remote || !repo.branch) {
      renderIncoming(null);
      return Promise.resolve();
    }
    return NTC.api("/api/incoming?remote=" + encodeURIComponent(remote))
      .then(renderIncoming)
      .catch(function (err) {
        incomingInfo.textContent = err.message;
      });
  }

  function afterNetwork(message) {
    status.set(message, "ok");
    NTC.emit("git:changed");
    return loadIncoming();
  }

  function doFetch() {
    var remote = selectedRemote();
    if (!remote) return;
    status.set("Fetching " + remote + "…");
    NTC.mutate("/api/fetch", { remote: remote }, "Fetched " + remote + ".", ["status", "log"]).then(
      function (data) {
        if (data) afterNetwork("Fetched " + remote + ".");
      }
    );
  }

  function doPull() {
    status.set("Pulling…");
    NTC.mutate("/api/pull", {}, "Pulled.", ["status", "log"]).then(function (data) {
      if (data) afterNetwork("Pulled with --ff-only.");
    });
  }

  function doPush() {
    status.set("Pushing…");
    NTC.mutate("/api/push", {}, "Pushed.", ["status", "log"]).then(function (data) {
      if (data) afterNetwork("Pushed.");
    });
  }

  function doPublish() {
    var remote = selectedRemote();
    if (!remote) {
      status.set("Choose a remote first.", "err");
      return;
    }
    NTC.mutate(
      "/api/publish",
      { remote: remote },
      "Published " + store.repo.branch + " to " + remote + ".",
      ["status", "log"]
    ).then(function (data) {
      if (data) afterNetwork("Published " + store.repo.branch + " to " + remote + ".");
    });
  }

  function mount(root) {
    panel.head.appendChild(upstreamTag);
    panel.body.appendChild(syncLine);
    panel.body.appendChild(
      ui.row(el("span", { class: "muted", text: "Remote" }), remoteSelect)
    );
    panel.body.appendChild(ui.row(fetchButton, pullButton, pushButton, publishButton));
    panel.body.appendChild(
      el("div", {
        class: "muted",
        style: "margin-top:8px",
        text: "Pull uses --ff-only, so it never creates an unresolvable merge commit."
      })
    );
    panel.body.appendChild(status.el);

    var incomingPanel = ui.panel("Incoming");
    incomingPanel.body.appendChild(incomingInfo);
    incomingPanel.body.appendChild(incomingList);
    incomingPanel.body.appendChild(
      ui.row(
        ui.button("Reload", loadIncoming, "mini"),
        el("span", { class: "muted", text: "Remote-tracking refs only move on fetch." })
      )
    );

    var notes = ui.panel("Remote URLs");
    notes.body.appendChild(
      el("div", {
        class: "muted",
        text: "Adding, renaming and re-pointing remotes is not exposed yet. Use git remote -v / git remote add / git remote set-url in the terminal; the page picks the change up on the next refresh."
      })
    );

    root.appendChild(panel.root);
    root.appendChild(incomingPanel.root);
    root.appendChild(notes.root);

    remoteSelect.addEventListener("change", function () {
      renderSync();
      loadIncoming();
    });
  }

  function refresh() {
    renderSync();
    return loadIncoming();
  }

  NTC.tabs.register({
    id: "remotes",
    label: "Remotes",
    icon: "⇅",
    hint: "Fetch, pull, push and publish",
    mount: mount,
    refresh: refresh,
    badge: function () {
      var repo = store.repo;
      var count = repo.ahead + repo.behind;
      return count ? { value: count, kind: "accent" } : 0;
    }
  });
})(window);
