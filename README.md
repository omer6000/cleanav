# Dependable Systems Projects: CLEANAV & MARKOCEAN

## Overview

This repository contains two hands-on projects, CLEANAV and MARKOCEAN, developed as part of the Hands-on Dependability course. The projects explore dependability in system design through practical applications.

- **CLEANAV**: Focuses on designing a dependable navigation system for a cleaning robot, ensuring reliability and safety.
- **MARKOCEAN**: Expands on the principles learned in CLEANAV by simulating environmental effects on vessel movement to predict outcomes using Markov models.

## Project Aim

The main objective is to apply concepts of dependability in system design to real-world scenarios, enhancing the reliability and safety of autonomous systems. CLEANAV aims to create a dependable navigation system for a cleaning robot, whereas MARKOCEAN deals with analyzing and predicting vessel movements under environmental effects.

## Technologies Used

- Rust Programming Language
- `nalgebra` crate for matrix operations and simulations
- Markov Models for predicting environmental impact on movements in MARKOCEAN

## Features

- **CLEANAV**:
  - Safe and reliable navigation algorithm for cleaning robots.
  - Implementation of fault tolerance mechanisms.

- **MARKOCEAN**:
  - Simulation of environmental effects, like wind, on vessel movement.
  - Use of Markov models for movement prediction and dependability analysis.

## Getting Started

Ensure Rust and Cargo are installed on your system. Clone this repository, navigate to each project's directory, use `cargo build` to compile, and `cargo run` to execute the programs.

## Contributions

Developed as part of a course on Hands-on Dependability, contributions are welcome to extend functionalities or improve simulation accuracy.
