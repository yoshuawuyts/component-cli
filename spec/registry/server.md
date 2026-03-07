# Registry Server

r[server.health]
The `/v1/health` endpoint MUST return `200 OK` with `{"status": "ok"}`.

## Package Indexing

r[server.index.dependencies]
The indexer MUST extract WIT dependencies from each indexed package version by
pulling the wasm layer and parsing its WIT metadata. The extracted dependency
graph MUST be stored in the local database so that the `/v1/packages` endpoint
can include dependency information in its responses.
