/* NTC mdview — file browser, editor and live markdown preview.
 *
 * The renderer is a deliberate subset of CommonMark: headings, fenced code,
 * blockquotes, nested lists, task lists, tables, rules, paragraphs and the
 * usual inline spans. Images, video and audio resolve relative to the open
 * document and stream from `/raw`. Raw HTML is escaped rather than trusted, so
 * a document can never inject markup into the page.
 */
(function (global) {
  "use strict";

  /* Folder of the open document, so relative media resolves next to it. */
  var currentDir = "";

  var IMAGE_EXT = /\.(png|jpe?g|gif|webp|avif|bmp|svg|ico)$/i;
  var VIDEO_EXT = /\.(mp4|m4v|webm|ogv|mov)$/i;
  var AUDIO_EXT = /\.(mp3|wav|ogg|m4a|flac|aac)$/i;
  var MD_EXT = /\.(md|markdown|mdown|mkd)$/i;

  /* ── markdown ─────────────────────────────────────────────────────── */

  function esc(text) {
    return String(text)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  /* Absolute URLs and data URIs are used as written; everything else is a
   * path inside the served folder. */
  function isRemote(src) {
    return /^(?:[a-z][a-z0-9+.-]*:|\/\/)/i.test(src);
  }

  /* Join `src` onto the document's folder and fold `.` / `..` away, so
   * `../img/a.png` from `docs/guide.md` becomes `img/a.png`. */
  function resolveRelative(src) {
    if (!src || isRemote(src) || src.charAt(0) === "#") return src;
    var rooted = src.charAt(0) === "/";
    var parts = rooted ? [] : currentDir.split("/").filter(Boolean);
    var tail = rooted ? src.slice(1) : src;
    tail.split("/").forEach(function (part) {
      if (!part || part === ".") return;
      if (part === "..") {
        parts.pop();
        return;
      }
      parts.push(part);
    });
    return parts.join("/");
  }

  function mediaUrl(src) {
    if (isRemote(src)) return src;
    return "/raw?path=" + encodeURIComponent(resolveRelative(src));
  }

  function mediaFigure(kind, src, alt) {
    var caption = alt ? "<figcaption>" + alt + "</figcaption>" : "";
    if (kind === "video") {
      // `#t=0.1` makes the browser paint the first frame instead of a black
      // rectangle, without asking it to download the whole clip.
      return (
        '<figure class="media"><video src="' +
        mediaUrl(src) +
        '#t=0.1" controls preload="metadata" playsinline></video>' +
        caption +
        "</figure>"
      );
    }
    if (kind === "audio") {
      return (
        '<figure class="media"><audio src="' +
        mediaUrl(src) +
        '" controls preload="metadata"></audio>' +
        caption +
        "</figure>"
      );
    }
    return '<img src="' + mediaUrl(src) + '" alt="' + alt + '">';
  }

  /* ── raw html ─────────────────────────────────────────────────────── */

  /* A README centres its badge and its demo with plain HTML. Those blocks are
   * passed through instead of escaped, but only through this allowlist: an
   * unknown tag is dropped, `on*` / `style` / `class` never survive, and every
   * URL is re-checked. */
  var HTML_TAGS = listToSet(
    "p div span br hr b strong i em u s del sub sup code pre small " +
      "h1 h2 h3 h4 h5 h6 ul ol li a img video audio source picture figure " +
      "figcaption blockquote table thead tbody tr td th details summary kbd mark"
  );

  var HTML_ATTRS = listToSet(
    "href src alt title width height align colspan rowspan controls loop " +
      "muted poster type playsinline target start reversed"
  );

  var VOID_TAGS = listToSet("br hr img source");

  function listToSet(words) {
    var set = {};
    words.split(" ").forEach(function (word) {
      set[word] = true;
    });
    return set;
  }

  /* Re-check a URL from a tag: drop script-bearing schemes, keep absolute and
   * data-image URLs, and route a relative path through `/raw`. */
  function safeUrl(value, isHref) {
    var url = value.trim();
    if (/^(?:javascript|vbscript|data:text|data:application)/i.test(url)) return null;
    if (/^data:image\//i.test(url)) return isHref ? null : url;
    if (isRemote(url) || /^mailto:/i.test(url)) return url;
    if (isHref) {
      if (MD_EXT.test(url)) return "#";
      if (!IMAGE_EXT.test(url) && !VIDEO_EXT.test(url) && !AUDIO_EXT.test(url)) {
        // Some other relative link (`LICENSE`): leave it to the browser.
        return url;
      }
    }
    return mediaUrl(url);
  }

  function sanitizeTag(raw) {
    if (/^<!--/.test(raw)) return "";
    var match = /^<\s*(\/?)\s*([a-zA-Z][a-zA-Z0-9]*)([\s\S]*?)\/?\s*>$/.exec(raw);
    if (!match) return "";
    var closing = match[1] === "/";
    var name = match[2].toLowerCase();
    if (!HTML_TAGS[name]) return "";
    if (closing) return VOID_TAGS[name] ? "" : "</" + name + ">";

    var attrs = "";
    var mdTarget = null;
    var attrRe = /([a-zA-Z_:][-a-zA-Z0-9_:.]*)\s*(?:=\s*("[^"]*"|'[^']*'|[^\s"'>]+))?/g;
    var m;
    while ((m = attrRe.exec(match[3]))) {
      var key = m[1].toLowerCase();
      if (!HTML_ATTRS[key]) continue;
      if (!m[2]) {
        attrs += " " + key;
        continue;
      }
      var value = m[2].replace(/^["']/, "").replace(/["']$/, "");
      if (key === "src" || key === "href" || key === "poster") {
        var safe = safeUrl(value, key === "href");
        if (safe === null) continue;
        if (key === "href" && safe === "#") {
          mdTarget = resolveRelative(value);
          continue;
        }
        value = safe;
      }
      attrs += " " + key + '="' + esc(value) + '"';
    }
    if (mdTarget !== null) {
      attrs = ' href="#" data-md="' + esc(mdTarget) + '"' + attrs;
    }
    if (name === "a" && attrs.indexOf("target=") >= 0) {
      attrs += ' rel="noopener"';
    }
    return "<" + name + attrs + ">";
  }

  function sanitizeHtml(html) {
    var out = "";
    var last = 0;
    var tagRe = /<[^>]*>/g;
    var m;
    while ((m = tagRe.exec(html))) {
      out += html.slice(last, m.index);
      out += sanitizeTag(m[0]);
      last = m.index + m[0].length;
    }
    out += html.slice(last);
    return out;
  }

  function isHtmlLine(line) {
    return /^\s{0,3}<(?:\/?[a-zA-Z][a-zA-Z0-9]*|!--)/.test(line);
  }

  /* ── editor highlighting ──────────────────────────────────────────── */

  /* The editor is a transparent textarea over a <pre>, so this has to emit
   * exactly the same characters: every token is wrapped in a span, and plain
   * text is escaped. Colours are CSS variables filled from `/api/theme`. */
  function span(role, text) {
    if (!text) return "";
    return '<span class="t-' + role + '">' + esc(text) + "</span>";
  }

  var INLINE_STOP = "`[<|*_";

  function inlineMarkdown(line) {
    var out = "";
    var i = 0;
    var n = line.length;
    while (i < n) {
      var rest = line.slice(i);
      var m;

      // inline code: `x`
      if (line[i] === "`" && (m = /^(`+)([\s\S]*?)\1/.exec(rest))) {
        out += span("punctuation", m[1]) + span("string", m[2]) + span("punctuation", m[1]);
        i += m[0].length;
        continue;
      }
      // image / link: ![alt](src) or [text](href)
      if ((m = /^(!?)\[([^\]]*)\]\(([^)]*)\)/.exec(rest))) {
        out +=
          span("punctuation", m[1] + "[") +
          esc(m[2]) +
          span("punctuation", "](") +
          span("string", m[3]) +
          span("punctuation", ")");
        i += m[0].length;
        continue;
      }
      // bold / italic markers
      if ((m = /^(\*\*|__)([^*_]+)\1/.exec(rest))) {
        out += span("punctuation", m[1]) + esc(m[2]) + span("punctuation", m[1]);
        i += m[0].length;
        continue;
      }
      // a raw HTML tag
      if (line[i] === "<" && (m = /^<\/?[a-zA-Z][^>]*>/.exec(rest))) {
        out += span("tag", m[0]);
        i += m[0].length;
        continue;
      }
      // table pipe
      if (line[i] === "|") {
        out += span("punctuation", "|");
        i += 1;
        continue;
      }
      // plain run
      var j = i + 1;
      while (j < n && INLINE_STOP.indexOf(line[j]) < 0) j++;
      out += esc(line.slice(i, j));
      i = j;
    }
    return out;
  }

  /* Line-oriented markdown highlighting: one pass, no parser state beyond the
   * fenced-code flag. */
  function highlightMarkdown(source) {
    var lines = String(source || "").split("\n");
    var out = [];
    var inFence = false;
    for (var i = 0; i < lines.length; i++) {
      var line = lines[i];
      var m;
      if (/^\s*(```+|~~~+)/.test(line)) {
        inFence = !inFence;
        out.push(span("punctuation", line));
      } else if (inFence) {
        out.push(esc(line));
      } else if (/^\s{0,3}#{1,6}\s/.test(line)) {
        out.push(span("keyword", line));
      } else if (/^\s{0,3}([-*_])(\s*\1){2,}\s*$/.test(line)) {
        out.push(span("punctuation", line));
      } else if (/^\s{0,3}>/.test(line)) {
        out.push(span("comment", line));
      } else if ((m = /^(\s*)([-*+]|\d+[.)])(\s+)([\s\S]*)$/.exec(line))) {
        out.push(
          esc(m[1]) + span("operator", m[2]) + esc(m[3]) + inlineMarkdown(m[4])
        );
      } else {
        out.push(inlineMarkdown(line));
      }
    }
    return out.join("\n");
  }

  /* ── theme ────────────────────────────────────────────────────────── */

  var SYNTAX_ROLES = [
    "keyword", "string", "comment", "number", "type", "builtin", "function",
    "operator", "punctuation", "attribute", "macro", "tag", "constant",
    "normal", "muted"
  ];

  function luma(hex) {
    var m = /^#?([0-9a-f]{6})$/i.exec(String(hex || "").trim());
    if (!m) return 0;
    var n = parseInt(m[1], 16);
    return (
      (0.2126 * ((n >> 16) & 255) + 0.7152 * ((n >> 8) & 255) + 0.0722 * (n & 255)) / 255
    );
  }

  function applyNtcTheme(theme) {
    var root = document.documentElement;
    var syntax = theme.syntax || {};
    SYNTAX_ROLES.forEach(function (role) {
      if (syntax[role]) root.style.setProperty("--t-" + role, syntax[role]);
    });
    var editorTheme = theme.editor || {};
    if (editorTheme.bg) root.style.setProperty("--ed-bg", editorTheme.bg);
    if (editorTheme.fg) root.style.setProperty("--ed-fg", editorTheme.fg);
    if (editorTheme.selection) root.style.setProperty("--ed-sel", editorTheme.selection);

    // Follow the theme's brightness unless the page theme was chosen by hand.
    var stored = null;
    try {
      stored = localStorage.getItem("ntc.mdview.theme");
    } catch (err) {
      /* private mode */
    }
    if (!stored && editorTheme.bg) {
      applyTheme(luma(editorTheme.bg) > 0.55 ? "light" : "dark");
    }
  }

  function loadTheme() {
    return api("/api/theme")
      .then(applyNtcTheme)
      .catch(function () {
        /* keep the built-in palette */
      });
  }

  function inline(text) {
    var stash = [];
    var out = esc(text);

    out = out.replace(/`([^`]+)`/g, function (_, code) {
      stash.push("<code>" + code + "</code>");
      return "\u0000" + (stash.length - 1) + "\u0000";
    });

    out = out.replace(/!\[([^\]]*)\]\(([^)\s]+)\)/g, function (_, alt, src) {
      // `esc()` already ran over the line, so undo its entity for the path.
      var path = src.replace(/&amp;/g, "&");
      if (VIDEO_EXT.test(path)) return mediaFigure("video", path, alt);
      if (AUDIO_EXT.test(path)) return mediaFigure("audio", path, alt);
      return mediaFigure("image", path, alt);
    });
    out = out.replace(/\[([^\]]+)\]\(([^)\s]+)\)/g, function (_, label, href) {
      var path = href.replace(/&amp;/g, "&");
      if (!isRemote(path)) {
        // A link to another document opens it in the editor instead of
        // leaving the page.
        if (MD_EXT.test(path)) {
          return '<a href="#" data-md="' + esc(resolveRelative(path)) + '">' + label + "</a>";
        }
        if (IMAGE_EXT.test(path) || VIDEO_EXT.test(path) || AUDIO_EXT.test(path)) {
          return (
            '<a href="' + mediaUrl(path) + '" target="_blank" rel="noopener">' + label + "</a>"
          );
        }
      }
      return '<a href="' + href + '" target="_blank" rel="noopener">' + label + "</a>";
    });
    out = out.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
    out = out.replace(/__([^_]+)__/g, "<strong>$1</strong>");
    out = out.replace(/(^|[^*])\*([^*\s][^*]*)\*/g, "$1<em>$2</em>");
    out = out.replace(/(^|[^_\w])_([^_\s][^_]*)_(?![\w_])/g, "$1<em>$2</em>");
    out = out.replace(/~~([^~]+)~~/g, "<del>$1</del>");
    out = out.replace(
      /(^|[\s(])(https?:\/\/[^\s<)]+)/g,
      '$1<a href="$2" target="_blank" rel="noopener">$2</a>'
    );

    return out.replace(/\u0000(\d+)\u0000/g, function (_, index) {
      return stash[Number(index)];
    });
  }

  function isBlockStart(line) {
    return (
      /^\s{0,3}(#{1,6})\s+/.test(line) ||
      /^\s*(```+|~~~+)/.test(line) ||
      /^\s{0,3}>/.test(line) ||
      /^\s*([-*+]|\d+[.)])\s+/.test(line) ||
      /^\s{0,3}([-*_])(\s*\1){2,}\s*$/.test(line) ||
      /^\s*\|/.test(line) ||
      isHtmlLine(line)
    );
  }

  function splitRow(line) {
    return line
      .trim()
      .replace(/^\|/, "")
      .replace(/\|$/, "")
      .split("|")
      .map(function (cell) {
        return cell.trim();
      });
  }

  function readTable(lines, start) {
    if (start + 1 >= lines.length) return null;
    var header = splitRow(lines[start]);
    var separator = splitRow(lines[start + 1]);
    if (!header.length || header.length !== separator.length) return null;
    for (var s = 0; s < separator.length; s++) {
      if (!/^:?-{1,}:?$/.test(separator[s])) return null;
    }
    var align = separator.map(function (cell) {
      var left = cell.charAt(0) === ":";
      var right = cell.charAt(cell.length - 1) === ":";
      return left && right ? "center" : right ? "right" : left ? "left" : "";
    });

    var i = start + 2;
    var rows = [];
    while (i < lines.length && lines[i].trim() && lines[i].indexOf("|") >= 0) {
      rows.push(splitRow(lines[i]));
      i++;
    }

    function cell(tag, value, index) {
      return (
        "<" +
        tag +
        (align[index] ? ' style="text-align:' + align[index] + '"' : "") +
        ">" +
        inline(value || "") +
        "</" +
        tag +
        ">"
      );
    }

    var html = "<table><thead><tr>";
    header.forEach(function (value, index) {
      html += cell("th", value, index);
    });
    html += "</tr></thead><tbody>";
    rows.forEach(function (row) {
      html += "<tr>";
      header.forEach(function (_, index) {
        html += cell("td", row[index], index);
      });
      html += "</tr>";
    });
    html += "</tbody></table>";
    return { html: html, next: i };
  }

  function readList(lines, start) {
    var first = /^(\s*)([-*+]|\d+[.)])\s+/.exec(lines[start]);
    var baseIndent = first[1].replace(/\t/g, "  ").length;
    var ordered = /\d/.test(first[2]);
    var html = ["<" + (ordered ? "ol" : "ul") + ">"];
    var i = start;

    while (i < lines.length) {
      var match = /^(\s*)([-*+]|\d+[.)])\s+(.*)$/.exec(lines[i]);
      if (!match) {
        // A blank line only continues the list when an item follows it.
        if (!lines[i].trim() && i + 1 < lines.length) {
          var peek = /^(\s*)([-*+]|\d+[.)])\s+/.exec(lines[i + 1]);
          if (peek && peek[1].replace(/\t/g, "  ").length >= baseIndent) {
            i++;
            continue;
          }
        }
        break;
      }
      var indent = match[1].replace(/\t/g, "  ").length;
      if (indent < baseIndent) break;
      // A different marker at the same level starts a new list, so `- a` then
      // `1. b` does not merge into one bullet list.
      if (indent === baseIndent && ordered !== /\d/.test(match[2])) break;
      if (indent > baseIndent) {
        var sub = readList(lines, i);
        html[html.length - 1] = html[html.length - 1].replace(/<\/li>$/, sub.html + "</li>");
        i = sub.next;
        continue;
      }
      var text = match[3];
      var task = /^\[( |x|X)\]\s+(.*)$/.exec(text);
      if (task) {
        html.push(
          '<li class="task"><input type="checkbox" disabled' +
            (task[1].toLowerCase() === "x" ? " checked" : "") +
            ">" +
            inline(task[2]) +
            "</li>"
        );
      } else {
        html.push("<li>" + inline(text) + "</li>");
      }
      i++;
    }
    html.push("</" + (ordered ? "ol" : "ul") + ">");
    return { html: html.join(""), next: i };
  }

  function render(source) {
    var lines = String(source || "")
      .replace(/\r\n?/g, "\n")
      .split("\n");
    var out = [];
    var i = 0;

    while (i < lines.length) {
      var line = lines[i];
      if (!line.trim()) {
        i++;
        continue;
      }

      var fence = /^\s*(```+|~~~+)\s*([\w+#.-]*)\s*$/.exec(line);
      if (fence) {
        var marker = fence[1].charAt(0);
        var lang = fence[2];
        var body = [];
        i++;
        var closer = new RegExp("^\\s*" + marker + "{3,}\\s*$");
        while (i < lines.length && !closer.test(lines[i])) {
          body.push(lines[i]);
          i++;
        }
        i++;
        out.push(
          "<pre><code" +
            (lang ? ' class="language-' + esc(lang) + '"' : "") +
            ">" +
            esc(body.join("\n")) +
            "</code></pre>"
        );
        continue;
      }

      var heading = /^\s{0,3}(#{1,6})\s+(.*?)\s*#*\s*$/.exec(line);
      if (heading) {
        var level = heading[1].length;
        out.push("<h" + level + ">" + inline(heading[2]) + "</h" + level + ">");
        i++;
        continue;
      }

      // A raw HTML block runs until the next blank line, like CommonMark.
      if (isHtmlLine(line)) {
        var html = [];
        while (i < lines.length && lines[i].trim()) {
          html.push(lines[i]);
          i++;
        }
        out.push(sanitizeHtml(html.join("\n")));
        continue;
      }

      if (/^\s{0,3}([-*_])(\s*\1){2,}\s*$/.test(line)) {
        out.push("<hr>");
        i++;
        continue;
      }

      if (line.indexOf("|") >= 0 && i + 1 < lines.length) {
        var table = readTable(lines, i);
        if (table) {
          out.push(table.html);
          i = table.next;
          continue;
        }
      }

      if (/^\s{0,3}>/.test(line)) {
        var quote = [];
        while (i < lines.length && /^\s{0,3}>/.test(lines[i])) {
          quote.push(lines[i].replace(/^\s{0,3}>\s?/, ""));
          i++;
        }
        out.push("<blockquote>" + render(quote.join("\n")) + "</blockquote>");
        continue;
      }

      if (/^\s*([-*+]|\d+[.)])\s+/.test(line)) {
        var list = readList(lines, i);
        out.push(list.html);
        i = list.next;
        continue;
      }

      var paragraph = [];
      while (i < lines.length && lines[i].trim() && !(paragraph.length && isBlockStart(lines[i]))) {
        paragraph.push(lines[i]);
        i++;
      }
      out.push("<p>" + inline(paragraph.join(" ")) + "</p>");
    }

    return out.join("\n");
  }

  global.md = { render: render };

  /* ── state ────────────────────────────────────────────────────────── */

  var state = {
    files: [],
    path: "",
    dirty: false,
    filter: "",
    mode: "split",
    saving: false
  };

  var editor = document.getElementById("editor");
  var highlightLayer = document.getElementById("hl");
  var preview = document.getElementById("preview");
  var pathLabel = document.getElementById("path");
  var dirtyDot = document.getElementById("dirty");
  var saveButton = document.getElementById("save");
  var fileList = document.getElementById("files");
  var filterInput = document.getElementById("filter");
  var panes = document.getElementById("panes");
  var stats = document.getElementById("stats");
  var message = document.getElementById("msg");

  /* ── helpers ──────────────────────────────────────────────────────── */

  function api(url, opts) {
    return fetch(url, opts || {}).then(function (response) {
      return response
        .json()
        .catch(function () {
          return { error: response.statusText || "HTTP " + response.status };
        })
        .then(function (data) {
          if (!response.ok) throw new Error(data.error || "request failed");
          return data;
        });
    });
  }

  function toast(text, kind) {
    var box = document.getElementById("toasts");
    var node = document.createElement("div");
    node.className = "toast " + (kind || "");
    node.textContent = text;
    node.addEventListener("click", function () {
      node.remove();
    });
    box.appendChild(node);
    while (box.children.length > 3) box.removeChild(box.firstChild);
    global.setTimeout(function () {
      node.remove();
    }, 3600);
  }

  function say(text, kind) {
    message.textContent = text || "";
    message.className = kind || "";
    if (text) {
      global.clearTimeout(say.timer);
      say.timer = global.setTimeout(function () {
        message.textContent = "";
        message.className = "";
      }, 4000);
    }
  }

  function setDirty(value) {
    state.dirty = value;
    dirtyDot.classList.toggle("hidden", !value);
    saveButton.disabled = !state.path || state.saving;
  }

  function countWords(text) {
    var words = text.trim();
    if (!words) return 0;
    return words.split(/\s+/).length;
  }

  function updateStats() {
    var text = editor.value;
    stats.textContent =
      countWords(text) + " words · " + text.length + " chars · " + text.split("\n").length + " lines";
  }

  /* Re-render the preview, keeping a playing video where it was: replacing
   * the markup would otherwise restart it on every keystroke. */
  function renderPreview() {
    var saved = [];
    var videos = preview.querySelectorAll("video");
    for (var i = 0; i < videos.length; i++) {
      if (!videos[i].paused || videos[i].currentTime > 0) {
        saved.push({
          src: videos[i].getAttribute("src"),
          time: videos[i].currentTime,
          paused: videos[i].paused
        });
      }
    }

    preview.innerHTML = render(editor.value);

    if (!saved.length) return;
    var fresh = preview.querySelectorAll("video");
    for (var j = 0; j < fresh.length; j++) {
      for (var k = 0; k < saved.length; k++) {
        if (fresh[j].getAttribute("src") !== saved[k].src) continue;
        (function (video, before) {
          video.addEventListener(
            "loadedmetadata",
            function () {
              try {
                video.currentTime = before.time;
              } catch (err) {
                /* the clip may have got shorter */
              }
              if (!before.paused) video.play().catch(function () {});
            },
            { once: true }
          );
        })(fresh[j], saved[k]);
        break;
      }
    }
  }

  var renderTimer = null;
  function scheduleRender() {
    global.clearTimeout(renderTimer);
    renderTimer = global.setTimeout(renderPreview, 90);
  }

  /* Repaint the syntax layer behind the textarea and keep it scrolled in step
   * with the text the user sees. */
  function paintHighlight() {
    if (!highlightLayer) return;
    highlightLayer.innerHTML = highlightMarkdown(editor.value);
    syncHighlightScroll();
  }

  function syncHighlightScroll() {
    if (!highlightLayer) return;
    highlightLayer.scrollTop = editor.scrollTop;
    highlightLayer.scrollLeft = editor.scrollLeft;
  }

  var highlightFrame = null;
  function scheduleHighlight() {
    if (highlightFrame !== null) return;
    if (!global.requestAnimationFrame) {
      paintHighlight();
      return;
    }
    highlightFrame = global.requestAnimationFrame(function () {
      highlightFrame = null;
      paintHighlight();
    });
  }

  /* ── files ────────────────────────────────────────────────────────── */

  function renderFiles() {
    fileList.innerHTML = "";
    var filter = state.filter.toLowerCase();
    var shown = state.files.filter(function (file) {
      return !filter || file.path.toLowerCase().indexOf(filter) >= 0;
    });
    if (!shown.length) {
      var empty = document.createElement("div");
      empty.className = "file";
      empty.style.color = "var(--muted)";
      empty.textContent = state.files.length ? "No match." : "No markdown files yet.";
      fileList.appendChild(empty);
      return;
    }
    shown.forEach(function (file) {
      var button = document.createElement("button");
      button.className = "file" + (file.path === state.path ? " active" : "");
      button.title = file.path;
      var slash = file.path.lastIndexOf("/");
      if (slash > 0) {
        var dir = document.createElement("span");
        dir.className = "dir";
        dir.textContent = file.path.slice(0, slash + 1);
        button.appendChild(dir);
      }
      var name = document.createElement("span");
      name.className = "name";
      name.textContent = slash > 0 ? file.path.slice(slash + 1) : file.path;
      button.appendChild(name);
      button.addEventListener("click", function () {
        openFile(file.path);
      });
      fileList.appendChild(button);
    });
  }

  function loadFiles() {
    return api("/api/files")
      .then(function (data) {
        state.files = data.files || [];
        renderFiles();
        return data;
      })
      .catch(function (err) {
        toast(err.message, "err");
      });
  }

  function confirmDiscard() {
    if (!state.dirty) return Promise.resolve(true);
    return new Promise(function (resolve) {
      var settled = false;
      var dialog = document.createElement("dialog");
      dialog.className = "name";
      dialog.innerHTML =
        '<p>Discard unsaved changes to <b>' +
        esc(state.path || "this document") +
        "</b>?</p>" +
        '<div class="row"><button data-act="cancel">Cancel</button>' +
        '<button data-act="ok" class="primary">Discard</button></div>';
      function finish(value) {
        if (settled) return;
        settled = true;
        resolve(value);
      }
      dialog.querySelector('[data-act="cancel"]').addEventListener("click", function () {
        finish(false);
        dialog.close();
      });
      dialog.querySelector('[data-act="ok"]').addEventListener("click", function () {
        finish(true);
        dialog.close();
      });
      dialog.addEventListener("close", function () {
        finish(false);
        dialog.remove();
      });
      document.body.appendChild(dialog);
      dialog.showModal();
    });
  }

  function openFile(path) {
    if (path === state.path && !state.dirty) return Promise.resolve();
    return confirmDiscard().then(function (ok) {
      if (!ok) return null;
      return api("/api/file?path=" + encodeURIComponent(path))
        .then(function (data) {
          state.path = data.path;
          // Media in the document is relative to its own folder.
          var slash = data.path.lastIndexOf("/");
          currentDir = slash > 0 ? data.path.slice(0, slash) : "";
          editor.value = data.content;
          pathLabel.textContent = data.path;
          // Deep link: `#docs%2Fguide.md` reopens this document after a reload.
          try {
            global.history.replaceState(null, "", "#" + encodeURIComponent(data.path));
          } catch (err) {
            /* file:// or a blocked history API */
          }
          setDirty(false);
          renderFiles();
          updateStats();
          paintHighlight();
          renderPreview();
          say("Opened " + data.path);
        })
        .catch(function (err) {
          toast(err.message, "err");
        });
    });
  }

  function save() {
    if (!state.path || state.saving) return Promise.resolve();
    state.saving = true;
    saveButton.disabled = true;
    return api("/api/file", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ path: state.path, content: editor.value })
    })
      .then(function (data) {
        setDirty(false);
        say("Saved " + data.path + " (" + data.bytes + " bytes)", "ok");
        return loadFiles();
      })
      .catch(function (err) {
        toast(err.message, "err");
        say(err.message, "err");
      })
      .then(function () {
        state.saving = false;
        saveButton.disabled = !state.path;
      });
  }

  function askName(suggestion) {
    return new Promise(function (resolve) {
      var settled = false;
      var dialog = document.createElement("dialog");
      dialog.className = "name";
      dialog.innerHTML =
        "<p>New document name (saved under the served folder):</p>" +
        '<input type="text" value="' +
        esc(suggestion || "notes.md") +
        '">' +
        '<div class="row"><button data-act="cancel">Cancel</button>' +
        '<button data-act="ok" class="primary">Create</button></div>';
      var input = dialog.querySelector("input");
      function finish(value) {
        if (settled) return;
        settled = true;
        resolve(value);
      }
      dialog.querySelector('[data-act="cancel"]').addEventListener("click", function () {
        finish(null);
        dialog.close();
      });
      dialog.querySelector('[data-act="ok"]').addEventListener("click", function () {
        var value = input.value.trim();
        finish(value || null);
        dialog.close();
      });
      input.addEventListener("keydown", function (event) {
        if (event.key === "Enter") {
          event.preventDefault();
          var value = input.value.trim();
          finish(value || null);
          dialog.close();
        }
      });
      dialog.addEventListener("close", function () {
        finish(null);
        dialog.remove();
      });
      document.body.appendChild(dialog);
      dialog.showModal();
      input.focus();
      input.select();
    });
  }

  function newFile() {
    confirmDiscard().then(function (ok) {
      if (!ok) return;
      askName("notes.md").then(function (name) {
        if (!name) return;
        var path = /\.(md|markdown|mdown|mkd)$/i.test(name) ? name : name + ".md";
        api("/api/file", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ path: path, content: "# " + path.replace(/\.\w+$/, "") + "\n\n" })
        })
          .then(function () {
            return loadFiles();
          })
          .then(function () {
            return openFile(path);
          })
          .catch(function (err) {
            toast(err.message, "err");
          });
      });
    });
  }

  /* ── chrome ───────────────────────────────────────────────────────── */

  function setMode(mode) {
    state.mode = mode;
    panes.className = mode;
    Array.prototype.forEach.call(document.querySelectorAll("#modes button"), function (button) {
      button.classList.toggle("on", button.dataset.mode === mode);
    });
    try {
      localStorage.setItem("ntc.mdview.mode", mode);
    } catch (err) {
      /* private mode */
    }
  }

  function applyTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme === "light" ? "light" : "dark");
    try {
      localStorage.setItem("ntc.mdview.theme", theme);
    } catch (err) {
      /* private mode */
    }
  }

  function wrapSelection(before, after) {
    var start = editor.selectionStart;
    var end = editor.selectionEnd;
    var value = editor.value;
    var selected = value.slice(start, end);
    editor.value =
      value.slice(0, start) + before + selected + after + value.slice(end);
    editor.selectionStart = start + before.length;
    editor.selectionEnd = start + before.length + selected.length;
    editor.focus();
    setDirty(true);
    scheduleHighlight();
    scheduleRender();
  }

  function bind() {
    editor.addEventListener("input", function () {
      setDirty(true);
      updateStats();
      scheduleHighlight();
      scheduleRender();
    });

    editor.addEventListener("scroll", syncHighlightScroll);

    editor.addEventListener("keydown", function (event) {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
        event.preventDefault();
        save();
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "n") {
        event.preventDefault();
        newFile();
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "b") {
        event.preventDefault();
        wrapSelection("**", "**");
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "i") {
        event.preventDefault();
        wrapSelection("*", "*");
        return;
      }
      if (event.key === "Tab") {
        event.preventDefault();
        var start = editor.selectionStart;
        editor.setRangeText("  ", start, editor.selectionEnd, "end");
        setDirty(true);
        scheduleHighlight();
        scheduleRender();
      }
    });

    saveButton.addEventListener("click", save);
    document.getElementById("new-file").addEventListener("click", newFile);
    document.getElementById("theme").addEventListener("click", function () {
      var current = document.documentElement.getAttribute("data-theme");
      applyTheme(current === "light" ? "dark" : "light");
    });

    // `[other.md](other.md)` inside a document opens that file in the editor.
    preview.addEventListener("click", function (event) {
      var link = event.target.closest ? event.target.closest("a[data-md]") : null;
      if (!link) return;
      event.preventDefault();
      openFile(link.getAttribute("data-md"));
    });

    filterInput.addEventListener("input", function () {
      state.filter = filterInput.value.trim();
      renderFiles();
    });

    Array.prototype.forEach.call(document.querySelectorAll("#modes button"), function (button) {
      button.addEventListener("click", function () {
        setMode(button.dataset.mode);
      });
    });

    global.addEventListener("beforeunload", function (event) {
      if (!state.dirty) return;
      event.preventDefault();
      event.returnValue = "";
    });

    document.addEventListener("keydown", function (event) {
      if (event.key === "Escape" && document.querySelector("dialog[open]")) return;
    });
  }

  function start() {
    var theme = null;
    var mode = null;
    try {
      theme = localStorage.getItem("ntc.mdview.theme");
      mode = localStorage.getItem("ntc.mdview.mode");
    } catch (err) {
      /* private mode */
    }
    if (!theme) {
      theme =
        global.matchMedia && global.matchMedia("(prefers-color-scheme: light)").matches
          ? "light"
          : "dark";
    }
    applyTheme(theme);
    setMode(mode === "edit" || mode === "preview" ? mode : "split");
    bind();
    setDirty(false);
    updateStats();
    paintHighlight();

    // The ntc theme supplies the syntax colours; it also decides light or dark
    // when the page theme has never been chosen by hand.
    loadTheme();

    loadFiles().then(function () {
      // `#<path>` wins over the first file, so a document can be bookmarked.
      var wanted = "";
      try {
        wanted = decodeURIComponent((global.location.hash || "").replace(/^#/, ""));
      } catch (err) {
        wanted = "";
      }
      var known = state.files.some(function (file) {
        return file.path === wanted;
      });
      if (known) openFile(wanted);
      else if (state.files.length) openFile(state.files[0].path);
    });
  }

  start();
})(window);
