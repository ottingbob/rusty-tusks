
.PHONY: view-crd
view-crd:
	RUST_LOG=walrus=info,krator=info cargo run --features=derive -- --output-crd | batcat -l yaml

.PHONY: run-operator
run-operator:
	RUST_BACKTRACE=1 \
		KUBERNETES_SERVICE_HOST=basecamp \
		KUBERNETES_SERVICE_PORT=6443 \
		RUST_LOG=info \
		ENABLE_WEBHOOKS=false \
		cargo run --features=derive 
