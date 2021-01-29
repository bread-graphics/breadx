# MIT/Apache2 License
# Used to regenerate the xml definition derived from xcbgen

CARGO = cargo
GENR = $(PWD)/generator/target/debug/breadx_generator
GENR_SRC = $(PWD)/generator/src
KEYSYM = $(PWD)/keysym/target/debug/keysym
KEYSYM_SRC = $(PWD)/keysym/src
XML = $(PWD)/xml
OUTPUT = $(PWD)/src/auto
RUSTFMT = rustfmt

GENR_DEPS := $(wildcard $(GENR_SRC)/**/*) $(wildcard $(GENR_SRC)/*)
KEYSYM_DEPS := $(wildcard $(KEYSYM_SRC)/**/*) $(wildcard $(KEYSYM_SRC)/*)

AUTOS = $(foreach f,$(notdir $(wildcard $(XML)/*.xml)),$(OUTPUT)/$(f:.xml=.rs)) $(PWD)/src/keyboard/convert.rs

autos: $(AUTOS) 

$(GENR): $(GENR_DEPS) 
	cd $(PWD)/generator; $(CARGO) build

$(KEYSYM): $(KEYSYM_DEPS)
	cd $(PWD)/keysym; $(CARGO) build

$(OUTPUT)/%.rs: $(XML)/%.xml $(GENR)
	RUST_BACKTRACE=1 $(GENR) $< $@
	$(RUSTFMT) $@

$(PWD)/src/keyboard/convert.rs: $(PWD)/keysym/keysyms.json $(KEYSYM)
	RUST_BACKTRACE=1 $(KEYSYM) $(PWD)/keysym/keysyms.json $(PWD)/src/keyboard/convert.rs
	$(RUSTFMT) $(PWD)/src/keyboard/convert.rs
