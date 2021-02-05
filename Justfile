features := ''
target := `drone print target 2>/dev/null || echo ""`

# Install dependencies
deps:
	type cargo-readme >/dev/null || cargo +stable install cargo-readme
	type drone >/dev/null || cargo install drone
	rustup target add $(drone print target)
	rustup component add clippy
	rustup component add rustfmt

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	cargo clippy --package drone-riscv-macros \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo clippy --package drone-riscv --features "{{features}}"

# Build the documentation
doc:
	cargo doc --package drone-riscv-macros \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo doc --package drone-riscv --features "{{features}}"

# Open the documentation in a browser
doc-open: doc
	cargo doc --package drone-riscv --features "{{features}}" --open

# Run the tests
test:
	cargo test --package drone-riscv-macros \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo test --package drone-riscv --features "{{features}} std" \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')

# Update README.md
readme:
	cargo readme -o README.md

# Bump the versions
version-bump version drone-core-version:
	sed -i "s/\(api\.drone-os\.com\/drone-riscv\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo {{version}} | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
		Cargo.toml macros/Cargo.toml src/lib.rs
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[package\]/version = "{{version}}"/;t;x}' \
		Cargo.toml macros/Cargo.toml
	sed -i '/\[.*\]/h;/version = "=.*"/{x;s/\[.*drone-riscv-.*\]/version = "={{version}}"/;t;x}' \
		Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone\(-macros\)\?-core\]/version = "{{drone-core-version}}"/;t;x}' \
		Cargo.toml macros/Cargo.toml
	sed -i 's/\(drone-riscv.*\)version = "[^"]\+"/\1version = "{{version}}"/' \
		src/lib.rs

# Publish to crates.io
publish:
	cd macros && cargo publish \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	sleep 30
	cargo publish --features "{{features}}"

# Publish the docs to api.drone-os.com
publish-doc: doc
	dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
		&& rm -rf ../drone-api/$dir \
		&& cp -rT target/doc ../drone-api/$dir \
		&& cp -rT target/{{target}}/doc ../drone-api/$dir \
		&& echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_riscv">' > ../drone-api/$dir/index.html \
		&& cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
