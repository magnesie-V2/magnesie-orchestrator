# magnes-ie-orchestrateur

- Prend des photos en entrée
- Vérifies la disponibilité des clusteurs basés sur énergie verte
- Déploie le service de photog. sur le cluster retenu
- Demande au service de photoG. de procéder au traitement des photos, et les récupère en sortie
- (Envoie le résultat à une base de données de modèles 3d)

## Building with Docker
    docker build -t magnesie-orchestrator .

## Running  with Docker
```
    docker run \
    --rm \
    --name=magnesie-orchestrator \
    -i -t -d \
    -p 7878:7878 \
    magnesie-orchestrator
```

## Pinging the orchestrator
The service listens to the 7878 TCP and handles the following endpoints : 
- [GET] /photogrammetry/<job-id>

