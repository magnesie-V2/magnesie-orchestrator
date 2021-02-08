# magnes-ie-orchestrateur

- Prend des photos en entrée
- Vérifies la disponibilité des clusteurs basés sur énergie verte
- Déploie le service de photog. sur le cluster retenu
- Demande au service de photoG. de procéder au traitement des photos, et les récupère en sortie
- (Envoie le résultat à une base de données de modèles 3d)

Pour tester le déploiement du service de photogrammetry sur l'orchestrateur :

```bash
cargo test launch_grid5000_client -- "username" "password" "site" "walltime" "ssh_pu_key_path" --nocapture
```
