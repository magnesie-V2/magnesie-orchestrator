#[allow(dead_code)]
extern crate reqwest;
extern crate serde;

use ssh2::Session;
use std::{io::prelude::*, path::PathBuf};
use std::net::TcpStream;

/// Representation of a SSH client with required information to initiate the connection
pub struct SshClient{
    tcp_address : String,
    username : String,
    pub_key: PathBuf,
    priv_key: PathBuf
}

impl SshClient {
    
    #[allow(dead_code)]
    pub fn new(tcp_address: String, username: String, pub_key: PathBuf, priv_key: PathBuf) -> SshClient {
        SshClient {
            tcp_address: format!("{}{}", tcp_address, ":22"),
            username,
            pub_key,
            priv_key
        }
    }

    /// Initiate a SSH connection
    fn initiate_ssh_connection(&self) -> Session {

        let tcp = TcpStream::connect(&self.tcp_address).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
    
        // Try to authenticate with a key pair
        sess.userauth_pubkey_file(&self.username, Some(&self.pub_key), &self.priv_key, None).unwrap();
    
        assert!(sess.authenticated());
    
        return sess;
    }

    /// Install Docker via SSH
    #[allow(dead_code)]
    pub fn install_docker(&self) {

        let sess : Session = self.initiate_ssh_connection();

        // Update apt repository
        let mut channel = sess.channel_session().unwrap();
        channel.exec("apt-get update").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        let mut err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);

        // install dependencies for Docker
        channel = sess.channel_session().unwrap();
        channel.exec("apt-get install apt-transport-https ca-certificates curl gnupg-agent software-properties-common -y").unwrap();
        s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);

        // Get Docker Debian GPG Key
        channel = sess.channel_session().unwrap();
        channel
            .exec("curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -")
            .unwrap();
        s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);

        // Add Docker Repository
        channel = sess.channel_session().unwrap();
        channel.exec("add-apt-repository \"deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable\" -y").unwrap();
        s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);

        // Update apt repository
        channel = sess.channel_session().unwrap();
        channel.exec("apt-get update").unwrap();
        s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);

        // Install Docker
        channel = sess.channel_session().unwrap();
        channel.exec("apt-get install docker-ce docker-ce-cli containerd.io -y").unwrap();
        s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);
    }

    /// Pull the mocked photogrammetry service from docker
    #[allow(dead_code)]
    pub fn pull_mock_photo_docker(&self) {

        let sess : Session = self.initiate_ssh_connection();

        // Builde Docker Image
        let mut channel = sess.channel_session().unwrap();
        channel.exec("docker pull mgaonach/magnesie-photogrammetry:latest").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        let mut err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);
    }

    /// Run Docker image via SSH
    #[allow(dead_code)]
    pub fn run_docker(&self) {

        let sess : Session = self.initiate_ssh_connection();

        // Run Docker Image
        let mut channel = sess.channel_session().unwrap();
        channel.exec("docker run --name=magnesie-photogrammetry -p 7879:8000 mgaonach/magnesie-photogrammetry &").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        let mut err = String::new();
        channel.stderr().read_to_string(&mut err).unwrap();
        println!("{}", err);
    }
}