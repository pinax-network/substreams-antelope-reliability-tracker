ENDPOINT ?= kylin.firehose.eosnation.io:9001

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s 12292922 -t +10

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_block -s 12292922 -t +10

.PHONY: prom_out
prom_out: build
	substreams gui -e $(ENDPOINT) substreams.yaml prom_out -s 12292922 -t +10

.PHONY: kv_out
kv_out: build
	substreams gui -e $(ENDPOINT) substreams.yaml kv_out -s 303006925 -t +100

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml
