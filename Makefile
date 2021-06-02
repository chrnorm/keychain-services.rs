target/doc/keychain_services:
	cargo rustdoc

docs: target/doc/keychain_services
	-git branch -D gh-pages
	git checkout --orphan gh-pages
	git reset README.md
	git reset --hard
	cp -r target/doc/* .
	cp -r keychain_services docs
	rm -rf target
	echo 'keychain-services.rs' > CNAME
	git add .
	git commit -m "Generate docs using 'make docs'"
	@echo "Use 'git push -f origin gh-pages' to deploy"

build: build-rust sign

build-rust:
	cargo build --example secure_enclave

sign: build-rust
	codesign  -s "Developer ID Application: Common Fate Technologies Pty Ltd" -f --identifier "VXC4RPMSUJ.com.chrisnorman.sekey" --entitlements ./assets/sekey.entitlements --timestamp=none ./target/debug/examples/secure_enclave

run:
	./target/debug/examples/secure_enclave