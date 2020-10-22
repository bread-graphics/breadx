# MIT/Apache2 License
# Used to regenerate the xml definition derived from xcbgen

CARGO = cargo
GENR = $(PWD)/generator/target/debug/flutterbug_generator
GENR_SRC = $(PWD)/generator/src
XML = $(PWD)/xml
OUTPUT = $(PWD)/src/auto
RUSTFMT = rustfmt

#AUTOS = $(foreach f,$(notdir $(wildcard $(XML)/*.xml)),$(OUTPUT)/$(f:.xml=.rs))
AUTOS = $(OUTPUT)/xproto.rs $(OUTPUT)/xc_misc.rs

autos: $(AUTOS) 

GENR_DEPS := $(wildcard $(GENR_SRC)/**/*) $(wildcard $(GENR_SRC)/*)

$(GENR): $(GENR_DEPS) 
	cd $(PWD)/generator; $(CARGO) build

$(OUTPUT)/%.rs: $(XML)/%.xml $(GENR)
	$(GENR) $< $@
	$(RUSTFMT) $@
