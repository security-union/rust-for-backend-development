local_build:
	docker build -t rust_json_api_local_dev -f Dockerfile.localdevelopment .

local_run:
	docker run --rm -it -p 8000:8000 --mount type=bind,source="$(shell pwd)",target=/app --name rust_json rust_json_api_local_dev

cut_production_image:
	./cut_build_push_image.sh