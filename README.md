# kubedev-controller

## Visualiser les traces avec Jaeger

Pour lancer une instance locale de Jaeger v2 avec un stockage en mémoire :

```bash
docker run --rm --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  cr.jaegertracing.io/jaegertracing/jaeger:2.22.0
```

L'interface Jaeger est disponible sur <http://localhost:16686>. Le contrôleur
exporte par défaut ses traces en OTLP/gRPC vers `http://localhost:4317`.

Pour utiliser un autre endpoint :

```bash
OPENTELEMETRY_ENDPOINT_URL=http://jaeger.example:4317 cargo run
```
