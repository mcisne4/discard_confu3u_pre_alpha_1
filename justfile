# Watch: 'rs_dev'
dev: build
  cd rs_dev && cargo watch -x run

# ===================== #
# === BUILD SCRIPTS === #
# ===================== #

# Build: 'rs_dev' and all dependency crates
build: build-logs && build-dev

# Build: All
build-all:
  cargo build

# Build: 'rs_dev'
build-dev: build-logs
  cargo build -p rs_dev

# Build: 'rs_logs' and 'rs_logs_macro'
build-logs:
  cargo build -p rs_logs_macro
  cargo build -p rs_logs

# ====================== #
# === EXPAND SCRIPTS === #
# ====================== #

# Expand: All
expand:
  cd rs_dev && cargo expand

# Expand: 'rs_dev'
expand-dev:
  cd rs_dev && cargo expand

# Expand: 'rs_logs'
expand-logs:
  cd rs_logs && cargo expand

# =========================== #
# === MODULE TREE SCRIPTS === #
# =========================== #

# Module Tree: All
mods: mods-dev && mods-logs mods-logs-macro

# Module Tree: 'rs_dev'
mods-dev:
  cargo-modules generate tree -p rs_dev --types --traits --fns

# Module Tree: 'rs_logs'
mods-logs:
  cargo-modules generate tree -p rs_logs --types --traits --fns

# Module Tree: 'rs_logs_macro'
mods-logs-macro:
  cargo-modules generate tree -p rs_logs_macro --types --traits --fns
