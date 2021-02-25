var searchIndex = JSON.parse('{\
"main":{"doc":"Magnes.ie project","i":[[0,"services","main","Contains clients for all the microservices and the …",null,null],[0,"service","main::services","",null,null],[8,"Service","main::services::service","This trait represents functionalities shared by all …",null,null],[10,"get_name","","Returns the name of this service",0,[[],["string",3]]],[10,"get_services_keeper","","Returns an Arc<RwLock<>> to the service keeper",0,[[],[["arc",3],["rwlock",3]]]],[11,"get_access_information","","Returns the current access information of this service",0,[[],[["result",4],["serviceaccessinformation",3],["serviceerror",4]]]],[0,"service_error","main::services","",null,null],[4,"ServiceError","main::services::service_error","This enum represents every type of error that a service …",null,null],[13,"BasicError","","Basic error containing only a string",1,null],[13,"RequestError","","Error thrown during request, here using the Reqwest …",1,null],[0,"remote","main::services","",null,null],[3,"ServicesKeeper","main::services::remote","Keeps a map of the micro services access information ",null,null],[12,"services","","",2,null],[11,"new","","Creates a ServiceKeeper struct",2,[[],["serviceskeeper",3]]],[11,"register_service","","Adds a service\'s access information to the map at a …",2,[[["serviceaccessinformation",3],["str",15]]]],[11,"get_service","","Returns a service\'s access information based on a key",2,[[["str",15]],[["serviceaccessinformation",3],["option",4]]]],[3,"ServiceAccessInformation","","Contains access information of a webservice",null,null],[12,"host","","",3,null],[12,"port","","",3,null],[12,"username","","",3,null],[12,"password","","",3,null],[11,"new","","Creates a ServiceAccessInformation struct",3,[[["u16",15],["str",15]],["serviceaccessinformation",3]]],[11,"get_host","","Returns the host of the webservice",3,[[],["str",15]]],[11,"get_port","","Returns the port that the webservice listens to",3,[[],["u16",15]]],[11,"get_username","","Returns the username needed to send requests to the …",3,[[],["str",15]]],[11,"get_password","","Returns the password needed to send requests to the …",3,[[],["str",15]]],[0,"photogrammetry","main::services","",null,null],[3,"PhotogrammetryJob","main::services::photogrammetry","Represents a job created by the PhotogrammetryService",null,null],[12,"id","","",4,null],[12,"status","","",4,null],[3,"PhotogrammetryJobRequestBody","","Represents a request body to start a job in the …",null,null],[12,"photos","","",5,null],[12,"callback","","",5,null],[3,"PhotogrammetryService","","HTTP client for the photogrammetry microservice",null,null],[12,"services_keeper","","",6,null],[12,"client","","",6,null],[11,"new","","Creates a PhotogrammetryService struct",6,[[["arc",3],["rwlock",3]],[["photogrammetryservice",3],["serviceerror",4],["result",4]]]],[11,"create_job","","Sends pictures urls to the photogrammetry microservice …",6,[[["str",15]],[["string",3],["serviceerror",4],["result",4]]]],[11,"get_job","","Retrieves information about a job based on its id",6,[[["str",15]],[["serviceerror",4],["photogrammetryjob",3],["result",4]]]],[11,"get_job_result_url","","Retrieves information about a job\'s result based on its id",6,[[["str",15]],[["string",3],["serviceerror",4],["result",4]]]],[0,"image_storage","main::services","",null,null],[3,"Submission","main::services::image_storage","Represents a submission from the ImageStorageService",null,null],[12,"id","","",7,null],[12,"photos","","",7,null],[12,"submission_date","","",7,null],[3,"SubmissionUpdateRequestBody","","Represents a request body to edit a submission in the …",null,null],[12,"id","","",8,null],[12,"status","","",8,null],[3,"ImageStorageService","","HTTP client to the image storage microservice",null,null],[12,"services_keeper","","Keeps track of all services access information, necessary …",9,null],[12,"client","","",9,null],[11,"new","","Creates a ImageStorageService struct",9,[[["arc",3],["rwlock",3]],[["serviceerror",4],["imagestorageservice",3],["result",4]]]],[11,"get_new_submissions","","Returns new submissions currently stored in the …",9,[[],[["vec",3],["serviceerror",4],["result",4]]]],[11,"change_submission_status","","Updates the status of a submission in the …",9,[[["i32",15],["str",15]],[["serviceerror",4],["result",4]]]],[0,"result_storage","main::services","",null,null],[3,"ResultRequestBody","main::services::result_storage","Request body of a result to send to the …",null,null],[12,"submission_id","","",10,null],[12,"result_url","","",10,null],[3,"ResultStorageService","","HTTP client for the ResultStorageService",null,null],[12,"services_keeper","","",11,null],[12,"client","","",11,null],[11,"new","","Creates a ResultStorageService struct",11,[[["arc",3],["rwlock",3]],[["resultstorageservice",3],["serviceerror",4],["result",4]]]],[11,"post_result","","Sends a result url to the result storage service",11,[[["i32",15],["str",15]],[["serviceerror",4],["result",4]]]],[0,"jobs_buffer","main","Contains the jobs buffer, which keeps track of the …",null,null],[0,"buffered_job","main::jobs_buffer","",null,null],[3,"BufferedJob","main::jobs_buffer::buffered_job","Represents a job in the orchestrator\'s buffer",null,null],[12,"id","","The job\'s id. It\'s None by default but can be set to …",12,null],[12,"photos","","The list of photos of the submission",12,null],[12,"submission_id","","The id of the original submission",12,null],[12,"submission_date","","The time when this submission has been added to the buffer",12,null],[11,"new","","Creates a BufferedJob struct",12,[[["systemtime",3],["i32",15],["option",4]],["bufferedjob",3]]],[11,"get_complexity","","Returns the complexity of this job",12,[[],["f32",15]]],[0,"jobs_buffer","main::jobs_buffer","",null,null],[3,"JobsBuffer","main::jobs_buffer::jobs_buffer","Keeps a list of jobs and submissions (which are jobs",null,null],[12,"jobs","","The list of submissions and jobs (which are submissions …",13,null],[11,"new","","Creates a JobsBuffer struct",13,[[],["jobsbuffer",3]]],[11,"add_job_or_submission","","Adds a submission or a job to the buffer",13,[[["bufferedjob",3]],[["buffererror",4],["result",4]]]],[11,"remove_job","","Removes a job based on its id",13,[[["str",15]],[["buffererror",4],["result",4]]]],[11,"get_job_by_id","","Returns a job based on it\'s id",13,[[["str",15]],[["bufferedjob",3],["option",4]]]],[11,"get_pending_submissions","","Returns the list of jobs that have not been sent to be …",13,[[],[["option",4],["vec",3]]]],[11,"submission_exists","","Returns <strong>true</strong> if there\'s a submission in the buffer that …",13,[[["bufferedjob",3]],["bool",15]]],[11,"job_exists","","Returns <strong>true</strong> if there is a job in the buffer with the …",13,[[["bufferedjob",3]],["bool",15]]],[11,"has_buffered_jobs","","Returns true if the buffer has jobs waiting to be …",13,[[],["bool",15]]],[11,"check_timeouts","","Checks whether there are jobs that were created more than …",13,[[]]],[0,"buffer_error","main::jobs_buffer","",null,null],[4,"BufferError","main::jobs_buffer::buffer_error","This enum represents every type of error that a buffer …",null,null],[13,"BasicError","","Basic error containing only a string",14,null],[0,"orchestrator","main","",null,null],[17,"ENERGY_COST_PER_COMPLEXITY_UNIT","main::orchestrator","This constants needs to be set to weight the energy cost …",null,null],[3,"Orchestrator","","The brain of the application. Its purpose is to …",null,null],[12,"ticks_delay","","Delay in seconds between iterations of the orchestrator …",15,null],[12,"green_energy_timeout","","Delay in seconds before forcing jobs processing without …",15,null],[12,"services_keeper","","Keeps information to access microservices (hostname, …",15,null],[12,"jobs_buffer","","Keeps the list of ongoing submissions and jobs. Note: a …",15,null],[12,"clusters_manager","","Keeps the list of clusters where the photogrammetry …",15,null],[12,"image_storage","","Client for the images storage service, which stores the …",15,null],[12,"photogrammetry","","Client for the photogrammetry microservice",15,null],[12,"result_storage","","Client for the results storage microservice, which stores …",15,null],[11,"new","","Constructs an Orchestrator struct",15,[[["arc",3],["arc",3],["u64",15],["imagestorageservice",3],["resultstorageservice",3],["arc",3],["rwlock",3],["arc",3],["rwlock",3],["arc",3],["arc",3],["photogrammetryservice",3],["rwlock",3]],["orchestrator",3]]],[11,"start","","Starts the orchestrator main loop",15,[[["orchestrator",3],["arc",3]]]],[11,"deploy_and_register_photogrammetry_service","","Deploys the photogrammetry service and registers its …",15,[[["box",3]]]],[11,"start_web_server","","Starts a web service that listens to the 7878 port to …",15,[[["orchestrator",3],["arc",3]]]],[11,"add_submissions_to_buffer","","Fetches the new submissions from the ImageStorageService …",15,[[],[["string",3],["result",4]]]],[11,"select_jobs_to_run","","Decides which jobs to run in the list of pending …",15,[[["box",3]],[["option",4],["vec",3]]]],[11,"run_jobs","","Sends all the received jobs to the photogrammetry service",15,[[],[["string",3],["result",4]]]],[11,"handle_tcp_connection","","Handles basic tcp connections",15,[[["tcpstream",3]],[["string",3],["result",4]]]],[11,"photogrammetry_callback","","Reacts to a ping from the photogrammetry microservice …",15,[[["str",15]],[["serviceerror",4],["result",4]]]],[0,"clusters","main","Contains the clusters manager as well as all the clusters …",null,null],[0,"clusters_manager","main::clusters","",null,null],[3,"ClustersManager","main::clusters::clusters_manager","The cluster manager keeps the list of all the available …",null,null],[12,"clusters","","",16,null],[11,"new","","Creates a ClustersManager struct",16,[[],["clustersmanager",3]]],[11,"add_cluster","","Adds a cluster to the list",16,[[["box",3],["clusterfeatures",8]]]],[11,"has_clusters","","Checks whether there are clusters in the list",16,[[],["bool",15]]],[11,"select_cluster","","Selects the best cluster in the list",16,[[],[["box",3],["option",4]]]],[0,"cluster","main::clusters","",null,null],[6,"Cluster","main::clusters::cluster","Custom type that represents a ClusterFeatures trait object",null,null],[8,"ClusterFeatures","","Defines feature shared by all clusters",null,null],[11,"get_green_energy_produced","","Returns how much energy as been produced since the last …",17,[[],[["f32",15],["option",4]]]],[11,"get_current_energy_consumption","","Returns how much energy has been consumed since the last …",17,[[],[["f32",15],["option",4]]]],[10,"deploy_photogrammetry_service","","Deploys the photogrammetry service on this cluster",17,[[],[["result",4],["serviceaccessinformation",3],["clustererror",4]]]],[11,"free_resources","","Frees the resources that were reserved with the last …",17,[[],[["result",4],["clustererror",4]]]],[10,"get_reservation_status","","Returns the current status of the resources reservations",17,[[],[["option",4],["reservationstatus",4]]]],[10,"get_access_information","","Returns the access information of the deployed …",17,[[],[["option",4],["serviceaccessinformation",3]]]],[4,"ReservationStatus","","Defines the possible statuses of a cluster resources …",null,null],[13,"ResourcesAvailable","","The resources can be used",18,null],[13,"Pending","","The resources can\'t be used yet",18,null],[13,"Expired","","The resources have expired and can\'t be used anymore",18,null],[0,"cluster_error","main::clusters","",null,null],[4,"ClusterError","main::clusters::cluster_error","This enum represents every type of error that a cluster …",null,null],[13,"BasicError","","Basic error containing only a string",19,null],[0,"local_photogrammetry","main::clusters","",null,null],[3,"LocalPhotogrammetry","main::clusters::local_photogrammetry","This represents a \\\"fake\\\" cluster, in a context where the …",null,null],[12,"reservation_status","","",20,null],[11,"new","","Creates a LocalPhotogrammetry struct",20,[[],["localphotogrammetry",3]]],[0,"grid5000_struct","main::clusters","",null,null],[0,"grid5000_deploy_env_response","main::clusters::grid5000_struct","",null,null],[3,"DeployEnvResponse","main::clusters::grid5000_struct::grid5000_deploy_env_response","Representation of an environment deployment response",null,null],[12,"uid","","",21,null],[12,"site_uid","","",21,null],[12,"user_uid","","",21,null],[12,"environment","","",21,null],[12,"status","","",21,null],[12,"key","","",21,null],[12,"nodes","","",21,null],[12,"created_at","","",21,null],[12,"updated_at","","",21,null],[12,"links","","",21,null],[0,"grid5000_deployment_request","main::clusters::grid5000_struct","",null,null],[3,"DeploymentRequest","main::clusters::grid5000_struct::grid5000_deployment_request","Representation of an environment deployment request",null,null],[12,"environment","","",22,null],[12,"nodes","","",22,null],[12,"key","","",22,null],[0,"grid5000_link_job","main::clusters::grid5000_struct","",null,null],[3,"LinkJob","main::clusters::grid5000_struct::grid5000_link_job","Representation of a Link",null,null],[12,"rel","","",23,null],[12,"href","","",23,null],[12,"type","","",23,null],[0,"grid5000_reservation_request","main::clusters::grid5000_struct","",null,null],[3,"ReservationRequest","main::clusters::grid5000_struct::grid5000_reservation_request","Representation of job reservation request",null,null],[12,"name","","",24,null],[12,"resources","","",24,null],[12,"command","","",24,null],[12,"types","","",24,null],[0,"gridd500_job_submit_response","main::clusters::grid5000_struct","",null,null],[3,"JobSubmitResponse","main::clusters::grid5000_struct::gridd500_job_submit_response","Representation of a job reservation response",null,null],[12,"uid","","",25,null],[12,"user_uid","","",25,null],[12,"user","","",25,null],[12,"walltime","","",25,null],[12,"queue","","",25,null],[12,"state","","",25,null],[12,"project","","",25,null],[12,"types","","",25,null],[12,"mode","","",25,null],[12,"command","","",25,null],[12,"submitted_at","","",25,null],[12,"started_at","","",25,null],[12,"message","","",25,null],[12,"properties","","",25,null],[12,"directory","","",25,null],[12,"events","","",25,null],[12,"links","","",25,null],[12,"assigned_nodes","","",25,null],[0,"grid5000","main::clusters","",null,null],[3,"Grid5000","main::clusters::grid5000","Representation of a Grid5000 job reservation",null,null],[12,"api_base_url","","",26,null],[12,"deploy_url","","",26,null],[12,"job_url_pretty","","",26,null],[12,"job_url","","",26,null],[12,"username","","",26,null],[12,"password","","",26,null],[12,"site","","",26,null],[12,"nb_nodes","","",26,null],[12,"walltime","","",26,null],[12,"ssh_key_path","","",26,null],[12,"reserved_node_address","","",26,null],[12,"uid","","",26,null],[11,"new","","",26,[[["string",3]],["grid5000",3]]],[11,"new_random_site","","Create a reservation on a random site that has available …",26,[[["string",3]],["grid5000",3]]],[11,"has_green_energy_available","","",26,[[],["bool",15]]],[11,"delete_reservation","","Delete reservation of node with uid = job_uid",26,[[["string",3]],[["error",3],["result",4]]]],[11,"reserve_node","","Make a request to the Grid5000 to reserve a node",26,[[],[["result",4],["jobsubmitresponse",3],["error",3]]]],[11,"make_reservation","","Make a reservartion and return the adress of the reserved …",26,[[],["string",3]]],[11,"get_reservation","","Check state of reservation with uid = job_uid",26,[[["string",3]],[["result",4],["jobsubmitresponse",3],["error",3]]]],[11,"deploy_env_on_node","","Deploy provided environment to specified node",26,[[["string",3],["vec",3]],[["error",3],["deployenvresponse",3],["result",4]]]],[11,"get_deployment","","Check state of deployment with uid = deployment_uid",26,[[["string",3]],[["error",3],["deployenvresponse",3],["result",4]]]],[11,"get_ssh_key","","Get the SSH key from provided file",26,[[],[["string",3],["result",4],["box",3]]]],[11,"get_sites_with_green_energy","","Uses the OpenWeatherMap api to get the Grid5000 with …",26,[[],[["string",3],["vec",3]]]],[11,"choose_random_site_with_green_energy","","Chooses a random grid5000 site with available green energy",26,[[],["string",3]]],[0,"ssh_client","main","Contains a client to perform a SSH connection and SSH …",null,null],[0,"ssh_client","main::ssh_client","",null,null],[3,"SshClient","main::ssh_client::ssh_client","Representation of a SSH client with required information …",null,null],[12,"tcp_address","","",27,null],[12,"username","","",27,null],[12,"pub_key","","",27,null],[12,"priv_key","","",27,null],[11,"new","","",27,[[["string",3],["pathbuf",3]],["sshclient",3]]],[11,"initiate_ssh_connection","","Initiate a SSH connection",27,[[],["session",3]]],[11,"install_docker","","Install Docker via SSH",27,[[]]],[11,"pull_mock_photo_docker","","Pull the mocked photogrammetry service from docker",27,[[]]],[11,"run_docker","","Run Docker image via SSH",27,[[]]],[0,"meteo_service","main","Contains a client to make REST request to the …",null,null],[0,"meteo_service","main::meteo_service","",null,null],[17,"GRID5000_SITES_FILE_PATH","main::meteo_service::meteo_service","",null,null],[17,"OPEN_WEATHER_MAP_CONF_FILE","","",null,null],[3,"MeteoClient","","Struct to hold the api adress and API for OpenWeatherMap. …",null,null],[12,"api_address","","",28,null],[12,"api_key","","",28,null],[11,"new","","",28,[[],["meteoclient",3]]],[11,"get_weather_for_city","","Make a request to the API to get the weather, wind speed …",28,[[["string",3]],[["error",3],["result",4]]]],[11,"get_weather_for_grid5000_sites","","Calls the get_weather_for_city() method for every city in …",28,[[],["vec",3]]],[11,"get_sites_from_file","","Get the Grid5000 sites from the …",28,[[],[["result",6],["vec",3]]]],[5,"read_api_key_from_file","","Read the OpenWeatherMap api key from …",null,[[],[["string",3],["box",3],["result",4]]]],[17,"VERBOSE","main","If set to true, displays logging in the standard output",null,null],[5,"main","","Instantiates the various components and starts the …",null,[[],[["string",3],["result",4]]]],[5,"add_clusters","","Add clusters to the clusters manager",null,[[["arc",3]]]],[5,"add_grid5000_cluster","","Adds a g5k cluster to the clusters manager",null,[[["str",15],["arc",3]]]],[5,"log","","Print a message to the standard output",null,[[["str",15]]]],[5,"log_error","","Print an error to the standard error",null,[[["str",15]]]],[11,"from","main::services::service_error","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","main::services::remote","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","main::services::photogrammetry","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","main::services::image_storage","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"from","main::services::result_storage","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"from","main::jobs_buffer::buffered_job","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"from","main::jobs_buffer::jobs_buffer","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"vzip","","",13,[[]]],[11,"from","main::jobs_buffer::buffer_error","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_string","","",14,[[],["string",3]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"vzip","","",14,[[]]],[11,"from","main::orchestrator","",15,[[]]],[11,"into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"vzip","","",15,[[]]],[11,"from","main::clusters::clusters_manager","",16,[[]]],[11,"into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"vzip","","",16,[[]]],[11,"from","main::clusters::cluster","",18,[[]]],[11,"into","","",18,[[]]],[11,"to_owned","","",18,[[]]],[11,"clone_into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"vzip","","",18,[[]]],[11,"from","main::clusters::cluster_error","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_string","","",19,[[],["string",3]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"vzip","","",19,[[]]],[11,"from","main::clusters::local_photogrammetry","",20,[[]]],[11,"into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"vzip","","",20,[[]]],[11,"from","main::clusters::grid5000_struct::grid5000_deploy_env_response","",21,[[]]],[11,"into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"vzip","","",21,[[]]],[11,"from","main::clusters::grid5000_struct::grid5000_deployment_request","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"vzip","","",22,[[]]],[11,"from","main::clusters::grid5000_struct::grid5000_link_job","",23,[[]]],[11,"into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"vzip","","",23,[[]]],[11,"from","main::clusters::grid5000_struct::grid5000_reservation_request","",24,[[]]],[11,"into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"vzip","","",24,[[]]],[11,"from","main::clusters::grid5000_struct::gridd500_job_submit_response","",25,[[]]],[11,"into","","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"vzip","","",25,[[]]],[11,"from","main::clusters::grid5000","",26,[[]]],[11,"into","","",26,[[]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"vzip","","",26,[[]]],[11,"from","main::ssh_client::ssh_client","",27,[[]]],[11,"into","","",27,[[]]],[11,"borrow","","",27,[[]]],[11,"borrow_mut","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"try_into","","",27,[[],["result",4]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"vzip","","",27,[[]]],[11,"from","main::meteo_service::meteo_service","",28,[[]]],[11,"into","","",28,[[]]],[11,"borrow","","",28,[[]]],[11,"borrow_mut","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"try_into","","",28,[[],["result",4]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"vzip","","",28,[[]]],[11,"get_name","main::services::photogrammetry","",6,[[],["string",3]]],[11,"get_services_keeper","","",6,[[],[["arc",3],["rwlock",3]]]],[11,"get_name","main::services::image_storage","",9,[[],["string",3]]],[11,"get_services_keeper","","",9,[[],[["arc",3],["rwlock",3]]]],[11,"get_name","main::services::result_storage","",11,[[],["string",3]]],[11,"get_services_keeper","","",11,[[],[["arc",3],["rwlock",3]]]],[11,"deploy_photogrammetry_service","main::clusters::local_photogrammetry","",20,[[],[["result",4],["serviceaccessinformation",3],["clustererror",4]]]],[11,"get_reservation_status","","",20,[[],[["option",4],["reservationstatus",4]]]],[11,"get_access_information","","",20,[[],[["option",4],["serviceaccessinformation",3]]]],[11,"deploy_photogrammetry_service","main::clusters::grid5000","Deploys the photogrammetry service on a Grid5000 node …",26,[[],[["result",4],["serviceaccessinformation",3],["clustererror",4]]]],[11,"get_access_information","","Get the access information for the photogrammetry service",26,[[],[["option",4],["serviceaccessinformation",3]]]],[11,"get_reservation_status","","Get the status of the reservation",26,[[],[["option",4],["reservationstatus",4]]]],[11,"from","main::services::service_error","",1,[[["error",3]]]],[11,"from","","",1,[[["str",15]]]],[11,"from","","",1,[[["string",3]]]],[11,"from","main::jobs_buffer::buffer_error","",14,[[["str",15]]]],[11,"from","","",14,[[["string",3]]]],[11,"from","main::clusters::cluster_error","",19,[[["str",15]]]],[11,"from","","",19,[[["string",3]]]],[11,"clone","main::clusters::cluster","",18,[[]]],[11,"eq","","",18,[[],["bool",15]]],[11,"ne","","",18,[[],["bool",15]]],[11,"to_string","main::jobs_buffer::buffered_job","",12,[[],["string",3]]],[11,"to_string","main::jobs_buffer::jobs_buffer","",13,[[],["string",3]]],[11,"fmt","main::services::photogrammetry","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","main::services::image_storage","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","main::services::result_storage","",10,[[["formatter",3]],["result",6]]],[11,"fmt","main::clusters::grid5000_struct::grid5000_deploy_env_response","",21,[[["formatter",3]],["result",6]]],[11,"fmt","main::clusters::grid5000_struct::grid5000_deployment_request","",22,[[["formatter",3]],["result",6]]],[11,"fmt","main::clusters::grid5000_struct::grid5000_link_job","",23,[[["formatter",3]],["result",6]]],[11,"fmt","main::clusters::grid5000_struct::grid5000_reservation_request","",24,[[["formatter",3]],["result",6]]],[11,"fmt","main::clusters::grid5000_struct::gridd500_job_submit_response","",25,[[["formatter",3]],["result",6]]],[11,"fmt","main::services::service_error","",1,[[["formatter",3]],["result",6]]],[11,"fmt","main::jobs_buffer::buffer_error","",14,[[["formatter",3]],["result",6]]],[11,"fmt","main::clusters::cluster_error","",19,[[["formatter",3]],["result",6]]],[11,"serialize","main::services::photogrammetry","",5,[[],["result",4]]],[11,"serialize","main::services::image_storage","",8,[[],["result",4]]],[11,"serialize","main::services::result_storage","",10,[[],["result",4]]],[11,"serialize","main::clusters::grid5000_struct::grid5000_deployment_request","",22,[[],["result",4]]],[11,"serialize","main::clusters::grid5000_struct::grid5000_reservation_request","",24,[[],["result",4]]],[11,"deserialize","main::services::photogrammetry","",4,[[],["result",4]]],[11,"deserialize","main::services::image_storage","",7,[[],["result",4]]],[11,"deserialize","main::clusters::grid5000_struct::grid5000_deploy_env_response","",21,[[],["result",4]]],[11,"deserialize","main::clusters::grid5000_struct::grid5000_link_job","",23,[[],["result",4]]],[11,"deserialize","main::clusters::grid5000_struct::gridd500_job_submit_response","",25,[[],["result",4]]]],"p":[[8,"Service"],[4,"ServiceError"],[3,"ServicesKeeper"],[3,"ServiceAccessInformation"],[3,"PhotogrammetryJob"],[3,"PhotogrammetryJobRequestBody"],[3,"PhotogrammetryService"],[3,"Submission"],[3,"SubmissionUpdateRequestBody"],[3,"ImageStorageService"],[3,"ResultRequestBody"],[3,"ResultStorageService"],[3,"BufferedJob"],[3,"JobsBuffer"],[4,"BufferError"],[3,"Orchestrator"],[3,"ClustersManager"],[8,"ClusterFeatures"],[4,"ReservationStatus"],[4,"ClusterError"],[3,"LocalPhotogrammetry"],[3,"DeployEnvResponse"],[3,"DeploymentRequest"],[3,"LinkJob"],[3,"ReservationRequest"],[3,"JobSubmitResponse"],[3,"Grid5000"],[3,"SshClient"],[3,"MeteoClient"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);