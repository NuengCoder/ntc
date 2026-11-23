/* NTC Git — Tags tab.
 *
 * List, create (lightweight or annotated) and delete local tags, and push
 * every tag to a remote.
 */
(function (global) {
  "use strict";

  var NTC = global.NTC;
  var el = NTC.el;
  var ui = NTC.ui;
  var store = NTC.store;

  var panel = ui.panel("Tags");
  var status = ui.status();
  var list = el("div", { class: "list" });
  var nameInput = ui.input("tag name");
  nameInput.classList.add("grow");
  var messageInput = ui.input("message (empty = lightweight)");
  messageInput.classList.add("grow");
  var selected = "";

  var deleteButton = ui.button("Delete", function () {
    if (!selected) return;
    var name = selected;
    NTC.confirm('Delete tag "' + name + '"? This runs git tag -d and only removes the local tag.', "Delete").then(
      function (ok) {
        if (!ok) return;
        NTC.mutate("/api/tag/delete", { name: name }, "Deleted tag " + name + ".", ["tags"]).then(function (
          data
        ) {
          if (data) {
            selected = "";
            refresh();
          }
        });
      }
    );
  }, "danger");

  var pushButton = ui.button("Push all tags", function () {
    var remote = (store.repo.remotes || [])[0];
    if (!remote) {
      status.set("No remote configured.", "err");
      return;
    }
    NTC.mutate("/api/tag/push", { remote: remote }, "Pushed tags to " + remote + ".", ["tags"]);
  }, null, "git push <first remote> --tags");

  function render() {
    var tags = store.tags.list || [];
    NTC.clear(list);
    panel.count(tags.length + (tags.length === 1 ? " tag" : " tags"));
    if (!tags.length) {
      selected = "";
      list.appendChild(ui.empty("No tags."));
      deleteButton.disabled = true;
      pushButton.disabled = true;
      return;
    }
    if (selected && !tags.some(function (tag) { return tag.name === selected; })) selected = "";
    tags.forEach(function (tag) {
      var row = el(
        "div",
        {
          class: "crow" + (tag.name === selected ? " active" : ""),
          title: "Select tag " + tag.name,
          tabindex: "0"
        },
        el("span", { class: "name", text: tag.name }),
        el("span", { class: "where", text: tag.hash }),
        el("span", { class: "summary", text: tag.summary || "(no message)", title: tag.summary || "" })
      );
      row.addEventListener("click", function () {
        selected = tag.name;
        render();
      });
      row.addEventListener("keydown", function (e) {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          selected = tag.name;
          render();
        }
      });
      list.appendChild(row);
    });
    deleteButton.disabled = !selected;
    pushButton.disabled = false;
  }

  function create() {
    var name = nameInput.value.trim();
    if (!name) {
      status.set("Enter a tag name.", "err");
      return;
    }
    NTC.mutate(
      "/api/tag/create",
      { name: name, message: messageInput.value.trim() },
      "Created tag " + name + ".",
      ["tags"]
    ).then(function (data) {
      if (data) {
        nameInput.value = "";
        messageInput.value = "";
        refresh();
      }
    });
  }

  function refresh() {
    return NTC.api("/api/tags")
      .then(function (data) {
        store.tags.list = data.tags || [];
        store.tags.loaded = true;
        render();
      })
      .catch(function (err) {
        NTC.clear(list);
        list.appendChild(ui.empty(err.message));
      });
  }

  function mount(root) {
    panel.body.appendChild(list);
    panel.body.appendChild(ui.row(deleteButton, pushButton));
    panel.body.appendChild(status.el);

    var createPanel = ui.panel("Create tag");
    createPanel.body.appendChild(ui.row(nameInput, messageInput));
    createPanel.body.appendChild(ui.row(ui.button("Create tag", create, "primary")));
    nameInput.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        e.preventDefault();
        create();
      }
    });

    root.appendChild(panel.root);
    root.appendChild(createPanel.root);
  }

  NTC.tabs.register({
    id: "tags",
    label: "Tags",
    icon: "◆",
    hint: "Create, delete and push tags",
    mount: mount,
    refresh: refresh,
    badge: function () {
      return store.tags.list.length;
    }
  });
})(window);
