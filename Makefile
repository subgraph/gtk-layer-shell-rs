GIR = gir/target/bin/gir
GIR_SRC != find gir/src -name '*.rs'
GIR_SRC += gir/Cargo.toml gir/Cargo.lock gir/build.rs
GIR_FILES = gir-files/GtkLayerShell-0.1.gir

gir: src/auto/mod.rs
	# Fixup enum imports for global functions since gir doesn't do it correctly
	sed -i 's/^use Edge/use crate::auto::enums::Edge/' src/auto/functions.rs
	sed -i 's/^use Layer/use crate::auto::enums::Layer/' src/auto/functions.rs
	cargo fmt

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init

