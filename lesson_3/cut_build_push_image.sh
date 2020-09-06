# TODO: replace this script with your gcr repo.
IMAGE_URL=gcr.io/project-name/app-name:$(git rev-parse HEAD)

echo "Uploading new image to "$IMAGE_URL

yarn build

docker build -t rust_json_api_prod . -f Dockerfile.production

docker tag garage $IMAGE_URL

docker push $IMAGE_URL

echo "New image uploaded to "$IMAGE_URL
