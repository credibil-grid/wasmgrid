# dynamically target Makefile.toml
.PHONY: %
%:
	@cargo make $@