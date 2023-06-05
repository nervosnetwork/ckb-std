.PHONY: gen check-moleculec-version install-tools clear
MOLC := moleculec
MOLC_VERSION := 0.7.2
GEN_MOL_IN_DIR := schemas
GEN_MOL_OUT_DIR := src/generated
GEN_MOL_FILES := ${GEN_MOL_OUT_DIR}/blockchain.rs

default: check-moleculec-version

gen: check-moleculec-version ${GEN_MOL_FILES} # Generate Files

clear:
	rm ${GEN_MOL_FILES} # Remove Generate Files

check-moleculec-version:
	test "$$(${MOLC} --version | awk '{ print $$2 }' | tr -d ' ')" = ${MOLC_VERSION}

install-tools:
	test "$$(${MOLC} --version)" == "Moleculec ${MOLC_VERSION}" || \
		cargo install --force "${MOLC}" --version ${MOLC_VERSION}

${GEN_MOL_OUT_DIR}/blockchain.rs: ${GEN_MOL_IN_DIR}/blockchain.mol
	${MOLC} --language rust --schema-file $< | rustfmt > $@
