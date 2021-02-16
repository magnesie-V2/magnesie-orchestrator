# magnes-ie-orchestrateur

- Prend des photos en entrée
- Vérifies la disponibilité des clusteurs basés sur énergie verte
- Déploie le service de photog. sur le cluster retenu
- Demande au service de photoG. de procéder au traitement des photos, et les récupère en sortie
- (Envoie le résultat à une base de données de modèles 3d)

## Configuration

* Modifier la clé d'API pour OpenWeatherMap dans config/open_weather_map.json
* Ajouter une paire de clé ssh au format pem nommée orchestrateur.pem et orchestrateur.pub dans config.
    * La clé publique doit être connue par le cloud provider (Grid5000 par exemple)
* Vérifier la liste des sites Grid5000 dans ressources/grid5000_sites.txt. Ajouter ceux qui manquent si besoin.

## Déploiement du service de photogrammétrie

Pour tester le déploiement du service de photogrammétrie sur l'orchestrateur :

```bash
cargo test launch_grid5000_client -- "username" "password" "site" "walltime" "ssh_pub_key_path" --nocapture
```

