#!/bin/bash
// Augusto Estevão [2023] Copyright
use std::{io, process::Command};

fn menu() -> i32 {
    let mut usr_choice: String = String::new();
    println!("--------------------------------------");
    println!("[1] - Start a service");
    println!("[2] - Verify active services");
    println!("[3] - Vertify open ports");
    println!("[4] - Stop a service");
    println!("[0] - Exit");
    println!("--------------------------------------");
    println!("Enter here a number: ");
    io::stdin()
        .read_line(&mut usr_choice)
        .expect("Error reading number");
    let usr_choice: i32 = match usr_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    return usr_choice;
}

fn start_service() -> String {
    let mut service: String = String::new();
    println!("Enter here the name of service: ");
    io::stdin()
        .read_line(&mut service)
        .expect("Invalid service name!");

    let service = service.trim(); // Remove whitespaces

    Command::new("service")
        .arg(&service)
        .arg("restart")
        .spawn()
        .expect(&format!(
            "Error starting the service {} [-]",
            service.trim()
        ));
    println!("The service {service} has successfully started [+]");
    return service.to_string();
}

fn list_all_services(service: &str) {
    let mut final_service = service.to_string();
    if service.is_empty() {
        println!("Enter the name of the service: ");
        io::stdin()
            .read_line(&mut final_service)
            .expect("Error reading the input");
        final_service = final_service.trim().to_string();
    }

    let full_command = format!("ps aux | grep '{}'", final_service);
    Command::new("sh")
        .arg("-c")
        .arg(&full_command)
        .spawn()
        .expect("Failed to list all services [-]");
}

fn list_all_open_ports() {
    Command::new("netstat")
        .arg("-nlpt")
        .spawn()
        .expect("Failed to list all open ports [-]");
}

fn stop_service() {
    let mut final_service = String::new();
        println!("Enter the name of the service to be stoped: ");
        io::stdin()
            .read_line(&mut final_service)
            .expect("Error reading the input");
        final_service = final_service.trim().to_string();

    let full_command = format!("service {} stop", final_service);
    Command::new("sh")
        .arg("-c")
        .arg(&full_command)
        .spawn()
        .expect(&format!(
            "Error stoping the service {} [-]",
            final_service.trim()
        ));
}

fn handle_usr_choice() {
    let mut service: String = String::new();
    loop {
        let choice: i32 = menu();
        match choice {
            0 => break,
            1 => service = start_service(),
            2 => list_all_services(&service),
            3 => list_all_open_ports(),
            4 => stop_service(),
            _ => println!("Invalid choice!"),
        }
    }
}
fn main() {
    handle_usr_choice();
}
