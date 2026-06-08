## Quick Demo (v2.1.0)

```bash
# Math expression evaluator
math 3+4*5                        # → 23
math sin(PI/2)                    # → 1
math print("hello world")         # hello world
math script.ntc.math              # Run .ntc.math file

# Dinosaur runner game (when bored)
dino                              # Press space to jump, avoid obstacles

# Export/Import run aliases
ral export --all myaliases        # Save all aliases to .ntc.ral
ral import myaliases.ntc.ral      # Restore aliases

# Export/Import ignore/care settings
igcare export --all myproject     # Save all filter rules to .ntc.igcare
igcare import myproject.ntc.igcare # Restore filters

# Built-in text editor with math auto-completion
ntc
ne math_file.ntc.math              # Open .ntc.math file
# Type "si" → auto-complete to "sin", "sqrt", etc.

# Backup diff
bkup
# ... make changes ...
diff 1                            # Show diff vs backup #1

# PDF / DOCX / XLSX reports
pdf
docx
xlsx

# Color toggle
setc OFF                          # Disable color output

# Multi-argument alias
ral add runc(x,y) "gcc -o $y $x.c && ./$y"
runc(main,program)
```
