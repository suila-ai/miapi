// Misskey のストリーミング API（WebSocket）につなぎ、受け取ったメッセージを JSON 行で標準出力に出す。
// scripts/update-fixtures.sh が、イベントの fixture の採取に使う。
//
//   node scripts/capture-stream.ts <サーバの origin> <接続の JSON>
//
// 接続の JSON は [{"token": "…" または null, "channels": [{"name": "main", "params": {}}, …]}, …]。
// 接続ごとに WebSocket を 1 本開き、チャンネルの id は "<接続の番号>-<チャンネルの番号>" にする。
// 全チャンネルの connected がそろったら {"ready": true} を出す。
// 受け取ったメッセージは {"channel": チャンネル名か null, "message": {"type", "body"}} で出す。
// 標準入力の JSON 行 {"connection": 番号, "send": {…}} は、その接続へそのまま送る。
// 標準入力が閉じたら、3 秒待ってから閉じる。
// サーバに閉じられたときと、WebSocket のエラーのときは、終了コード 1 で終わる。
import { createInterface } from "node:readline";

type ChannelSpec = { name: string; params: Record<string, unknown> };
type ConnectionSpec = { token: string | null; channels: ChannelSpec[] };

const [origin, specJson] = process.argv.slice(2);
if (origin === undefined || specJson === undefined) {
  console.error("使い方: node scripts/capture-stream.ts <origin> <接続の JSON>");
  process.exit(2);
}
const specs: ConnectionSpec[] = JSON.parse(specJson);
const expected = specs.reduce((count, spec) => count + spec.channels.length, 0);
let acked = 0;

function emit(value: unknown): void {
  process.stdout.write(`${JSON.stringify(value)}\n`);
}

const sockets = specs.map((spec, connection) => {
  const url = new URL("/streaming", origin.replace(/^http/, "ws"));
  if (spec.token !== null) {
    url.searchParams.set("i", spec.token);
  }
  const channelNames = new Map<string, string>();
  const ws = new WebSocket(url);
  ws.addEventListener("open", () => {
    spec.channels.forEach((channel, index) => {
      const id = `${connection}-${index}`;
      channelNames.set(id, channel.name);
      ws.send(
        JSON.stringify({
          type: "connect",
          body: { channel: channel.name, id, params: channel.params, pong: true },
        }),
      );
    });
  });
  ws.addEventListener("message", (event) => {
    const message = JSON.parse(String(event.data));
    if (message.type === "connected") {
      acked += 1;
      if (acked === expected) {
        emit({ ready: true });
      }
      return;
    }
    if (message.type === "channel") {
      const { id, type, body } = message.body;
      emit({ channel: channelNames.get(id) ?? null, message: { type, body } });
      return;
    }
    emit({ channel: null, message });
  });
  ws.addEventListener("error", () => {
    console.error(`connection ${connection}: WebSocket のエラー`);
    process.exit(1);
  });
  ws.addEventListener("close", () => {
    console.error(`connection ${connection}: サーバが接続を閉じた`);
    process.exit(1);
  });
  return ws;
});

const stdin = createInterface({ input: process.stdin });
stdin.on("line", (line) => {
  if (line.trim() === "") {
    return;
  }
  const command: { connection: number; send: unknown } = JSON.parse(line);
  sockets[command.connection]?.send(JSON.stringify(command.send));
});
stdin.on("close", () => {
  setTimeout(() => {
    for (const ws of sockets) {
      ws.close();
    }
    process.exit(0);
  }, 3000);
});
