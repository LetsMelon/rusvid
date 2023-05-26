.PHONY: docker_build_all docker_build_server

docker_build_all:
	DOCKER_BUILDKIT=1 docker build -t rusvid:latest . --progress=plain

docker_build_server:
	DOCKER_BUILDKIT=1 docker build -t rusvid_server:latest -f ./Dockerfile.server . --progress=plain
