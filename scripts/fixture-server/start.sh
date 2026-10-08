# 公式イメージの中で Misskey を起動する。
# `pnpm run migrateandstart` と同じ手順を、node で直接実行している。
# 古いイメージ（2024.10.1）では pnpm が起動せず、
# エントリポイントの場所もリリースによって違うためである。
set -e
cd /misskey/packages/backend
if [ -f scripts/compile_config.js ]; then node scripts/compile_config.js; fi
node node_modules/typeorm/cli.js migration:run -d ormconfig.js
if [ -f built/entry.js ]; then exec node built/entry.js; else exec node built/boot/entry.js; fi
