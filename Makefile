TARGET := riscv64imac-unknown-none-elf
DOCKER_IMAGE := jjy0/ckb-capsule-recipe-rust:2020-5-9
CC := riscv64-unknown-elf-gcc

default: integration-in-docker

integration-in-docker:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo:/root/.cargo ${DOCKER_IMAGE} bash -c 'cd /code && make integration; chown -R $$OWNER target'

publish-in-docker:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo:/root/.cargo ${DOCKER_IMAGE} bash -c 'cd /code && cargo publish --target ${TARGET}; chown -R $$OWNER target'

doc:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo:/root/.cargo ${DOCKER_IMAGE} bash -c 'cd /code && cargo doc --target ${TARGET} --target-dir docs; chown -R $$OWNER docs'

integration: check test

test:
	make -C test test

check:
	cargo check --target ${TARGET} --examples

install-tools:
	cargo install --git https://github.com/xxuejie/ckb-binary-patcher.git

.PHONY: test check
