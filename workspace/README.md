# Install home-dev-container

The SSH server is exposed to the tailnet as a layer 3 Tailscale
`LoadBalancer` Service. A Kubernetes `Ingress` must not be used here because
it only proxies HTTP/HTTPS traffic.

For a first install:

```bash
kubectl create namespace home-dev-container
kubectl apply -n home-dev-container -f home-dev-container/deployment.yaml
```

If upgrading from the previous Ingress-based manifest, first remove the old
Ingress so it releases the `home-dev-container` Tailscale hostname:

```bash
kubectl delete ingress -n home-dev-container home-dev-container
kubectl apply -n home-dev-container -f home-dev-container/deployment.yaml
kubectl wait -n home-dev-container \
  --for=jsonpath='{.status.loadBalancer.ingress[0]}' \
  service/home-dev-container \
  --timeout=180s
```

Then connect on the SSH port exposed by the Service:

```bash
ssh remote@home-dev-container.taild7c8e5.ts.net
```

## Build and push an image with BuildKit

The development container uses the rootless BuildKit service running in the
cluster. Check the connection first:

```bash
buildctl debug workers
```

From a directory containing a `Dockerfile`, build the current directory and
push the resulting image directly to the internal Zot service:

```bash
buildctl build \
  --frontend dockerfile.v0 \
  --local context=. \
  --local dockerfile=. \
  --progress plain \
  --output \
    type=image,name=zot.zot.svc.cluster.local:5000/my-image:0.0.1,push=true
```

Replace `my-image:0.0.1` with the repository and tag to publish. Once pushed,
the same image is accessible from the tailnet through the public Zot hostname:

```text
zot.taild7c8e5.ts.net/my-image:0.0.1
```

`BUILDKIT_HOST` must be available in the SSH shell and point to:

```text
tcp://buildkitd.buildkit.svc.cluster.local:1234
```

## Workspace volume

The Kubernetes volume is mounted at the image-neutral path `/workspace`.
At startup, the image creates `~/workspace` as a symbolic link to that volume,
so the controller does not need to know the container user's home directory.

The behavior can be customized with:

- `WORKSPACE_VOLUME_PATH`, which defaults to `/workspace`;
- `WORKSPACE_LINK_NAME`, which defaults to `workspace`;
- `WORKSPACE_HOME_PATH`, which defaults to `$HOME/$WORKSPACE_LINK_NAME`.


```bash
buildctl \
  build \
  --frontend dockerfile.v0 \
  --local context=. \
  --local dockerfile=. \
  --output \
    type=image,name=zot.taild7c8e5.ts.net/home-dev-container:0.0.1,push=true
```
