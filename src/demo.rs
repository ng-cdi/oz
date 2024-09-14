use bollard::container::StartContainerOptions;
use bollard::exec::{StartExecOptions, StartExecResults};
use bollard::secret::{DeviceMapping, HostConfig};
use bollard::{container::{Config, CreateContainerOptions}, Docker, API_DEFAULT_VERSION};
use futures::future::join_all;
use futures::StreamExt;
use rand::distributions::Standard;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use reqwest::header::HeaderMap;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
//use rtnetlink::{new_connection, LinkAddRequest};
use std::process::Command;

use crate::common::{self};
use crate::onos;

// Return this from container fgc 
// then use for querying.
pub struct Node<'a> {
    name: &'a str,
    mac_id: &'a str,
}

pub struct FGCContainerOptions<'a> {
    name: &'a str,
    env: Option<Vec<&'a str>>,
    cmd: Option<Vec<&'a str>>,
    cap_add: Option<Vec<&'a str>>,
    devices: Option<Vec<DeviceMapping>>,
    binds: Option<Vec<&'a str>>,
    extra_hosts: Option<Vec<&'a str>>
}

pub struct MyContainerOptions<'a> {
    name: &'a str,
    image: &'a str,
    env: Option<Vec<&'a str>>,
    cmd: Option<Vec<&'a str>>,
    cap_add: Option<Vec<&'a str>>,
    devices: Option<Vec<DeviceMapping>>,
    binds: Option<Vec<&'a str>>,
}

#[allow(unreachable_code)]
pub async fn start_demo() {

    let compose = read_compose();
    //println!("{:?}", compose);
    println!("{:?}", compose["services"]["free5gc-upf"]);
    //return;

    //cleanup().await;
    //tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

    let mut next_ip: u8 = 32;
    //let compose = read_compose();
    //println!("{:?}", compose);
    //println!("{:?}", compose["services"]["free5gc-upf"]);

    for i in 1..=14 {
        let bridge_name = format!("s{}", i);
        add_bridge(&bridge_name);
        set_bridge(&bridge_name);
    }

    connect_bridges("s1", "s7");
    connect_bridges("s1", "s8");
    connect_bridges("s2", "s7");
    connect_bridges("s2", "s8");
    connect_bridges("s2", "s9");
    connect_bridges("s3", "s9");
    connect_bridges("s3", "s10");
    connect_bridges("s4", "s9");
    connect_bridges("s4", "s10");
    connect_bridges("s5", "s10");
    connect_bridges("s5", "s11");
    connect_bridges("s6", "s10");
    connect_bridges("s6", "s11");
    connect_bridges("s7", "s12");
    connect_bridges("s8", "s12");
    connect_bridges("s8", "s13");
    connect_bridges("s9", "s12");
    connect_bridges("s9", "s13");
    connect_bridges("s10", "s12");
    connect_bridges("s10", "s13");
    connect_bridges("s11", "s13");
    connect_bridges("s11", "s14");
    connect_bridges("s12", "s13");
    connect_bridges("s12", "s14");
    connect_bridges("s13", "s14");

    let cert_dir = "/home/paul/projects/free5gc-compose/cert:/free5gc/cert";
    let all_hosts: Vec<&str> = vec![
        "gnb1.free5gc.org:192.0.0.1",
        "gnb2.free5gc.org:192.0.0.2",
        "gnb3.free5gc.org:192.0.0.3",
        "gnb4.free5gc.org:192.0.0.4",
        "upf.free5gc.org:192.0.0.5",
        "db:192.0.0.6",
        "nrf.free5gc.org:192.0.0.7",
        "amf.free5gc.org:192.0.0.8",
        "ausf.free5gc.org:192.0.0.9",
        "nssf.free5gc.org:192.0.0.10",
        "pcf.free5gc.org:192.0.0.11",
        "smf.free5gc.org:192.0.0.12",
        "udm.free5gc.org:192.0.0.13",
        "udr.free5gc.org:192.0.0.14",
        "chf.free5gc.org:192.0.0.15",
        "n3iwf.free5gc.org:192.0.0.16",
        "webui:192.0.0.17",
        "n3ue.free5gc.org:192.0.0.18",
    ];
   
    let upf = FGCContainerOptions {
        name: "upf",
        //cmd: Some(vec!["tail", "-f", "/dev/null"]),
        cmd: Some(vec![
            "bash", 
            "-c",
            "sleep 25s && ./upf-iptables.sh && ./upf -c ./config/upfcfg.yaml"
        ]),
        cap_add: Some(vec!["NET_ADMIN"]),
        binds: Some(vec![
            "/home/paul/projects/free5gc-compose/config/upf-iptables.sh:/free5gc/upf-iptables.sh",
        ]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let db = MyContainerOptions {
        name: "mongodb",
        image: "docker.io/mongo:7.0.12-jammy",
        cmd: Some(vec!["mongod", "--port", "27017"]),
        cap_add: None,
        binds: None,
        env: Some(vec!["DB_URI=mongodb://db/free5gc"]),
        devices: None,
    };
    let nrf = FGCContainerOptions {
        name: "nrf",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: Some(vec!["DB_URI=mongodb://db/free5gc"]),
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let amf = FGCContainerOptions {
        name: "amf",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let ausf = FGCContainerOptions {
        name: "ausf",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let nssf = FGCContainerOptions {
        name: "nssf",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let pcf = FGCContainerOptions {
        name: "pcf",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let smf = FGCContainerOptions {
        name: "smf",
        cmd: Some(vec![
            "sh",
            "-c",
            "sleep 25s && ./smf -c ./config/smfcfg.yaml -u ./config/uerouting.yaml"
        ]),
        cap_add: None,
        binds: Some(vec![
            "/home/paul/projects/free5gc-compose/config/uerouting.yaml:/free5gc/config/uerouting.yaml",
            cert_dir
        ]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let udm = FGCContainerOptions {
        name: "udm",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let udr = FGCContainerOptions {
        name: "udr",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: Some(vec!["DB_URI=mongodb://db/free5gc"]),
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let chf = FGCContainerOptions {
        name: "chf",
        cmd: None,
        cap_add: None,
        binds: Some(vec![cert_dir]),
        env: Some(vec!["DB_URI=mongodb://db/free5gc"]),
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let n3iwf = FGCContainerOptions {
        name: "n3iwf",
        cmd: Some(vec![
            "sh", 
            "-c", 
            "sleep 25s && ./n3iwf-ipsec.sh && ./n3iwf -c ./config/n3iwfcfg.yaml"
        ]),
        cap_add: Some(vec!["NET_ADMIN"]),
        binds: Some(vec![
            "/home/paul/projects/free5gc-compose/config/n3iwf-ipsec.sh:/free5gc/n3iwf-ipsec.sh",
            cert_dir
        ]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let webui = FGCContainerOptions {
        name: "webui",
        cmd: None,
        cap_add: None,
        binds: None,
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let n3iwue = FGCContainerOptions {
        name: "n3iwue",
        cmd: Some(vec!["sleep", "infinity"]),
        cap_add: Some(vec!["NET_ADMIN"]),
        binds: None,
        env: None,
        devices: Some(vec![DeviceMapping{
            path_on_host: Some("/dev/net/tun".to_string()),
            path_in_container: Some("/dev/net/tun".to_string()),
            cgroup_permissions: Some("rw".to_string()),
        }]),
        extra_hosts: Some(all_hosts.clone()),
    };
    add_container_fgc(upf,      "s3", "192.0.0.5/24").await;
    add_container(db,           "s3", "192.0.0.6/24").await;
    add_container_fgc(nrf,      "s3", "192.0.0.7/24").await;
    add_container_fgc(amf,      "s3", "192.0.0.8/24").await;
    add_container_fgc(ausf,     "s3", "192.0.0.9/24").await;
    add_container_fgc(nssf,     "s3", "192.0.0.10/24").await;
    add_container_fgc(pcf,      "s3", "192.0.0.11/24").await;
    add_container_fgc(smf,      "s3", "192.0.0.12/24").await;
    add_container_fgc(udm,      "s3", "192.0.0.13/24").await;
    add_container_fgc(udr,      "s3", "192.0.0.14/24").await;
    add_container_fgc(chf,      "s3", "192.0.0.15/24").await;
    add_container_fgc(n3iwf,    "s3", "192.0.0.16/24").await;
    add_container_fgc(webui,    "s3", "192.0.0.17/24").await;
    add_container_fgc(n3iwue,   "s3", "192.0.0.18/24").await;
    /*
    let mut futures = Vec::new();
    futures.push(add_container_fgc(upf,      "s3", "192.0.0.5/24",  "192.0.0.19/24"));
    futures.push(add_container_fgc(nrf,      "s3", "192.0.0.7/24",  "192.0.0.21/24"));
    futures.push(add_container_fgc(amf,      "s3", "192.0.0.8/24",  "192.0.0.22/24"));
    futures.push(add_container_fgc(ausf,     "s3", "192.0.0.9/24",  "192.0.0.23/24"));
    futures.push(add_container_fgc(nssf,     "s3", "192.0.0.10/24",  "192.0.0.24/24"));
    futures.push(add_container_fgc(pcf,      "s3", "192.0.0.11/24",  "192.0.0.25/24"));
    futures.push(add_container_fgc(smf,      "s3", "192.0.0.12/24",  "192.0.0.26/24"));
    futures.push(add_container_fgc(udm,      "s3", "192.0.0.13/24",  "192.0.0.27/24"));
    futures.push(add_container_fgc(udr,      "s3", "192.0.0.14/24", "192.0.0.28/24"));
    futures.push(add_container_fgc(chf,      "s3", "192.0.0.15/24", "192.0.0.29/24"));
    futures.push(add_container_fgc(n3iwf,    "s3", "192.0.0.16/24", "192.0.0.30/24"));
    futures.push(add_container_fgc(webui,    "s3", "192.0.0.17/24", "192.0.0.31/24"));
    futures.push(add_container_fgc(n3iwue,   "s3", "192.0.0.18/24", "192.0.0.32/24"));
    join_all(futures).await;
    */

    ueransim_fleet(all_hosts).await;

    for i in 1..=14 {
        let bridge_name = format!("s{}", i);
        set_bridge_ports(&bridge_name, &mut next_ip);
    }

    add_veth_ovs("s3", "hosts");
    /*
    for i in 0..2 {
        add_subscriber(i).await;
    }
    get_hosts().await;
    */
}

pub async fn cleanup() {
    let containers: Vec<&str> = vec![
        "upf",
        "nrf",
        "amf",
        "ausf",
        "nssf",
        "pcf",
        "smf",
        "udm",
        "udr",
        "chf",
        "n3iwf",
        "webui",
        "n3iwue",
        "mongodb",
        "ueransim1",
        "ueransim2",
        "ueransim3",
        "ueransim4",
    ];
    let docker = default_connect();
    let mut futs = Vec::new();
    for i in &containers {
        futs.push(docker.stop_container(i, None));
    }
    let _ = join_all(futs).await;
    let mut futr = Vec::new();
    for i in &containers {
        futr.push(docker.remove_container(i, None));
    }
    let _ = join_all(futr).await;
    for i in 1..=14 {
        del_bridge(format!("s{}", i).as_str());
    }
    delete_bridge_connections();
}

async fn situation(all_hosts: Vec<&str>) {
    let cert_dir = "/home/paul/projects/free5gc-compose/cert:/free5gc/cert";
    let upf = FGCContainerOptions {
        name: "upf1",
        //cmd: Some(vec!["tail", "-f", "/dev/null"]),
        cmd: Some(vec![
            "bash", 
            "-c",
            "sleep 25s && ./upf-iptables.sh && ./upf -c ./config/upfcfg.yaml"
        ]),
        cap_add: Some(vec!["NET_ADMIN"]),
        binds: Some(vec![
            "/home/paul/projects/free5gc-compose/config/upf-iptables.sh:/free5gc/upf-iptables.sh",
        ]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    let smf = FGCContainerOptions {
        name: "smf1",
        cmd: Some(vec![
            "sh",
            "-c",
            "sleep 25s && ./smf -c ./config/smfcfg.yaml -u ./config/uerouting.yaml"
        ]),
        cap_add: None,
        binds: Some(vec![
            "/home/paul/projects/free5gc-compose/config/uerouting.yaml:/free5gc/config/uerouting.yaml",
            cert_dir
        ]),
        env: None,
        devices: None,
        extra_hosts: Some(all_hosts.clone()),
    };
    add_container_fgc(upf,      "s3", "192.0.0.19/24").await;
    add_container_fgc(smf,      "s3", "192.0.0.20/24").await;
}

fn assign_ip(intf: &str, cidr: &str) {
    let _output = Command::new("ip")
        .args(&["addr", "add", cidr, "dev", intf])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

async fn get_container_mac(name: &str) {
    let docker = default_connect();
    let res = docker.create_exec(name, bollard::exec::CreateExecOptions { 
        privileged: Some(true),
        tty: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(vec![
            "sh",
            "-c",
            format!("ip addr show dev eth0").as_str(),
        ]), 
        ..Default::default()
    }).await;
    if let StartExecResults::Attached{ mut output, ..} = docker.start_exec(&res.unwrap().id, Some(StartExecOptions { 
        tty: true, 
        ..Default::default()
    })).await.unwrap() {
        while let Some(Ok(msg)) = output.next().await {
            print!("AIC: {msg}");
        }
    } else {
        unreachable!();
    }
}

async fn assign_container_ip(intf: &str, cidr: &str, name: &str) {
    let docker = default_connect();
    let res = docker.create_exec(name, bollard::exec::CreateExecOptions { 
        privileged: Some(true),
        tty: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(vec![
            "sh",
            "-c",
            format!("ip addr add {} dev {}", cidr, intf).as_str(),
        ]), 
        ..Default::default()
    }).await;
    if let StartExecResults::Attached{ mut output, ..} = docker.start_exec(&res.unwrap().id, Some(StartExecOptions { 
        tty: true, 
        ..Default::default()
    })).await.unwrap() {
        while let Some(Ok(msg)) = output.next().await {
            print!("AIC: {msg}");
        }
    } else {
        unreachable!();
    }
    let res = docker.create_exec(name, bollard::exec::CreateExecOptions { 
        privileged: Some(true),
        tty: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(vec![
            "sh",
            "-c",
            format!("ip link set {} up", intf).as_str(),
        ]), 
        ..Default::default()
    }).await;
    if let StartExecResults::Attached{ mut output, ..} = docker.start_exec(&res.unwrap().id, Some(StartExecOptions { 
        tty: true, 
        ..Default::default()
    })).await.unwrap() {
        while let Some(Ok(_msg)) = output.next().await {
            //print!("{msg}");
        }
    } else {
        unreachable!();
    }
}

fn set_bridge(name: &str) {
    let _output = Command::new("ovs-vsctl")
        .args(&["set",
            "bridge", 
            name, 
            "protocols=OpenFlow14",
            "other_config:disable-in-band=true",
            "other_config:enable-stateless-reply=true",
            "other_config:inactivity_probe=10000",
            "other_config:max_backoff=20000",
        ])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
    let _output = Command::new("ovs-vsctl")
        .args(&["set-controller",
            name, 
            //"tcp:127.0.0.1:6633",
            "tcp:127.0.0.1:6653",
        ])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn set_bridge_ports(name: &str, next_ip: &mut u8) {
    let output = Command::new("sh")
        .args(&["-c",
            format!("ovs-vsctl list-ports {}", name).as_str(), 
        ])
        .output()
        .expect("Failed to execute command");
    //let port_count = String::from_utf8_lossy(&output.stdout);
    //let port_int: u8 = port_count.parse().unwrap();
    //println!("{}", String::from_utf8_lossy(&output.stdout));
    //println!("{:?}", output);
    let out = String::from_utf8_lossy(&output.stdout);
    let ports = out.lines();

    for i in ports {
        set_veth_ip(
            i, 
            format!("192.0.0.{}/24", next_ip).as_str()
        );
        set_veth_up(i);
        *next_ip += 1;
    }
}

fn add_bridge(name: &str) {
    let _output = Command::new("ovs-vsctl")
        .args(&["add-br", name])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn del_bridge(name: &str) {
    let _output = Command::new("ovs-vsctl")
        .args(&["del-br", name])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn add_veth(first: &str, second: &str) {
    let _output = Command::new("ip")
        .args(&["link", "add", first, "type", "veth", "peer", "name", second])
        .output()
        .expect("Failed to execute command");
    let _output = Command::new("ip")
        .args(&["link", "set", "dev", second, "address", format!("{}", random_mac()).as_str()])
        .output()
        .expect("Failed to execute command");
    //println!("Add Veth: {}", String::from_utf8_lossy(&output.stdout));
}

fn add_veth_container(name: &str, pid: i64) {
    let output = Command::new("ip")
        .args(&["link", "set", name, "netns", format!("{}", pid).as_str()])
        .output()
        .expect("Failed to execute command");
    println!("Add veth docker: {}", String::from_utf8_lossy(&output.stdout));
}

fn add_veth_ovs(name: &str, veth: &str) {
    let _output = Command::new("ovs-vsctl")
        .args(&["add-port", name, veth])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn del_veth(name: &str) {
    let _output = Command::new("ip")
        .args(&["link", "delete", name])
        .output()
        .expect("Failed to execute command");
}

fn set_veth_up(veth: &str) {
    let _output = Command::new("ip")
        .args(&["link", "set", veth, "up"])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

async fn set_veth_up_oci(veth: &str, name: &str) {
    let docker = default_connect();
    let res = docker.create_exec(name, bollard::exec::CreateExecOptions { 
        privileged: Some(true),
        tty: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(vec![
            "sh",
            "-c",
            format!("ip link set {} up", veth).as_str(),
        ]), 
        ..Default::default()
    }).await;
    if let StartExecResults::Attached{ mut output, ..} = docker.start_exec(&res.unwrap().id, Some(StartExecOptions { 
        tty: true, 
        ..Default::default()
    })).await.unwrap() {
        while let Some(Ok(msg)) = output.next().await {
            print!("SVUO: {msg}");
        }
    } else {
        unreachable!();
    }
}

// TODO
fn set_veth_ip(veth: &str, ip: &str) {
    let _output = Command::new("ip")
        .args(&["addr", "add", ip, "dev", veth])
        .output()
        .expect("Failed to execute command");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn connect_bridges(first: &str, second: &str) {
    let veth1 = format!("{}{}s", first, second);
    let veth2 = format!("{}{}e", first, second);
    add_veth(veth1.as_str(), veth2.as_str());

    add_veth_ovs(first, veth1.as_str());
    add_veth_ovs(second, veth2.as_str());
}

fn delete_bridge_connections() {
    let switches: Vec<(&str, &str)> = vec![
        ("s1", "s7"), ("s1", "s8"),
        ("s2", "s7"), ("s2", "s8"),
        ("s2", "s9"), ("s3", "s9"),
        ("s3", "s10"), ("s4", "s9"),
        ("s4", "s10"), ("s5", "s10"),
        ("s5", "s11"), ("s6", "s10"),
        ("s6", "s11"), ("s7", "s12"),
        ("s8", "s12"), ("s8", "s13"),
        ("s9", "s12"), ("s9", "s13"),
        ("s10", "s12"), ("s10", "s13"),
        ("s11", "s13"), ("s11", "s14"),
        ("s12", "s13"), ("s12", "s14"),
        ("s13", "s14"),
    ];
    for (k, v) in switches {
        del_veth(format!("{}{}s", k, v).as_str());
    }
}

fn default_connect() -> Docker {
    //Docker::connect_with_unix("/run/user/1000/podman/podman.sock", 120, API_DEFAULT_VERSION).unwrap()
    Docker::connect_with_unix("/run/docker.sock", 120, API_DEFAULT_VERSION).unwrap()
}

async fn get_pid(name: &str) -> Option<i64>{
    let docker = default_connect();
    let inspection = docker.inspect_container(name, None).await.unwrap();
    match inspection.state {
        Some(some) => return some.pid,
        None => return None
    }
}

async fn ueransim_fleet(hosts: Vec<&str>) {
    for i in 1..=4 {
        let name = format!("ueransim{}", i);
        let oci_ip = format!("192.0.0.{i}/24");
        let ueransim_veth1 = format!("ue{i}1");
        let ueransim_veth2 = "eth0";
        let hosts = hosts.clone();        
        let _strip_ip = oci_ip.split("/").collect::<Vec<&str>>()[0];
        //let o = format!("gnb.free5gc.org:{}", &strip_ip);
        //hosts.push(o.as_str());

        //let mut is_ue = false;
        //if i == 1 { is_ue = true; }
        let is_ue = true;
        add_ueransim(i, hosts, is_ue).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        //let ueransim_veth2 = format!("ue{i}2");
        add_veth(&ueransim_veth1, &ueransim_veth2);
        if i <= 2 {
            add_veth_ovs(format!("s{}", i).as_str(), &ueransim_veth1);
        } else {
            add_veth_ovs(format!("s{}", i+1).as_str(), &ueransim_veth1);
        }
        let pid = get_pid(&name).await;
        match pid {
            Some(id) => add_veth_container(&ueransim_veth2, id),
            None => cleanup().await,
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
        //let switch_ip = format!("192.0.0.{i}/16");
        assign_container_ip(&ueransim_veth2, oci_ip.as_str(), &name).await;
        //assign_ip(&ueransim_veth1, switch_ip.as_str());
        set_veth_up(&ueransim_veth1);
    }
}

async fn add_ueransim(num: u8, hosts: Vec<&str>, is_ue: bool) {
    let docker = default_connect();    
    let name = format!("ueransim{}", num);
    let ransim_options = Some(CreateContainerOptions {
        name: name.clone(),
        platform: None
    });
    let mut d_binds: Vec<String> = vec![
                format!("/home/paul/projects/free5gc-compose/config/gnb/gnb{}cfg.yaml:/ueransim/config/gnbcfg.yaml", num),
    ];
    match is_ue {
        //true => d_binds.push("/home/paul/projects/free5gc-compose/config/uecfg.yaml:/ueransim/config/uecfg.yaml".to_string()),
        true => d_binds.push("/home/paul/projects/free5gc-compose/config/ue:/ueransim/config/ue".to_string()),
        false => (), 
    }
    let ransim_config = Config {
        image: Some("docker.io/free5gc/ueransim:v3.4.1"),
        // Remember: Needs valid interface on startup
        cmd: Some(vec![
            "sh",
            "-c",
            "sleep 25s && ./nr-gnb -c ./config/gnbcfg.yaml"
        ]),
        //cmd: Some(vec!["tail", "-f", "/dev/null"]),
        host_config: Some(HostConfig{
            cap_add: Some(vec!["NET_ADMIN".to_string()]),
            devices: Some(vec![DeviceMapping{
                    path_on_host: Some("/dev/net/tun".to_string()),
                    path_in_container: Some("/dev/net/tun".to_string()),
                    cgroup_permissions: Some("rw".to_string()),
                }]),
            binds: Some(d_binds),
            network_mode: Some("none".to_string()),
            extra_hosts: Some(common::vec_to_vec(hosts)),
            ..Default::default()
        }),
        ..Default::default()
    };
    //let _ = docker.create_container(ransim_options, ransim_config).await;
    match docker.create_container(ransim_options, ransim_config).await {
        Ok(o) => println!("{} started\n{:?}", name, o),
        Err(e) => println!("{} error\n{:?}", name, e),
    }
    let _ = docker.start_container(&name, None::<StartContainerOptions<String>>).await;
}

async fn add_container_fgc<'a>(opts: FGCContainerOptions<'a>, switch: &str, oci_ip: &str) {
    let docker = default_connect();    
    let name = opts.name.to_string();
    let ransim_options = Some(CreateContainerOptions {
        name,
        platform: None
    });
    let e_cap_add = match opts.cap_add {
        Some(ca) => Some(common::vec_to_vec(ca)),
        None => None,
    };
    let e_binds = match opts.binds {
        Some(ca) => {
            let mut newvec: Vec<String> = vec![
            format!("/home/paul/projects/free5gc-compose/config/{}cfg.yaml:/free5gc/config/{}cfg.yaml", opts.name, opts.name)
        ];
            let mut oldvec: Vec<String> = common::vec_to_vec(ca);
            newvec.append(&mut oldvec);
            //Some(common::vec_to_vec(ca)),
            Some(newvec)
        },
        None => {
            let vec: Vec<String> = vec![format!("/home/paul/projects/free5gc-compose/config/{}cfg.yaml:/free5gc/config/{}cfg.yaml", opts.name, opts.name)];
            Some(vec)
        },
    };
    let e_cmd = match opts.cmd {
        Some(ca) => Some(common::vec_to_vec(ca)),
        None => {
            //let newvec: Vec<String> = common::vec_to_vec(vec!["tail", "-f", "/dev/null"]);
            let newvec: Vec<String> = vec![
                "sh".to_string(), 
                "-c".to_string(),
                format!("sleep 25s && ./{} -c ./config/{}cfg.yaml", &opts.name, opts.name)
            ];
            
            Some(newvec)
        }
    };
    let e_env = match opts.env {
        Some(ca) => {
            let mut newvec: Vec<String> = vec!["GIN_MODE=release".to_string()];
            let mut oldvec: Vec<String> = common::vec_to_vec(ca);
            newvec.append(&mut oldvec);
            Some(newvec)
        },
        None => None
    };
    let e_hosts = match opts.name {
        "n3iwue" => {
            let newvec = common::vec_to_vec(opts.extra_hosts.clone().unwrap());
            //remove_address("n3ue", &mut newvec);
            Some(newvec)
        },
        &_ => {
            let newvec = common::vec_to_vec(opts.extra_hosts.clone().unwrap());
            //remove_address(&opts.name, &mut newvec);
            Some(newvec)
        }
    };
    /*
    let e_hosts = match opts.extra_hosts {
        Some(hosts) => {
            let newvec: Vec<String> = common::vec_to_vec(hosts);
            Some(newvec)
        },
        None => None
    };
    */
    let ransim_config = Config {
        image: Some(format!("docker.io/free5gc/{}:v3.4.1", opts.name)),
        cmd: e_cmd,
        env: e_env,
        host_config: Some(HostConfig{
            cap_add: e_cap_add,
            devices: opts.devices,
            binds: e_binds,
            network_mode: Some("none".to_string()),
            extra_hosts: e_hosts,
            ..Default::default()
        }),
        ..Default::default()
    };
    match docker.create_container(ransim_options, ransim_config).await {
        Ok(o) => println!("{} started\n{:?}", opts.name, o),
        Err(e) => println!("{} error\n{:?}", opts.name, e),
    }
    let _ = docker.start_container(opts.name, None::<StartContainerOptions<String>>).await;
    let ovs_veth = format!("{}", opts.name);
    let oci_veth = "eth0";
    add_veth(&ovs_veth, oci_veth);
    add_veth_ovs(switch, &ovs_veth);
    //assign_ip(&ovs_veth, oci_ip);
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let pid = get_pid(&opts.name).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    add_veth_container(&oci_veth, pid);
    assign_container_ip(oci_veth, oci_ip, opts.name).await;
    set_veth_up(&ovs_veth);
    set_veth_up_oci(&oci_veth, opts.name).await;
}

async fn add_container<'a>(opts: MyContainerOptions<'a>, switch: &str, oci_ip: &str) {
    let docker = default_connect();    
    let name = opts.name.to_string();
    let ransim_options = Some(CreateContainerOptions {
        name,
        platform: None
    });
    let e_cap_add = match opts.cap_add {
        Some(ca) => Some(common::vec_to_vec(ca)),
        None => None,
    };
    let e_binds = match opts.binds {
        Some(ca) => Some(common::vec_to_vec(ca)),
        None => None,
    };
    /*
    let e_cmd = match opts.cmd {
        Some(ca) => Some(common::vec_to_vec(ca)),
        None => None,
    };
    let e_env = match opts.env {
        Some(ca) => Some(common::vec_to_vec(ca)),
        None => None,
    };
    */
    let ransim_config = Config {
        image: Some(opts.image),
        cmd: opts.cmd,
        env: opts.env,
        host_config: Some(HostConfig{
            cap_add: e_cap_add,
            devices: opts.devices,
            binds: e_binds,
            network_mode: Some("none".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    match docker.create_container(ransim_options, ransim_config).await {
        Ok(o) => println!("{} started\n{:?}", opts.name, o),
        Err(e) => println!("{} error\n{:?}", opts.name, e),
    }
    let _ = docker.start_container(opts.name, None::<StartContainerOptions<String>>).await;
    let ovs_veth = format!("{}", opts.name);
    let oci_veth = "eth0";
    add_veth(&ovs_veth, oci_veth);
    add_veth_ovs(switch, &ovs_veth);
    //assign_ip(&ovs_veth, oci_ip);
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let pid = get_pid(&opts.name).await.unwrap();
    add_veth_container(&oci_veth, pid);
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    assign_container_ip(oci_veth, oci_ip, opts.name).await;
    set_veth_up(&ovs_veth);
    set_veth_up_oci(&oci_veth, opts.name).await;
}

fn remove_address(name: &str, hosts: &mut Vec<String>) {
    for i in hosts.iter_mut() {
        let first_split = i.split(":").collect::<Vec<&str>>();
        let split = first_split[0].split(".").collect::<Vec<&str>>();
        if split[0] == name {
            let new = format!("{}{}", first_split[0], ":127.0.0.1");
            *i = new;
            println!("{}", i);
        }
    }
}

fn random_mac() -> String {
    let mut arr: [u8; 6] = StdRng::from_entropy().sample(Standard);
    arr[0] &= 0xFE;
    arr[0] |= 0x2;
    let mut mac: String = String::new();
    for i in 0..arr.len() {
        if i != 0 {mac.push_str(":"); }
        let char = hex::encode(vec![arr[i]]);
        mac.push_str(&char);
    }

    mac
}

fn read_compose() -> Value {
    let config = File::open("./5g/free5gc-compose.yaml").unwrap();
    let config_reader = BufReader::new(config);
    let cfg: Value = serde_yaml::from_reader(config_reader).unwrap();
    cfg
}

pub async fn add_subscriber(num: u8) {
    let sub = File::open("./5g/subscriber.json").unwrap();
    let sub_rdr = BufReader::new(sub);
    let mut req_json: Value = serde_json::from_reader(sub_rdr).unwrap();
   
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Accept-Enoding", "gzip, deflate".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let mut map = HashMap::new();
    map.insert("username", "admin");
    map.insert("password", "free5gc");
    let req = client.post("http://192.0.0.17:5000/api/login")
                    .headers(headers)
                    .json(&map)
                    .send()
                    .await.unwrap();

    let json: Value = req.json::<Value>().await.unwrap();
    let token = json["access_token"].as_str().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
    headers.insert("Accept-Enoding", "gzip, deflate".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Token", token.parse().unwrap());
    let imsi = format!("imsi-20893000000000{}", num);
    let user = "20893";
    req_json["ueID"] = Value::String(imsi.clone());
    let req = client.post(format!("http://192.0.0.17:5000/api/subscriber/{}/{}", imsi, user))
                    .headers(headers)
                    .json(&req_json)
                    .send().await.unwrap();

    println!("{:?}", req);
    //let text = req.text().await.unwrap();
    //println!("{}", text);
}

fn json_to_bollard(val: Value, extra_hosts: Vec<String>) -> Config<String> {

    let cap_add = match &val["cap_add"] {
        Value::Sequence(seq) => {
            let mut vec: Vec<String> = Vec::new();
            for i in seq {
                if let Value::String(s) = i {
                    vec.push(s.to_string());
                }
            }
            Some(vec)
        }
        _ => None,
    };

    let devices = match &val["devices"] {
        Value::Sequence(seq) => {
            let mut vec: Vec<DeviceMapping> = Vec::new();
            for i in seq {
                if let Value::String(v) = i {
                    if v.contains(":") {
                        let split = v.split(":").collect::<Vec<&str>>();
                        vec.push(DeviceMapping { 
                            path_on_host: Some(split[0].to_string()),
                            path_in_container: Some(split[1].to_string()),
                            cgroup_permissions: Some("rw".to_string()) 
                        });
                    } else {
                        vec.push(DeviceMapping { 
                            path_on_host: Some(v.clone()),
                            path_in_container: Some(v.to_string()),
                            cgroup_permissions: Some("rw".to_string()) 
                        });
                    }
                }
            }
            Some(vec)
        },
        _ => None,
    };

    let binds = match &val["volumes"] {
        Value::Sequence(seq) => {
            let mut vec: Vec<String> = Vec::new();
            for i in seq {
                if let Value::String(v) = i {
                    vec.push(v.to_string());
                }
            }
            Some(vec)
        },
        _ => None,
    };

    let oci_config = Config {
        image: Some(val["image"].as_str().unwrap().to_string()),
        cmd: Some(vec![val["command"].as_str().unwrap().to_string()]),
        host_config: Some(HostConfig {
            cap_add,
            devices,
            binds,
            network_mode: Some("none".to_string()),
            extra_hosts: Some(extra_hosts),
            ..Default::default()
        }),
        ..Default::default()
    };

    oci_config
} 

pub async fn test() {
    let id = onos::get_switch_id("s3").await;
    let res = match id {
        Some(d) => onos::get_switch_ports(d).await,
        None => return,
    };
    println!("{:?}", res);
    onos::get_hosts().await;
}
