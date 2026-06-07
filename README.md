## Quick Demo (v2.0.0)

```bash
# Built-in text editor (ntcEditor)
ntc
ne myfile.txt                     # Open file in built-in editor
ne --init main.rs                 # Create file with template

# Search and navigate
fgo main.c                        # Search files, pick one, navigate to its parent
fsc main.c                        # Search files, pick one, display its contents
locate test                       # Combined file + directory search

# Backup diff
bkup
# ... make changes ...
diff 1                            # Show diff vs backup #1

# New report formats
pdf                               # Generate PDF report
docx                              # Generate DOCX report
xlsx                              # Generate XLSX report

# Color toggle
setc OFF                          # Disable color output

# Multi-argument alias
ral add runc(x,y) "gcc -o $y $x.c && ./$y"
runc(main,program)
```
