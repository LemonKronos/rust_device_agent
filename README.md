
This is a non-AI application built to gather device information from Endpoint machines and send it to a Central Server for monitoring purposes. Core functions include gathering static (hardware-oriented) and dynamic (software-oriented) machine information, sending heartbeats, and handling command responses from the Central Server.

# Overview

The main goal of this Agent is to be lightweight and robust, running almost silently on Endpoint machines with a minimal CPU and memory footprint.
The Agent is built with separated binaries for security concerns. It includes the following:

1. `launcher`: Extremely **lightweight** and **stable**. It manages versioning, updates, and downloading binaries. This binary requires administrator rights, which it will request and gain during installation. For security reasons, all download URLs are **hardcoded** to the Central Server repository.
    
2. `main_worker`: The core binary that handles configuration, scheduling, information gathering, networking, and server commands. This binary **does not** have administrator privileges.

3. `admin_fetcher`: An **optional** binary that **has administrator privileges** to scan protected machine information. This is called by the `main_worker` when needed for a one-pass "run and die" execution. This binary **must not** have any networking capabilities and is strictly restricted to read-only information gathering.
    
4. `proxy_scanner`: An **optional** binary that promotes the agent to a `Proxy Scanner`. In this mode, the agent not only gathers its own information but also actively scans for device information within its configured subnet.

The agent is designed to be **highly customizable** to customer needs. All configuration should be handled through the Central Server and **cannot** be changed by the Endpoint user.

The agent is currently built for Windows and Linux (Ubuntu), with future work targeting macOS and other common Linux server distros. The codebase utilizes shared logic for all operating systems alongside an `OsSpecific` module to handle OS differences. Developers can easily switch targets during the build process to generate OS-specific binaries and packages.

The Agent is "lazy and smart." It only scans when explicitly asked by the server or when scheduled by the config. It then compares the new data with its cache to ensure it only sends information that **has changed**. For numeric data, there is a customizable threshold limit in the config to determine how much of a variance is considered a valid change. If there are drastic changes (e.g., a newly plugged-in USB or a new disk partition), the agent will send the full information for that specific topic. The cache is re-initialized every time the machine starts.

The heartbeat signal is designated as `general:run_time` since that value continuously changes as the machine runs. Therefore, the heartbeat cycle is simply tied to whatever the `general:run_time` scan cycle is set to.

The scheduler is adaptive and lightweight (using a min-heap), allowing the scan cycle to scale from a single second to extremely long intervals. The scheduler is the config, and the config is the scheduler.

# Design Architecture
- For Web view:
![Gsoft Agent Architecture](../../../doc/figure/design_architecture.png)
- For Markdown view: 
![Gsoft Agent Architecture](doc/figure/design_architecture.png)
# Agent Life Cycle

- **Start-up**
    - The Agent initializes and loads its local config as a scheduler.
    - Performs a full scan (configurable by devs, but should generally be left as `true`).
    - Sends information to the server and handles any server response commands.
    - Sleeps until the next scheduled task (interruptible).
- **Scheduled Cycle**
    - Wakes up.
    - Processes a batch of due tasks.
    - Gathers the information dictated by the batch.
    - Sends information to the server and handles any server response.
    - Sleeps until the next scheduled task (interruptible).
- **Normal Shutdown Interrupt**
    - Sends a shutdown heartbeat to the server.
    - Saves the current scheduler state as the config.
___
