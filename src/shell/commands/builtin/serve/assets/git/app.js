/* NTC Git — application shell.
 *
 * Plain ES5, no build step. Owns the store, the hash router, the rail, the
 * inspector, toasts and the command bar; every tab is a module that calls
 * NTC.tabs.register().
 *
 * Rules kept from the single-page version: the client never invents a git
 * command — mutations echo the one the server actually ran.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC || (global.NTC = {});
  var TAB_KEY = "ntc.git.tab";
  var THEME_KEY = "ntc.git.theme";
  var AUTO_KEY = "ntc.git.auto";

  /* ── DOM helpers ───────────────────────────────────────────────────── */

  function append(node, children) {
    for (var i = 0; i < children.length; i++) {
      var child = children[i];
      if (child === null || child === undefined || child === false || child === "") continue;
      if (Array.isArray(child)) {
        append(node, child);
      } else if (child instanceof Node) {
        node.appendChild(child);
      } else {
        node.appendChild(document.createTextNode(String(child)));
      }
    }
  }

  function el(tag, props) {
    var node = document.createElement(tag);
    if (props) {
      for (var key in props) {
        if (!Object.prototype.hasOwnProperty.call(props, key)) continue;
        var value = props[key];
        if (value === null || value === undefined || value === false) continue;
        if (key === "class") node.className = value;
        else if (key === "text") node.textContent = value;
        else if (key === "dataset") {
          for (var name in value) node.dataset[name] = value[name];
        } else if (key.slice(0, 2) === "on" && typeof value === "function") {
          node.addEventListener(key.slice(2).toLowerCase(), value);
        } else if (value === true) {
          node.setAttribute(key, "");
        } else {
          node.setAttribute(key, value);
        }
      }
    }
    append(node, Array.prototype.slice.call(arguments, 2));
    return node;
  }

  function clear(node) {
    while (node.firstChild) node.removeChild(node.firstChild);
    return node;
  }

  function setStatus(node, message, kind) {
    if (!node) return;
    node.textContent = message || "";
    node.className = "status" + (kind ? " " + kind : "");
  }

  NTC.el = el;
  NTC.clear = clear;
  NTC.setStatus = setStatus;

  /* ── store ─────────────────────────────────────────────────────────── */

  var store = (NTC.store = {
    repo: {
      root: "",
      branch: null,
      upstream: "",
      ahead: 0,
      behind: 0,
      clean: true,
      merging: false,
      rebasing: false,
      remotes: []
    },
    status: { files: [], conflicts: [], staged: 0 },
    branches: { current: "", detached: false, list: [] },
    log: { commits: [], loaded: false },
    stash: { list: [], loaded: false },
    tags: { list: [], loaded: false },
    ui: { tab: "", theme: "dark", auto: false }
  });

  /* ── events ────────────────────────────────────────────────────────── */

  var listeners = {};

  NTC.on = function (name, fn) {
    (listeners[name] || (listeners[name] = [])).push(fn);
  };

  NTC.emit = function (name, payload) {
    var list = listeners[name] || [];
    for (var i = 0; i < list.length; i++) {
      try {
        list[i](payload);
      } catch (err) {
        if (global.console) console.error(err);
      }
    }
  };

  /* ── api ───────────────────────────────────────────────────────────── */

  function api(url, opts) {
    return fetch(url, opts || {}).then(
      function (res) {
        return res
          .json()
          .catch(function () {
            return { error: res.statusText || "HTTP " + res.status };
          })
          .then(function (data) {
            if (!res.ok) {
              var err = new Error(data.error || res.statusText || "HTTP " + res.status);
              err.data = data;
              throw err;
            }
            return data;
          });
      },
      function (err) {
        throw new Error("Cannot reach the server: " + err.message);
      }
    );
  }

  function post(url, payload) {
    return api(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload || {})
    });
  }

  NTC.api = api;
  NTC.post = post;

  /* Mutating request: echoes the command, toasts the outcome, and asks the
   * shell to reload. Resolves to the payload, or null after an error (the
   * error is already on screen, so callers do not need their own catch). */
  NTC.mutate = function (url, payload, okMessage, hints) {
    return post(url, payload)
      .then(function (data) {
        command(data.command, true);
        if (okMessage) {
          toast(typeof okMessage === "function" ? okMessage(data) : okMessage, "ok");
        }
        NTC.emit("git:changed", hints || null);
        return data;
      })
      .catch(function (err) {
        toast(err.message, "err");
        return null;
      });
  };

  /* ── toasts ────────────────────────────────────────────────────────── */

  function toast(message, kind) {
    var box = document.getElementById("toasts");
    if (!box || !message) return;
    var node = el("div", { class: "toast " + (kind || ""), text: message });
    node.addEventListener("click", function () {
      node.remove();
    });
    box.appendChild(node);
    while (box.children.length > 4) box.removeChild(box.firstChild);
    global.setTimeout(function () {
      node.remove();
    }, kind === "err" ? 8000 : 4200);
  }

  NTC.toast = toast;

  /* ── command bar ───────────────────────────────────────────────────── */

  var history = [];
  var lastCommand = "";

  function command(text, ok) {
    if (!text) return;
    lastCommand = text;
    var code = document.getElementById("command-text");
    if (code) {
      code.textContent = text;
      code.className = ok === false ? "failed" : ok === true ? "" : "";
    }
    history.unshift({
      text: text,
      ok: ok !== false,
      when: new Date().toLocaleTimeString()
    });
    if (history.length > 60) history.pop();
    renderHistory();
  }

  NTC.command = command;

  function renderHistory() {
    var box = document.getElementById("command-history-list");
    if (!box || box.classList.contains("hidden")) return;
    clear(box);
    if (!history.length) {
      box.appendChild(el("div", { class: "empty", text: "Nothing run yet." }));
      return;
    }
    history.forEach(function (entry) {
      box.appendChild(
        el(
          "div",
          {
            class: "cmd-line " + (entry.ok ? "ok" : "err"),
            onclick: function () {
              command(entry.text, entry.ok);
            },
            title: "Copy this command into the bar"
          },
          el("span", { class: "when", text: entry.when }),
          el("span", { class: "what", text: entry.text })
        )
      );
    });
  }

  function copyText(value) {
    if (!value) return;
    var done = function () {
      toast("Copied.", "ok");
    };
    if (global.navigator && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(value).then(done, fallback);
      return;
    }
    fallback();

    function fallback() {
      var area = el("textarea", { style: "position:fixed;left:-9999px;top:0" });
      area.value = value;
      document.body.appendChild(area);
      area.select();
      try {
        document.execCommand("copy");
        done();
      } catch (err) {
        toast("Copy failed — select the text manually.", "warn");
      }
      area.remove();
    }
  }

  NTC.copy = copyText;

  /* ── confirm dialog ────────────────────────────────────────────────── */

  NTC.confirm = function (message, label) {
    return new Promise(function (resolve) {
      var settled = false;
      function finish(value) {
        if (settled) return;
        settled = true;
        resolve(value);
      }
      var dialog = el(
        "dialog",
        { class: "confirm" },
        el("p", {}, message),
        el(
          "div",
          { class: "row" },
          el(
            "button",
            {
              onclick: function () {
                finish(false);
                dialog.close();
              }
            },
            "Cancel"
          ),
          el(
            "button",
            {
              class: "primary danger",
              onclick: function () {
                finish(true);
                dialog.close();
              }
            },
            label || "Continue"
          )
        )
      );
      dialog.addEventListener("close", function () {
        finish(false);
        dialog.remove();
      });
      document.body.appendChild(dialog);
      if (dialog.showModal) dialog.showModal();
      else finish(global.confirm(message));
    });
  };

  /* ── inspector ─────────────────────────────────────────────────────── */

  var inspector = {
    title: null,
    body: null
  };

  function inspectShow(title, nodes) {
    if (!inspector.body) return;
    inspector.title.textContent = title || "Inspector";
    clear(inspector.body);
    append(inspector.body, Array.isArray(nodes) ? nodes : [nodes]);
    document.body.classList.add("inspector-open");
  }

  function inspectHide() {
    document.body.classList.remove("inspector-open");
  }

  NTC.inspect = {
    show: inspectShow,
    hide: inspectHide,
    get isOpen() {
      return document.body.classList.contains("inspector-open");
    }
  };

  /* ── tabs ──────────────────────────────────────────────────────────── */

  var tabs = [];
  var byId = {};
  var active = null;

  NTC.tabs = {
    register: function (def) {
      if (!def || !def.id || byId[def.id]) return;
      var root = el("section", { class: "tab", id: "tab-" + def.id });
      document.getElementById("content").appendChild(root);
      def.root = root;
      tabs.push(def);
      byId[def.id] = def;
      if (def.mount) def.mount(root);
    },
    get: function (id) {
      return byId[id];
    },
    list: function () {
      return tabs;
    }
  };

  function activeTab() {
    return active;
  }

  NTC.activeTab = activeTab;

  function buildRail() {
    var nav = document.getElementById("rail-nav");
    if (!nav) return;
    clear(nav);
    tabs.forEach(function (tab) {
      var badge = el("span", { class: "badge zero", text: "0" });
      var item = el(
        "button",
        {
          class: "rail-item",
          dataset: { tab: tab.id },
          onclick: function () {
            NTC.go(tab.id);
          },
          title: tab.hint || tab.label
        },
        el("span", { class: "icon", text: tab.icon || "•" }),
        el("span", { class: "label", text: tab.label }),
        badge
      );
      tab.badgeNode = badge;
      tab.railItem = item;
      nav.appendChild(item);
    });
  }

  function renderBadges() {
    tabs.forEach(function (tab) {
      if (!tab.badgeNode) return;
      var value = tab.badge ? tab.badge() : 0;
      var kind = "";
      if (value && typeof value === "object") {
        kind = value.kind || "";
        value = value.value;
      }
      var empty = !value;
      tab.badgeNode.textContent = empty ? "0" : String(value);
      tab.badgeNode.className = "badge" + (empty ? " zero" : "") + (kind ? " " + kind : "");
    });
  }

  NTC.renderBadges = renderBadges;

  function markRail(id) {
    tabs.forEach(function (tab) {
      if (tab.railItem) tab.railItem.classList.toggle("active", tab.id === id);
    });
  }

  /* ── header ────────────────────────────────────────────────────────── */

  function renderHeader() {
    var repo = store.repo;
    var path = document.getElementById("repo-path");
    if (path) {
      path.textContent = repo.root || "…";
      path.title = "Copy the repository path";
    }
    var branch = document.getElementById("rail-branch");
    if (branch) {
      branch.textContent = repo.branch || "detached HEAD";
      branch.classList.toggle("branch", !!repo.branch);
    }
    var ahead = document.getElementById("rail-ahead");
    if (ahead) {
      ahead.textContent = "↑" + repo.ahead;
      ahead.classList.toggle("hidden", !repo.ahead);
    }
    var behind = document.getElementById("rail-behind");
    if (behind) {
      behind.textContent = "↓" + repo.behind;
      behind.classList.toggle("hidden", !repo.behind);
    }
    var dot = document.getElementById("rail-dot");
    if (dot) {
      var dirty = !repo.clean;
      dot.className = "dot" + (store.status.conflicts.length ? " conflict" : dirty ? " dirty" : "");
      dot.title = store.status.conflicts.length
        ? store.status.conflicts.length + " conflicted file(s)"
        : dirty
          ? "Uncommitted changes"
          : "Working tree clean";
    }
    renderBadges();
  }

  /* ── global banner ─────────────────────────────────────────────────── */

  function renderBanner() {
    var content = document.getElementById("content");
    var banner = document.getElementById("banner");
    if (!content) return;
    var repo = store.repo;
    var conflicts = store.status.conflicts.length;
    var parts = [];
    var kind = "";
    if (repo.rebasing) {
      parts.push("Rebase in progress");
      kind = "warn";
    }
    if (repo.merging) {
      parts.push("Merge in progress");
      kind = "warn";
    }
    if (conflicts) {
      parts.push(conflicts + (conflicts === 1 ? " conflicted file" : " conflicted files"));
      kind = "err";
    }
    if (!parts.length) {
      if (banner) banner.remove();
      return;
    }
    if (!banner) {
      banner = el("div", { class: "banner", id: "banner" });
      content.insertBefore(banner, content.firstChild);
    }
    banner.className = "banner" + (kind === "err" ? " err" : "");
    clear(banner);
    banner.appendChild(el("span", { text: parts.join(" · ") }));
    banner.appendChild(el("span", { class: "spacer" }));
    banner.appendChild(
      el(
        "button",
        {
          onclick: function () {
            NTC.go(repo.merging || repo.rebasing ? "actions" : "changes");
          }
        },
        "Open " + (repo.merging || repo.rebasing ? "Actions" : "Changes")
      )
    );
  }

  /* ── status loader ─────────────────────────────────────────────────── */

  var statusSeq = 0;

  function loadStatus() {
    var token = ++statusSeq;
    return api("/api/status")
      .then(function (data) {
        if (token !== statusSeq) return data;
        var repo = store.repo;
        repo.root = data.root || "";
        repo.branch = data.branch || null;
        repo.upstream = data.upstream || "";
        repo.ahead = data.ahead || 0;
        repo.behind = data.behind || 0;
        repo.clean = !!data.clean;
        repo.merging = !!data.merging;
        repo.rebasing = !!data.rebasing;
        repo.remotes = data.remotes || [];
        store.status.files = data.files || [];
        store.status.conflicts = data.conflicts || [];
        store.status.staged = data.staged_count || 0;
        renderHeader();
        renderBanner();
        return data;
      })
      .catch(function (err) {
        toast(err.message, "err");
        return null;
      });
  }

  NTC.loadStatus = loadStatus;

  var reloadSeq = 0;

  function reload() {
    var token = ++reloadSeq;
    return loadStatus().then(function () {
      if (token !== reloadSeq) return null;
      var tab = activeTab();
      if (!tab || !tab.refresh) return null;
      return Promise.resolve(tab.refresh()).then(function (value) {
        renderBadges();
        return value;
      });
    });
  }

  NTC.reload = reload;

  /* ── router ────────────────────────────────────────────────────────── */

  function safeDecode(value) {
    try {
      return decodeURIComponent(value);
    } catch (err) {
      return value;
    }
  }

  function parseHash() {
    var raw = (global.location.hash || "").replace(/^#\/?/, "");
    if (!raw) return null;
    var parts = raw.split("/").filter(function (part) {
      return part.length > 0;
    });
    if (!parts.length) return null;
    return {
      id: safeDecode(parts[0]),
      args: parts.slice(1).map(safeDecode)
    };
  }

  function renderCrumbs(tab, args) {
    var crumbs = document.getElementById("crumbs");
    if (!crumbs) return;
    clear(crumbs);
    crumbs.appendChild(el("span", { class: "here", text: tab.label }));
    if (args && args.length) {
      crumbs.appendChild(el("span", { class: "arg", text: args.join(" / ") }));
    }
    if (tab.crumbs) {
      var extra = tab.crumbs(args);
      if (extra) crumbs.appendChild(extra);
    }
  }

  function activate(route) {
    var tab = byId[route.id];
    if (!tab) tab = byId.changes;
    if (!tab) return;
    if (active && active !== tab) active.root.classList.remove("active");
    tab.root.classList.add("active");
    active = tab;
    store.ui.tab = tab.id;
    try {
      localStorage.setItem(TAB_KEY, tab.id);
    } catch (err) {
      /* private mode */
    }
    markRail(tab.id);
    renderCrumbs(tab, route.args);
    // Always re-read status first: it feeds the header, the rail badges and
    // the banner, and every tab renders from the same store.
    reload().then(function () {
      if (tab.activate) tab.activate(route.args);
    });
  }

  NTC.go = function (id, args) {
    var hash =
      "#/" +
      id +
      (args && args.length
        ? "/" +
          args
            .map(function (arg) {
              return encodeURIComponent(arg);
            })
            .join("/")
        : "");
    if (global.location.hash === hash) {
      activate({ id: id, args: args || [] });
      return;
    }
    global.location.hash = hash;
  };

  /* ── chrome ────────────────────────────────────────────────────────── */

  function applyTheme(theme) {
    store.ui.theme = theme === "light" ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", store.ui.theme);
    try {
      localStorage.setItem(THEME_KEY, store.ui.theme);
    } catch (err) {
      /* private mode */
    }
  }

  function typingTarget(target) {
    if (!target) return false;
    var tag = (target.tagName || "").toLowerCase();
    if (tag === "textarea" || tag === "select") return true;
    if (tag === "input" && target.type !== "checkbox") return true;
    return !!target.isContentEditable;
  }

  function bindChrome() {
    var refresh = document.getElementById("refresh");
    if (refresh) {
      refresh.addEventListener("click", function () {
        reload();
      });
    }

    var auto = document.getElementById("auto-refresh");
    if (auto) {
      try {
        store.ui.auto = localStorage.getItem(AUTO_KEY) === "1";
      } catch (err) {
        store.ui.auto = false;
      }
      auto.checked = store.ui.auto;
      auto.addEventListener("change", function () {
        store.ui.auto = auto.checked;
        try {
          localStorage.setItem(AUTO_KEY, auto.checked ? "1" : "0");
        } catch (err) {
          /* private mode */
        }
        toast(auto.checked ? "Auto-refresh on (5 s)." : "Auto-refresh off.", "ok");
      });
    }

    var theme = document.getElementById("theme-toggle");
    if (theme) {
      theme.addEventListener("click", function () {
        applyTheme(store.ui.theme === "dark" ? "light" : "dark");
      });
    }

    var path = document.getElementById("repo-path");
    if (path) {
      path.addEventListener("click", function () {
        copyText(store.repo.root);
      });
    }

    var copy = document.getElementById("command-copy");
    if (copy) {
      copy.addEventListener("click", function () {
        copyText(lastCommand);
      });
    }

    var historyButton = document.getElementById("command-history");
    var historyList = document.getElementById("command-history-list");
    if (historyButton && historyList) {
      historyButton.addEventListener("click", function () {
        historyList.classList.toggle("hidden");
        renderHistory();
      });
    }

    var close = document.getElementById("inspector-close");
    if (close) close.addEventListener("click", inspectHide);

    var toggle = document.getElementById("inspector-toggle");
    if (toggle) {
      toggle.addEventListener("click", function () {
        document.body.classList.toggle("inspector-open");
      });
    }

    var scrim = document.getElementById("scrim");
    if (scrim) scrim.addEventListener("click", inspectHide);

    document.addEventListener("keydown", function (e) {
      if (e.key === "Escape") {
        if (inspectHide) inspectHide();
        if (historyList) historyList.classList.add("hidden");
        return;
      }
      if (e.ctrlKey || e.metaKey || e.altKey) return;
      if (typingTarget(e.target)) return;
      if (e.key === "/") {
        var tab = activeTab();
        if (tab && tab.focusFilter) {
          e.preventDefault();
          tab.focusFilter();
        }
        return;
      }
      if (e.key === "r" || e.key === "R") {
        e.preventDefault();
        reload();
        return;
      }
      var index = parseInt(e.key, 10);
      if (index >= 1 && index <= 9 && tabs[index - 1]) {
        e.preventDefault();
        NTC.go(tabs[index - 1].id);
      }
    });

    global.setInterval(function () {
      if (!store.ui.auto) return;
      if (document.hidden || typingTarget(document.activeElement)) return;
      if (document.querySelector("dialog[open]")) return;
      reload();
    }, 5000);

    NTC.on("git:changed", function () {
      reload();
    });
  }

  /* ── small shared UI builders ──────────────────────────────────────── */

  NTC.ui = {
    panel: function (title, extra) {
      var count = el("span", { class: "count" });
      var head = el("div", { class: "panel-head" }, el("h2", { text: title }), count);
      append(head, Array.isArray(extra) ? extra : extra ? [extra] : []);
      var body = el("div", { class: "body" });
      var root = el("section", { class: "panel" }, head, body);
      return {
        root: root,
        body: body,
        head: head,
        count: function (text) {
          count.textContent = text || "";
          count.classList.toggle("hidden", !text);
        }
      };
    },
    row: function () {
      var node = el("div", { class: "row" });
      append(node, Array.prototype.slice.call(arguments));
      return node;
    },
    status: function () {
      var node = el("div", { class: "status" });
      return {
        el: node,
        set: function (message, kind) {
          setStatus(node, message, kind);
        }
      };
    },
    empty: function (text) {
      return el("div", { class: "empty", text: text });
    },
    list: function () {
      return el("div", { class: "list" });
    },
    button: function (label, onClick, cls, title) {
      return el("button", { class: cls || null, onclick: onClick, title: title || null }, label);
    },
    input: function (placeholder, opts) {
      opts = opts || {};
      return el("input", {
        type: opts.type || "text",
        placeholder: placeholder || "",
        class: opts.class || null,
        maxlength: opts.maxlength || null,
        spellcheck: "false",
        autocomplete: "off",
        value: opts.value || null
      });
    },
    select: function () {
      return el("select");
    },
    fact: function (key, value) {
      return el(
        "div",
        { class: "fact" },
        el("div", { class: "k", text: key }),
        el("div", { class: "v", text: value, title: value })
      );
    },
    option: function (value, label) {
      return el("option", { value: value }, label === undefined ? value : label);
    }
  };

  /* ── boot ──────────────────────────────────────────────────────────── */

  NTC.start = function () {
    inspector.title = document.getElementById("inspector-title");
    inspector.body = document.getElementById("inspector-body");

    var stored = null;
    try {
      stored = localStorage.getItem(THEME_KEY);
    } catch (err) {
      stored = null;
    }
    if (!stored) {
      stored =
        global.matchMedia && global.matchMedia("(prefers-color-scheme: light)").matches
          ? "light"
          : "dark";
    }
    applyTheme(stored);

    buildRail();
    bindChrome();

    var route = parseHash();
    if (!route) {
      var saved = null;
      try {
        saved = localStorage.getItem(TAB_KEY);
      } catch (err) {
        saved = null;
      }
      route = { id: byId[saved] ? saved : "changes", args: [] };
      global.history.replaceState(null, "", "#/" + route.id);
    }
    activate(route);

    global.addEventListener("hashchange", function () {
      var next = parseHash();
      if (next) activate(next);
    });
  };
})(window);
