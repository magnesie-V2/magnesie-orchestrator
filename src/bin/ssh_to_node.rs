#[allow(dead_code)]

extern crate reqwest;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use std::{thread, time};
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use std::path::Path;

#[allow(unused_must_use)]
fn main() {
    install_docker_pull_image();
}

#[allow(dead_code)]
fn install_docker_pull_image() {

    // Connect to the local SSH server
    let tcp = TcpStream::connect("parapide-25.rennes.grid5000.fr:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let pub_key : &Path = Path::new("C:\\Users\\Bart\\.ssh\\test.pub");
    let priv_key : &Path = Path::new("C:\\Users\\Bart\\.ssh\\test.pem");

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file("root", Some(pub_key), priv_key, None).unwrap();
    // sess.userauth_password("tester", "password").unwrap();

    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("apt-get update").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get install apt-transport-https ca-certificates curl gnupg-agent software-properties-common -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("add-apt-repository \"deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable\" -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get update").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get install docker-ce docker-ce-cli containerd.io -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("apt-get install git-all -y").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("git clone git@github.com:magnesie/magnesie-photogrammetry.git").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("docker build --tag magnesie-photogrammetry-mock magnesie-photogrammetry").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel = sess.channel_session().unwrap();
    channel.exec("docker run --rm --name=magnesie-photogrammetry-mock -i -t -p 80:8000 -v ~/dev/magnesie/res:/res magnesie-photogrammetry-mock").unwrap();
    s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
}