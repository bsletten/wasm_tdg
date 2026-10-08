# Requires the Emscripten SDK (emcc) on your PATH.
CFLAGS = -pthread -sPTHREAD_POOL_SIZE=4 -sINITIAL_MEMORY=134217728 -O2

threads.js threads.wasm: threads.c Makefile
	emcc threads.c -o threads.js $(CFLAGS)

.PHONY: run serve clean
run: threads.js
	node threads.js

serve: threads.js
	python3 server.py

clean:
	rm -f threads.js threads.wasm threads.worker.js
