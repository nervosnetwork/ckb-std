TARGET := riscv64imac-unknown-none-elf
DOCKER_IMAGE := jjy0/ckb-capsule-recipe-rust:2020-9-28
CC := riscv64-unknown-elf-gcc

default: integration-in-docker

fix-permission-in-docker:
	chown -R $$OWNER target; chown -R $$OWNER docs; chown -R $$OWNER $$HOME/.cargo/git; chown -R $$OWNER $$HOME/.cargo/registry;

integration-in-docker: test-shared-lib
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code ${DOCKER_IMAGE} bash -c 'make integration; CODE=$$?; make fix-permission-in-docker; exit $$CODE'

publish-crate:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/credentials:/root/.cargo/credentials -w/code ${DOCKER_IMAGE} bash -c 'cargo publish --target ${TARGET}; make fix-permission-in-docker'

generate-doc:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code ${DOCKER_IMAGE} bash -c 'cargo doc --target ${TARGET} --target-dir docs; make fix-permission-in-docker'

publish-doc:
	git checkout gh-page
	git reset --hard master
	make generate-doc
	git add .
	git commit -m "update doc" || true
	git push -f upstream
	git checkout master
	echo "done"

publish: publish-crate publish-doc

clean:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/credentials:/root/.cargo/credentials -w/code ${DOCKER_IMAGE} bash -c 'cargo clean; make -C test clean'

test-shared-lib:
	make -C test/shared-lib all-via-docker

integration: check test

test:
	make -C test test

check:
	cargo check --target ${TARGET} --examples

install-tools:
	cargo install --git https://github.com/xxuejie/ckb-binary-patcher.git

.PHONY: test check

