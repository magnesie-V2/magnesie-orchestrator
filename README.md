# magnes-ie-orchestrateur

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

**For this branch, you need a photogrammetry service on the same machine as the orchestrator on the port 7879 !**

## Test with mocked services
The orchestrator can run with mocked photogrammetry, image db and result db services. See the [mocks](https://github.com/magnesie/mocks) repository to learn how to deploy those mocks.

Once the mocks are deployed, execute the following command :

```bash
cargo run
```

The orchestrator will start running and dispatching the submissions created by the image db mock to the mocked photogrammetry service, which will then return fake addresses of falsely generated 3D models. The orchestrator will then send theses addresses to the mocked result database in order to simulate a download of the model.

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

