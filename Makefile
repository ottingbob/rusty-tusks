
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

# Get the registry from an environment variable
HELM_SECRET_UPGRADE := helm secrets upgrade --install
RUSTY_TUSKS_TAG := v0.0.1
RUSTY_TUSKS_IMAGE := rusty-tusks:$(RUSTY_TUSKS_TAG)

# IMPORTANT: Need to define a `REMOTE_REGISTRY` if you are going to use this make target
# sudo make docker-push REMOTE_REGISTRY=my-registry.website.com
.PHONY: docker-push
docker-push: docker-build
	docker tag $(RUSTY_TUSKS_IMAGE) $$REMOTE_REGISTRY/$(RUSTY_TUSKS_IMAGE)
	docker push $$REMOTE_REGISTRY/$(RUSTY_TUSKS_IMAGE)

.PHONY: docker-build
docker-build:
	docker build -t $(RUSTY_TUSKS_IMAGE) . -f Dockerfile

.PHONY: docker-run
docker-run:
	# FIXME: I think this script still has some issues communicating with cluster
	# 	via kube config while running in the docker container...
	# 	Might need to mount the kube config but just havent gotten there...
	# TODO: Need to double check which ports are needed to be exposed on this...
	# 	Might need some ENV vars in order to connect to the k8s cluster
	docker run \
		-p 8000:8000 \
		--name rusty-tusks \
		--rm \
		$(RUSTY_TUSKS_IMAGE)

