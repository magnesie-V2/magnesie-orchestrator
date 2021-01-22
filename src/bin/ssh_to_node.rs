#[allow(dead_code)]

extern crate reqwest;
extern crate serde;

use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use std::path::Path;

#[allow(unused_must_use)]
fn main() {

    let tcp_address= "parapide-21.rennes.grid5000.fr:22";
    let username = "root";
    let pub_key : &Path = Path::new("C:\\Users\\Bart\\.ssh\\test.pub");
    let priv_key : &Path = Path::new("C:\\Users\\Bart\\.ssh\\test.pem");

    install_docker_git(tcp_address, username, pub_key, priv_key);
    clone_git_repo(tcp_address, username, pub_key, priv_key);
    run_docker(tcp_address, username, pub_key, priv_key);
}

#[allow(dead_code)]
fn install_docker_git(tcp_address : &str, username : &str, pub_key : &Path, priv_key : &Path) {

    // Connect to the local SSH server
    let tcp = TcpStream::connect(tcp_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file(username, Some(pub_key), priv_key, None).unwrap();
    // sess.userauth_password("tester", "password").unwrap();

    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("apt-get update").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    let mut err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get install apt-transport-https ca-certificates curl gnupg-agent software-properties-common -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("add-apt-repository \"deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable\" -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get update").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get install docker-ce docker-ce-cli containerd.io -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get install git-all -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);
}

fn clone_git_repo(tcp_address : &str, username : &str, pub_key : &Path, priv_key : &Path) {

    // Connect to the local SSH server
    let tcp = TcpStream::connect(tcp_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file(username, Some(pub_key), priv_key, None).unwrap();
    // sess.userauth_password("tester", "password").unwrap();

    assert!(sess.authenticated());

    let  mut channel = sess.channel_session().unwrap();
    channel.exec("git clone https://github.com/magnesie/magnesie-photogrammetry.git").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    let mut err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("git -C magnesie-photogrammetry checkout feature/webservice_mock_ref").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);
}

fn run_docker(tcp_address : &str, username : &str, pub_key : &Path, priv_key : &Path) {

    // Connect to the local SSH server
    let tcp = TcpStream::connect(tcp_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file(username, Some(pub_key), priv_key, None).unwrap();
    // sess.userauth_password("tester", "password").unwrap();

    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("docker build --tag magnesie-photogrammetry-mock magnesie-photogrammetry").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    let mut err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);

    channel = sess.channel_session().unwrap();
    channel.exec("cd magnesie-photogrammetry; docker run --rm --name=magnesie-photogrammetry-mock -p 7979:8000 -v $(pwd)/ref:/res magnesie-photogrammetry-mock &").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    err = String::new();
    channel.stderr().read_to_string(&mut err).unwrap();
    println!("{}", err);
}