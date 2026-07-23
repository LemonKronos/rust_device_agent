use serde_json::{json, Map, Value};
use std::time::Duration;
use tokio::time::{sleep_until, Instant};
use tokio::signal;

use gsoft_device_agent::info_gatherer::Info;
use gsoft_device_agent::config_handler::*;
use gsoft_device_agent::scheduler::*;

fn insert_nested(mut current_map: &mut Map<String, Value>, path: &[&str], val: Value) {
    // Split into ["disk", "physical", "partition"] and "name"
    let (last, prefixes) = path.split_last().expect("Path cannot be empty");
    
    for &prefix in prefixes {
        let next_val = current_map
            .entry(prefix.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
            
        if let Value::Object(map) = next_val {
            current_map = map; // Move deeper into the tree
        } else {
            return; // Safety fallback in case of a JSON type conflict
        }
    }
    
    // Insert the final value at the deepest level
    current_map.insert(last.to_string(), val);
}

pub async fn run_agent_loop(mut queue: TimerWheel) {
    println!("Agent running. Timer wheel engaged.");
    let mut info = Info::new();

    loop {
        // --- STEP 1: Find next wake up time ---
        let next_wake = match queue.peek() {
            Some(task) => task.execute_at,
            None => {
                println!("Not found any task, sleep for 5 sec");
                Instant::now() + Duration::from_secs(5) // Sleep if empty
            },
        };

        // --- STEP 2: Sleep till then (with shutdown intercept) ---
        tokio::select! {
            _ = sleep_until(next_wake) => {
                // Time's up, proceed to execution
            }
            _ = signal::ctrl_c() => {
                println!("Shutdown signal received. Saving state...");
                // config_handler::save_config("doc/config.json", &queue);
                break; // Exit the loop safely
            }
        }

        // --- STEP 3: Pop the batch ---
        info.prepare();
        let now = Instant::now();
        let mut batch = Vec::new();

        while let Some(task) = queue.peek() {
            if task.execute_at <= now {
                batch.push(queue.pop().expect("I was just peeking it, where is it now?"));
            } else {
                break; // Hit a future task, stop popping
            }
        }

        if batch.is_empty() {
            continue; // Safety catch
        }

        // --- STEP 4: Execute, Diff, JSON, Reschedule ---
        // Just one single map for the whole wake-up cycle!
        let mut payload_map = Map::new();

        for mut task in batch {
            match task.id {
                TaskID::GeneralRunTime => {
                    let current_uptime = info.get_up_time();

                    let should_send = match task.last_value {
                        Some(AgentValue::Int(last)) => current_uptime != last,
                        _ => true,
                    };

                    if should_send {
                        task.last_value = Some(AgentValue::Int(current_uptime));
                        
                        // Inject straight into the master payload map!
                        insert_nested(
                            &mut payload_map, 
                            "general", 
                            "run_time", 
                            json!(current_uptime)
                        );
                    }
                },
                TaskID::GeneralBootTime => {
                    let current_bootime = info.get_boot_time();

                    let should_send = match task.last_value {
                        Some(AgentValue::Int(last)) => current_bootime != last,
                        _ => true,
                    };

                    if should_send {
                        task.last_value = Some(AgentValue::Int(current_bootime));

                        insert_nested(&mut payload_map,
                            "general",
                            "boot_time",
                            json!(current_bootime)
                        );
                    }
                },
                TaskID::CpuUsage => {
                    let current_cpu = info.get_cpu_usage();
                    
                    // ... assuming the diff logic passes ...
                    
                    // This will automatically create the "cpu" object if it doesn't exist
                    insert_nested(
                        &mut payload_map, 
                        "cpu", 
                        "usage",
                        json!(current_cpu)
                    );
                }
                // ... handle other TaskIds
                _ => {}
            }

            // Reschedule the task
            if task.cycle_time > 0 {
                task.execute_at = tokio::time::Instant::now() + Duration::from_secs(task.cycle_time);
                queue.push(task); 
            }
        }

        // --- STEP 5: Hand the JSON to sender ---
        // If the payload map isn't empty, it means at least one category was created
        if !payload_map.is_empty() {
            // Add your static root identifiers
            payload_map.insert("agent_version".to_string(), json!("0.3.0.scheduler"));
            
            let final_json = Value::Object(payload_map);
            println!("Sending Payload: {}", serde_json::to_string_pretty(&final_json).unwrap());
            
            // tokio::spawn(async move {
            //     send_to_server(final_json).await;
            // });
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Booting Agent...");
    let wheel = load_config();

    run_agent_loop(wheel).await;
}