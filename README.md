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

## Configuration for Grid5000

* Edit the OpenWeatherMap API key in config/open_weather_map.json
* Add a SSH key pair in pem format named orchestrateur_key.pem and orchestrateur_key.pubin the config folder.
    * The public key must be known by Grid5000
* Check the list of Grid5000 sites in ressources/grid5000_sites.txt. Add any missing ones if needed.

## Test Grid5000 deployment

To test the deployment of the photogrammetry service on Grid5000 :

```bash
cargo test launch_grid5000_client -- "username" "password" "site" "walltime" --nocapture
```

