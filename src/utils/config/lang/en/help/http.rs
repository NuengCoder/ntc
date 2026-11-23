use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.http.title".into(), "HTTP COMMAND:".into());

    m.insert(
        "help.http.usage".into(),
        "http <method> <url> [options] ## Send a request (method defaults to GET)".into(),
    );

    m.insert(
        "help.http.usage2".into(),
        "http GET|POST|PUT|PATCH|DELETE|HEAD|QUERY <url> ## Supported methods".into(),
    );

    m.insert(
        "help.http.note".into(),
        "Options: -H \"Key: Value\" header, -d text body, --json JSON body (sets Content-Type), -q key=value query param, -a user:pass Basic auth, --bearer token, -o file save the body, -s print only the body, -v show the request, -t seconds timeout (default 30). JSON responses are pretty-printed unless --no-pretty is given.".into(),
    );

    m.insert(
        "help.http.example1".into(),
        "http GET https://api.example.com/users ## Simple GET request".into(),
    );

    m.insert(
        "help.http.example2".into(),
        "http POST https://api.example.com/users --json '{\"name\":\"ada\"}' ## POST a JSON body".into(),
    );

    m.insert(
        "help.http.example3".into(),
        "http GET https://api.github.com/repos/rust-lang/rust -H \"Accept: application/vnd.github+json\" -q per_page=1 ## Header + query parameter".into(),
    );

    m.insert(
        "help.http.example4".into(),
        "http GET https://api.example.com/me -a user:pass -o me.json ## Basic auth, save body to a file".into(),
    );

    m.insert(
        "help.http.example5".into(),
        "http GET https://api.example.com/users -s | qur -s name ## Pipe the raw body into another command".into(),
    );
}
