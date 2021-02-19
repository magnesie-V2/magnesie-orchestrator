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
    -i -t [-d] \
    -p 7878:7878 \
    magnesie-orchestrator
```

## Pinging the orchestrator
The service listens to the 7878 TCP and handles the following endpoints : 
- [GET] /photogrammetry/<job-id>

## Configuration

* Modifier la clé d'API pour OpenWeatherMap dans config/open_weather_map.json
* Ajouter une paire de clé ssh au format pem nommée orchestrateur_key.pem et orchestrateur_key.pub dans le dossier config.
    * La clé publique doit être connue par le cloud provider (Grid5000 par exemple)
* Vérifier la liste des sites Grid5000 dans ressources/grid5000_sites.txt. Ajouter ceux qui manquent si besoin.

## Test Grid5000 deployment

To test the deployment of the photogrammetry service on Grid5000 :

```bash
cargo test launch_grid5000_client -- "username" "password" "walltime" --nocapture
```

