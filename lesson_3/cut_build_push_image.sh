# TODO: replace this script with your gcr repo.
IMAGE_URL=gcr.io/project-name/rust_json_api_prod:$(git rev-parse HEAD)

echo "Uploading new image to "$IMAGE_URL

docker build -t rust_json_api_prod . -f Dockerfile.production

docker tag rust_json_api_prod $IMAGE_URL

docker push $IMAGE_URL

echo "New image uploaded to "$IMAGE_URL
