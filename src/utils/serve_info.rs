//! The `/info` page served by `ntc serve files`.
//!
//! One self-contained HTML document (no JavaScript, no external assets) that
//! answers "what is this server and how do I use it?" with copy/paste recipes
//! for the browser, `curl`, `wget`, PowerShell, `ntc up`, `ntc dl` and
//! `ntc http`. Placeholders are filled by plain string replacement so the
//! braces in the stylesheet never need escaping.

/// Fill the template for one server instance.
pub(crate) fn render(base: &str, auth: bool, locked: bool) -> String {
    let mode = if locked {
        "read-only (--lock): listing and download only, uploads and edits return 403"
    } else {
        "read/write: browsing, upload, download, rename, delete"
    };
    let auth_line = if auth {
        "authentication is ON (--auth): every /api/* request needs HTTP Basic credentials"
    } else {
        "authentication is OFF: anyone who can reach the port can browse and upload"
    };
    TEMPLATE
        .replace("__BASE__", base)
        .replace("__MODE__", mode)
        .replace("__AUTH__", auth_line)
        .replace("__AUTH_CURL__", if auth { "-u user:pass " } else { "" })
        .replace(
            "__AUTH_PS__",
            if auth {
                "-Credential (Get-Credential) "
            } else {
                ""
            },
        )
}

const TEMPLATE: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>ntc serve files - info</title>
<style>
  :root { color-scheme: dark; }
  * { box-sizing: border-box; }
  body {
    margin: 0; padding: 32px 20px 64px;
    background: #101018; color: #d8d8e8;
    font: 15px/1.6 'Segoe UI', system-ui, sans-serif;
  }
  main { max-width: 860px; margin: 0 auto; }
  h1 { font-size: 26px; margin: 0 0 4px; color: #f2f2fa; }
  h2 { font-size: 18px; margin: 32px 0 8px; color: #ff6ec7; }
  p { margin: 8px 0; }
  .sub { color: #9a9ab0; }
  .url { color: #66d9ef; word-break: break-all; }
  .state { margin: 16px 0; padding: 10px 14px; border-left: 3px solid #ff6ec7; background: #181826; }
  code, pre {
    font-family: 'Cascadia Mono', Consolas, monospace;
    background: #181826; color: #e6e6f0; border-radius: 6px;
  }
  code { padding: 1px 5px; font-size: 13px; }
  pre { padding: 12px 14px; overflow-x: auto; border: 1px solid #26263a; }
  pre b { color: #9a9ab0; font-weight: 400; }
  table { width: 100%; border-collapse: collapse; margin: 10px 0; }
  th, td { text-align: left; padding: 6px 8px; border-bottom: 1px solid #26263a; font-size: 14px; }
  th { color: #9a9ab0; font-weight: 600; }
  a { color: #66d9ef; }
  .grid { display: grid; gap: 8px 16px; grid-template-columns: max-content 1fr; }
</style>
</head>
<body>
<main>
  <h1>This folder is being served by <code>ntc serve files</code></h1>
  <p class="sub">Server root: <span class="url">__BASE__</span></p>

  <div class="state">
    <p><b>Mode:</b> __MODE__</p>
    <p><b>Access:</b> __AUTH__</p>
  </div>

  <p>
    The page at <a href="/">/</a> is the file manager: pick a folder, drag files in,
    click a name to download. Everything below is the same server, driven from a
    terminal.
  </p>

  <h2>Endpoints</h2>
  <table>
    <tr><th>URL</th><th>Method</th><th>What it does</th></tr>
    <tr><td><span class="url">__BASE__/</span></td><td>GET</td><td>web file manager (browse, upload, download)</td></tr>
    <tr><td><span class="url">__BASE__/api/upload</span></td><td>POST</td><td>upload one file as <code>multipart/form-data</code>, field name <code>file</code></td></tr>
    <tr><td><span class="url">__BASE__/api/upload-chunk</span></td><td>POST</td><td>chunked upload (used by the web UI for big files)</td></tr>
    <tr><td><span class="url">__BASE__/api/files?path=/</span></td><td>GET</td><td>folder listing as JSON</td></tr>
    <tr><td><span class="url">__BASE__/download/root/&lt;path&gt;</span></td><td>GET</td><td>download a file (supports <code>Range</code>)</td></tr>
    <tr><td><span class="url">__BASE__/info</span></td><td>GET</td><td>this page</td></tr>
  </table>

  <h2>Upload</h2>
  <p class="sub">Browser: open <span class="url">__BASE__/</span> and drop files on the page (drag and drop anywhere in the list).</p>
  <pre><b># curl</b>
curl __AUTH_CURL__-F "file=@photo.jpg" __BASE__/api/upload

<b># wget</b>
wget __AUTH_CURL__--method POST --body-file=photo.jpg \
     --header="Content-Type: application/octet-stream" __BASE__/api/upload

<b># PowerShell</b>
Invoke-RestMethod __AUTH_PS__-Method Post -Form @{ file = Get-Item photo.jpg } __BASE__/api/upload

<b># ntc</b>
ntc up photo.jpg __BASE__/api/upload
ntc http POST __BASE__/api/upload -f photo.jpg</pre>

  <h2>Download</h2>
  <pre><b># curl</b>
curl __AUTH_CURL__-O __BASE__/download/root/photo.jpg

<b># wget</b>
wget __AUTH_CURL____BASE__/download/root/photo.jpg

<b># PowerShell</b>
Invoke-WebRequest __AUTH_PS__-Uri __BASE__/download/root/photo.jpg -OutFile photo.jpg

<b># ntc</b>
ntc dl __BASE__/download/root/photo.jpg
ntc http GET __BASE__/download/root/photo.jpg -o photo.jpg</pre>

  <h2>Notes</h2>
  <p>Downloads are path-safe: <code>..</code> cannot escape the shared root, and a
     <code>Range</code> request resumes a partial file.</p>
  <p>Uploads stream to disk, so files up to 10 GB work without loading them into memory.</p>
  <p>Folders are created from the web UI (<code>+ Folder</code>) or with
     <code>curl -X POST -H "Content-Type: application/json" -d '{"path":"/","name":"docs"}' __BASE__/api/create-folder</code>.</p>
  <p>Stop the server with <code>Ctrl+C</code> in the terminal that started it.</p>
</main>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn info_page_carries_every_recipe() {
        let html = render("http://192.168.1.20:8080", false, false);
        assert!(html.contains("http://192.168.1.20:8080/api/upload"));
        assert!(html.contains("http://192.168.1.20:8080/download/root/"));
        assert!(html.contains("curl -F \"file=@photo.jpg\""));
        assert!(html.contains("wget "));
        assert!(html.contains("Invoke-RestMethod"));
        assert!(html.contains("ntc up photo.jpg"));
        assert!(html.contains("ntc dl "));
        assert!(html.contains("read/write"));
        // No leftover placeholders.
        assert!(!html.contains("__BASE__"));
        assert!(!html.contains("__AUTH"));
    }

    #[test]
    fn info_page_reflects_auth_and_lock() {
        let html = render("http://h:1", true, true);
        assert!(html.contains("read-only"));
        assert!(html.contains("authentication is ON"));
        // Credential placeholders are filled with the flags.
        assert!(html.contains("-u user:pass"));
        assert!(html.contains("-Credential (Get-Credential)"));
        assert!(!html.contains("__BASE__"));
    }
}
