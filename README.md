## Quick Demo (v1.8.0)

```bash
# Backup your project
ntc
bkup                              # Create backup
# ... make changes ...
pldw 1                            # Restore from backup
unpd                              # Undo the restore

# Search for files
fs main.c                         # Exact match
fs mian.c                         # Fuzzy match (suggests main.c)
fs test -d 5                      # Search 5 levels deep
ds src                            # Search directories

# Teleport with go/cd
tp add work ~/projects/myapp
go to work                        # Jump to savepoint

# View tree with custom depth and sizes
view -s -d 3

# Multi-argument alias
ral add runc(x,y) "gcc -o $y $x.c && ./$y"
runc(main,program)