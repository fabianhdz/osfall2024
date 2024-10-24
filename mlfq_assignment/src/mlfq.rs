// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize, // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // TODO: Implement this function
        // Add the process to the appropriate queue based on its priority
        // Ensure the priority is within the valid range (0 to num_levels - 1)
        
        // Ensures priority is within valid range (0 to num_levels -1)
        if process.priority >= 0 && process.priority <= self.num_levels - 1 {
            self.queues[process.priority].push(process);
        } else {
            //if out of range, lowest priority
            self.queues[self.num_levels - 1].push(process);
        }
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete
        // Check if queue_index is within bound of queue
        if !(queue_index < self.num_levels) {
            println!("Index out of bounds");
            return;
        }
        // Check if queue is empty
        if self.queues[queue_index].is_empty() {
            return;
        }

        //  collect process that need to be removed
        let mut to_remove = Vec::new();

        // Iterate through the processes in the current queue
        for i in 0..self.queues[queue_index].len() {
            let mut process = self.queues[queue_index][i].clone();

            // If process is already finished, mark it for removal
            if process.remaining_time <= 0 {
                to_remove.push(i);
                println!("Process {} is finished.", process.id);
            } else {
                // Update process times
                let time_quantum = self.time_quanta[queue_index];
                let execute_time = time_quantum.min(process.remaining_time);
                process.remaining_time -= execute_time;
                process.total_executed_time += execute_time;
                self.current_time += execute_time;

                if process.remaining_time <= 0 {
                    println!("Process {} completed.", process.id);
                    to_remove.push(i); // Mark it for removal since it's finished
                } else {
                    // Move the process to the lower priority queue if time quantum is fully used
                    let new_priority = (queue_index + 1).min(self.num_levels - 1);
                    process.priority = new_priority;
                    self.queues[new_priority].push(process.clone()); // Move to the next queue
                    to_remove.push(i); // Mark it for removal from the current queue
                }
            }
        }
        // Remove process from queues
        for &i in to_remove.iter().rev() {
            self.queues[queue_index].remove(i);
        }
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0
        
        // Vector to store process 
        let mut processes = Vec::new();

        // Move all lower priority processes to a single vector
        for queue_index in 1..self.num_levels {
            processes.append(&mut self.queues[queue_index]);
        }
        // Push all processes to highest priority queue
        for mut process in processes {
            process.priority = 0;
            self.queues[0].push(process);
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        let process1 = Process {
            id: 1,
            priority: 0,
            remaining_time: 10,
            total_executed_time: 0,
        };
        let process2 = Process {
            id: 2,
            priority: 1,
            remaining_time: 5,
            total_executed_time: 0,
        };
        let process3 = Process {
            id: 3,
            priority: 5,
            remaining_time: 8,
            total_executed_time: 0,
        };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process {
            id: 1,
            priority: 0,
            remaining_time: 5,
            total_executed_time: 0,
        });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process {
            id: 1,
            priority: 1,
            remaining_time: 5,
            total_executed_time: 3,
        });
        mlfq.queues[2].push(Process {
            id: 2,
            priority: 2,
            remaining_time: 3,
            total_executed_time: 7,
        });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process {
            id: 1,
            priority: 1,
            remaining_time: 5,
            total_executed_time: 3,
        });

        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}
