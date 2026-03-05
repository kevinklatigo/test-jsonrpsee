# JSON-RPC Todo App — React + Rust (jsonrpsee)

A proof-of-concept exploring JSON-RPC 2.0 over HTTP with a **Rust backend** (`jsonrpsee` v0.26) and a **React frontend**. The backend supports two modes:

- **Standalone** — jsonrpsee's built-in HTTP server
- **Axum** — JSON-RPC handler mounted inside an axum router

## Running

### Standalone mode (default)

jsonrpsee serves JSON-RPC directly on the root path.

```bash
cd backend
cargo run
# Server at http://localhost:3000
```

### Axum mode

JSON-RPC is served at `/rpc`, with an additional `/health` endpoint.

```bash
cd backend
cargo run --no-default-features --features axum
# Server at http://localhost:3000
#   POST /rpc    — JSON-RPC endpoint
#   GET  /health — Health check
```

### Frontend

```bash
cd frontend
npm install
npm run dev
# Dev server at http://localhost:5173
```

By default the frontend sends RPC calls to `http://localhost:3000` (standalone mode). For axum mode, set:

```bash
VITE_RPC_URL=http://localhost:3000/rpc npm run dev
```

## Verification

```bash
# Test standalone mode
curl -s -X POST http://localhost:3000 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"todo_list","id":1}'
# → {"jsonrpc":"2.0","result":[],"id":1}

# Test axum mode
curl -s -X POST http://localhost:3000/rpc \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"todo_list","id":1}'
# → {"jsonrpc":"2.0","result":[],"id":1}

# Axum health check
curl http://localhost:3000/health
# → OK
```

## RPC Methods

| Method              | Params           | Returns                  |
| ------------------- | ---------------- | ------------------------ |
| `todo_list`         | none             | `Todo[]`                 |
| `todo_add`          | `{ text }`       | `Todo` (created)         |
| `todo_toggle`       | `{ id }`         | `Todo` (updated)         |
| `todo_remove`       | `{ id }`         | `bool`                   |
| `todo_clearCompleted` | none           | `u64` (count removed)    |
