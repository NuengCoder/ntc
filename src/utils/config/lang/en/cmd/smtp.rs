use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.smtp.build_fail".into(),
        "Failed to build email message".into(),
    );

    m.insert(
        "cmd.smtp.create_one_hint".into(),
        "  Create one: {} -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert(
        "cmd.smtp.create_syntax".into(),
        "smtp -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert("cmd.smtp.create_usage".into(), "Server, username, and password are required. Usage: smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.created_label".into(), "Created:".into());

    m.insert(
        "cmd.smtp.decode_failed".into(),
        "Failed to decode stored password".into(),
    );

    m.insert(
        "cmd.smtp.body_file_missing".into(),
        "Body file not found: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_html_from_file".into(),
        "HTML body from file: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_md_from_file".into(),
        "Markdown body from file: {} (rendered to HTML)".into(),
    );

    m.insert(
        "cmd.smtp.body_read_fail".into(),
        "Failed to read body file {}: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_stdin_empty".into(),
        "No body received on stdin".into(),
    );

    m.insert(
        "cmd.smtp.body_text_from_file".into(),
        "Text body from file: {}".into(),
    );

    m.insert(
        "cmd.smtp.help_body_file".into(),
        "  -b letter.html | letter.md       body from a file (Markdown is rendered to HTML)".into(),
    );

    m.insert(
        "cmd.smtp.help_body_format".into(),
        "  --body-format auto|text|html|md  force how -b is interpreted".into(),
    );

    m.insert(
        "cmd.smtp.help_body_inline".into(),
        "  -b '<p style=text-align:center>Hi</p>'   inline HTML is detected automatically".into(),
    );

    m.insert(
        "cmd.smtp.help_body_stdin".into(),
        "  -b -                            read the body from stdin".into(),
    );

    m.insert("cmd.smtp.help_body_title".into(), "Message body:".into());

    m.insert(
        "cmd.smtp.invalid_body_format".into(),
        "Unknown body format '{}' (use auto, text, html or md)".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_file".into(),
        "Missing path after --body-file".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_format".into(),
        "Missing value after --body-format".into(),
    );

    m.insert(
        "cmd.smtp.help_attachments_desc".into(),
        "  -f accepts comma-separated file paths and directories (files are expanded recursively)"
            .into(),
    );

    m.insert("cmd.smtp.help_attachments_example".into(), "  Example: smtp --id a1b2 --to user@test.com -s \\\"Report\\\" -b \\\"See attached\\\" -f \\\"./report.pdf,./images/\\\"".into());

    m.insert(
        "cmd.smtp.help_attachments_title".into(),
        "Attachments:".into(),
    );

    m.insert(
        "cmd.smtp.help_create_example".into(),
        "  Example: smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123".into(),
    );

    m.insert(
        "cmd.smtp.help_create_syntax".into(),
        "  smtp -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert(
        "cmd.smtp.help_create_title".into(),
        "Create a profile:".into(),
    );

    m.insert(
        "cmd.smtp.help_info_syntax".into(),
        "  smtp --id-info <id>".into(),
    );

    m.insert(
        "cmd.smtp.help_info_title".into(),
        "Show profile info:".into(),
    );

    m.insert("cmd.smtp.help_list_syntax".into(), "  smtp --id-ls".into());

    m.insert("cmd.smtp.help_list_title".into(), "List profiles:".into());

    m.insert(
        "cmd.smtp.help_port_25".into(),
        "  25   Plain (if supported by server)".into(),
    );

    m.insert("cmd.smtp.help_port_465".into(), "  465  TLS (SMTPS)".into());

    m.insert(
        "cmd.smtp.help_port_587".into(),
        "  587  STARTTLS (default)".into(),
    );

    m.insert("cmd.smtp.help_ports_title".into(), "Ports:".into());

    m.insert(
        "cmd.smtp.help_rm_syntax".into(),
        "  smtp --id-rm <id>".into(),
    );

    m.insert("cmd.smtp.help_rm_title".into(), "Delete a profile:".into());

    m.insert("cmd.smtp.help_send_syntax".into(), "  smtp --id <id> --to <email> [--cc <email>] [--bcc <email>] -s <subject> -b <body> [-f <files>]".into());

    m.insert("cmd.smtp.help_send_title".into(), "Send email:".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP Email Client".into());

    m.insert("cmd.smtp.id_label".into(), "ID:".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "Invalid BCC: {}".into());

    m.insert("cmd.smtp.invalid_cc".into(), "Invalid CC: {}".into());

    m.insert("cmd.smtp.invalid_port".into(), "Invalid port number".into());

    m.insert(
        "cmd.smtp.invalid_pwd_encoding".into(),
        "Invalid password encoding: {}".into(),
    );

    m.insert(
        "cmd.smtp.invalid_recipient".into(),
        "Invalid recipient: {}".into(),
    );

    m.insert(
        "cmd.smtp.invalid_sender".into(),
        "Invalid sender email address".into(),
    );

    m.insert(
        "cmd.smtp.missing_body".into(),
        "Missing body after -b".into(),
    );

    m.insert("cmd.smtp.missing_id".into(), "Missing ID after --id".into());

    m.insert(
        "cmd.smtp.missing_id_info".into(),
        "Missing ID after --id-info".into(),
    );

    m.insert(
        "cmd.smtp.missing_id_rm".into(),
        "Missing ID after --id-rm".into(),
    );

    m.insert(
        "cmd.smtp.missing_password".into(),
        "Missing password after -pwd".into(),
    );

    m.insert(
        "cmd.smtp.missing_path".into(),
        "Missing path after -f".into(),
    );

    m.insert(
        "cmd.smtp.missing_port".into(),
        "Missing port after -p".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_bcc".into(),
        "Missing recipient after --bcc".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_cc".into(),
        "Missing recipient after --cc".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_to".into(),
        "Missing recipient after --to".into(),
    );

    m.insert(
        "cmd.smtp.missing_server".into(),
        "Missing server after -i".into(),
    );

    m.insert(
        "cmd.smtp.missing_subject".into(),
        "Missing subject after -s".into(),
    );

    m.insert(
        "cmd.smtp.missing_user".into(),
        "Missing username after -u".into(),
    );

    m.insert(
        "cmd.smtp.need_recipient".into(),
        "Recipient required. Use --to <email>".into(),
    );

    m.insert(
        "cmd.smtp.need_subject".into(),
        "Subject required. Use -s <subject>".into(),
    );

    m.insert(
        "cmd.smtp.no_files_in_dir".into(),
        "No files found in directory: {}".into(),
    );

    m.insert(
        "cmd.smtp.no_profiles".into(),
        "  {}  No SMTP profiles found.".into(),
    );

    m.insert("cmd.smtp.password_label".into(), "Password:".into());

    m.insert(
        "cmd.smtp.path_not_found".into(),
        "Path not found: {}".into(),
    );

    m.insert("cmd.smtp.port_label".into(), "Port:".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP Profile:".into());

    m.insert(
        "cmd.smtp.profile_not_found".into(),
        "Profile not found: {}".into(),
    );

    m.insert(
        "cmd.smtp.profile_saved".into(),
        "SMTP profile saved: {}".into(),
    );

    m.insert("cmd.smtp.profiles_title".into(), "SMTP Profiles:".into());

    m.insert(
        "cmd.smtp.read_attach_fail".into(),
        "Failed to read attachment: {}".into(),
    );

    m.insert(
        "cmd.smtp.removed_profile".into(),
        "Removed SMTP profile: {}".into(),
    );

    m.insert(
        "cmd.smtp.send_fail".into(),
        "Failed to send email: {}".into(),
    );

    m.insert(
        "cmd.smtp.send_ok".into(),
        "Email sent successfully{}".into(),
    );

    m.insert("cmd.smtp.sending".into(), "Sending email to {}...".into());

    m.insert("cmd.smtp.server_label".into(), "Server:".into());

    m.insert(
        "cmd.smtp.skipping_exec".into(),
        "Skipping executable file: {}".into(),
    );

    m.insert(
        "cmd.smtp.skipping_not_file".into(),
        "Skipping (not a file or directory): {}".into(),
    );

    m.insert(
        "cmd.smtp.transport_fail".into(),
        "Failed to configure SMTP transport: {}".into(),
    );

    m.insert(
        "cmd.smtp.unknown_option".into(),
        "Unknown option: {}".into(),
    );

    m.insert("cmd.smtp.user_label".into(), "User:".into());

    m.insert(
        "cmd.smtp.with_attachments".into(),
        " ({} attachment{})".into(),
    );

    m.insert(
        "cmd.smtp.help_info_deadlock".into(),
        "  smtp --id-info <id> --deadlock   also show the app password (asks the master password)"
            .into(),
    );

    m.insert(
        "cmd.smtp.password_hidden".into(),
        "******** (hidden - use --deadlock)".into(),
    );
}
