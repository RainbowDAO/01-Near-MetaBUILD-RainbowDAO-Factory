help:
	@echo "\
	develop help \n\
	create :  create a new Contract .try command  [make create name=test]\n\
	build  :  build an exit Contract. try command  [make build  name=test]\n\
	";

create:
	cargo contract new  $(name);
	@echo "[profile.release]\n overflow-checks = false" >> $(name)/Cargo.toml

build:
	cd $(name) && cargo +nightly contract build && \
	mv target/ink/metadata.json target/ink/$(name).wasm target/ink/$(name).contract target  \
  	&& rm -r target/ink

