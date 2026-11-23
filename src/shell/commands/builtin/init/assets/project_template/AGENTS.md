# Agent guide

## Project

{{name}} — describe the project in one sentence.

## Conventions

- Read the surrounding code before editing.
- Match existing naming, formatting and structure.
- Keep changes small and focused.
- Run the project checks before reporting completion.

## Commands

```bash
ntc ran        # default task
ntc ran list   # available tasks
ntc todo       # open tasks
```

## Notes

- Never commit secrets. Local secrets live in `.env` (gitignored).
- SMTP credentials for `ntc smtp` live in `ntc_smtp.json` (gitignored).
