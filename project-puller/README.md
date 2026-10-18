# Project puller

This image clones or updates a Git repository in the workspace PVC. The
controller only supplies configuration through environment variables.

## Build and publish

```bash
docker build -t hugo08/project-puller:0.0.1 ./project-puller
docker push hugo08/project-puller:0.0.1
```

With the in-cluster BuildKit service:

```bash
buildctl build \
  --frontend dockerfile.v0 \
  --local context=./project-puller \
  --local dockerfile=./project-puller \
  --progress plain \
  --output type=image,name=docker.io/hugo08/project-puller:0.0.1,push=true
```

## Configuration

- `REPOSITORY_URL` is required.
- `REVISION` defaults to `main`.
- `TARGET_DIR` defaults to `/workspace/repository`.
- `WORKSPACE_UID` and `WORKSPACE_GID` default to `1000`.
