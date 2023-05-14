#!/bin/sh

echo "Connect to storage"
mc config host add docker_minio $STORAGE_URL $STORAGE_ROOT_USER $STORAGE_ROOT_PASSWORD

echo "Create bucket 'rusvid-media'"
mc mb docker_minio/rusvid-media
mc anonymous set public docker_minio/rusvid-media

echo "Create policy 'rusvid_access'"
mc admin policy create docker_minio rusvid_access ./minio_policy.json

echo "Add new user, add policy"
mc admin user add docker_minio $STORAGE_ACCESS_KEY $STORAGE_SECRET_KEY
mc admin policy attach docker_minio rusvid_access --user $STORAGE_ACCESS_KEY
