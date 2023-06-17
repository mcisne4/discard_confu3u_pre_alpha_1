# Watch: 'rs_dev'
dev: build
  cd rs_dev && cargo watch -x run

# ===================== #
# === BUILD SCRIPTS === #
# ===================== #

# Build: All
build:
  cargo build

# Build: 'rs_dev'
build-dev:
  cargo build -p rs_dev

# ====================== #
# === EXPAND SCRIPTS === #
# ====================== #

# Expand: All
expand:
  cd rs_dev && cargo expand

expand-dev:
  cd rs_dev && cargo expand

# =========================== #
# === MODULE TREE SCRIPTS === #
# =========================== #

# Module Tree: All
mods: mods-dev

# Module Tree: 'rs_dev'
mods-dev:
  cargo-modules generate tree -p rs_dev --types --traits --fns
