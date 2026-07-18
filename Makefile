.PHONY: build-web serve clean help

BROWSER_PORT ?= 8080

help: ## Show available commands
	@echo "Snake Web Build Pipeline"
	@echo ""
	@echo "Commands:"
	@echo "  build-web   Build the game to WebAssembly (wasm-pack build --target web --out-dir pkg)"
	@echo "  serve       Start HTTP server on port $(BROWSER_PORT) serving the web/ directory"
	@echo "  clean       Remove build artifacts (pkg/, wasm32 build output)"
	@echo "  help        Show this help message"

build-web: ## Build the game to WebAssembly
	@echo "Building Snake game for Web..."
	@rustup target add wasm32-unknown-unknown 2>/dev/null
	@cargo build --release --target wasm32-unknown-unknown
	@wasm-bindgen --no-typescript --target web \
		--out-dir ./web/pkg \
		--out-name "slivovy_snake" \
		./target/wasm32-unknown-unknown/release/slivovy_snake.wasm
	@echo ""
	@echo "Build complete! Output in web/pkg/:"
	@ls -lh web/pkg/
	@echo ""
	@echo "Open web/index.html in your browser, or run 'make serve' to start an HTTP server."

serve: ## Start HTTP server on port $(BROWSER_PORT)
	@echo "Starting HTTP server on port $(BROWSER_PORT)..."
	@echo "Open http://localhost:$(BROWSER_PORT) in your browser"
	@echo ""
	@if command -v python3 >/dev/null 2>&1; then \
		python3 -m http.server $(BROWSER_PORT) --directory web; \
	elif command -v node >/dev/null 2>&1; then \
		npx --prefer-offline http-server -a 0.0.0.0 -p $(BROWSER_PORT) ./web; \
	else \
		echo "No HTTP server found. Install one of:"; \
		echo "  - python3 (recommended)"; \
		echo "  - npx serve"; \
		echo "  - basic-http-server"; \
		echo ""; \
		echo "Then run:"; \
		echo "  python3 -m http.server $(BROWSER_PORT) --directory web"; \
	fi

clean: ## Remove build artifacts
	@echo "Cleaning build artifacts..."
	@rm -rf pkg
	@rm -rf target/wasm32-unknown-unknown/release
	@echo "Done."
