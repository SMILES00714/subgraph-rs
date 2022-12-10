#!/usr/bin/env bash

set -e

ROOT=$(dirname "${BASH_SOURCE[0]}")

IPFS="https://api.thegraph.com/ipfs/"
NODE="https://api.studio.thegraph.com/deploy/"

BUILD=release

log() {
	1>&2 echo "==> $@"
}

cargo_build() {
	log "building mapping..."
	local rel
	if [[ $BUILD != debug ]]; then
		rel=--release
	fi

	cargo build -p graph-token $rel --target wasm32-unknown-unknown
}

ipfs_add() {
	local fn="$(basename "$1")"
	log "adding '$fn' to IPFS..."

	local result
	if [[ -z "$2" ]]; then
		result=$(curl -s -F "file=@$ROOT/$1;filename=$fn" "$IPFS/api/v0/add")
	else
		result=$(curl -s -F "file=@-;filename=$fn" "$IPFS/api/v0/add" <<< "$2")
	fi

	local hash=$(echo "$result" | jq -r '.Hash' | head -n 1)

	log "pinning $hash..."
	curl -s -X POST "$IPFS/api/v0/pin/add?arg=$hash" > /dev/null

	echo $hash
}

subgraph_compile() {
	cat "$ROOT/subgraph.yaml" \
		| sed 's/\${schema.graphql}/'$(ipfs_add "schema.graphql")'/' \
		| sed 's/\${abis\/GraphToken.json}/'$(ipfs_add "abis/GraphToken.json")'/' \
		| sed 's/\${graph_token.wasm}/'$(ipfs_add "../../target/wasm32-unknown-unknown/$BUILD/graph_token.wasm")'/'
}

subgraph_upload() {
	ipfs_add "subgraph.yaml" "$(subgraph_compile)"
}

subgraph_deploy() {
	local hash="$(subgraph_upload)"

	curl -s -X POST $NODE \
		-H "Authorization: Bearer ${GRAPH_STUDIO_KEY}" \
		-H "Content-Type: application/json; charset=utf-8" \
		--data @- <<JSON
{
	"jsonrpc": "2.0",
	"method": "subgraph_deploy",
	"params": {
		"name": "graph-token",
		"ipfs_hash": "$hash",
		"version_label": "v0.0.38"
	},
	"id": "2"
}
JSON
}

cargo_build
subgraph_deploy
