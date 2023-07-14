vite_watch:
	./node_modules/vite/bin/vite.js

vite_build:
	./node_modules/vite/bin/vite.js build

build_proto:
	mkdir -p src/proto
	protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_opt=outputClientImpl=grpc-web --ts_proto_out=src/proto/. actor.proto -I ./../../easyact/proto/