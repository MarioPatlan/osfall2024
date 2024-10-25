// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
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
	    if process.priority < self.num_levels {
            self.queues[process.priority].push(process);
            // Process placed in appropriate queue if priority value falls in valid range
        } else {
            self.queues[self.num_levels - 1].push(process);
            // If priority value falls out of range, process is placed in lowest priority queue
        }
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete
        let mut process = self.queues[queue_index].remove(0);
        // Retrieves first process at provided queue index and allows mutability of attributes
        let time_quantum = self.time_quanta[process.priority];
        // Defines time quantum given process priority
        let execution_time = time_quantum.min(process.remaining_time);
        // Execution time for process may be less than time quantum for a given process priority
        if process.remaining_time > 0 {
            // Checks if process has remaining execution time
            process.remaining_time -= execution_time;
            // Subtracts execution time from remaining_time of process
            process.total_executed_time += execution_time;
            // Adds execution time to total_executed_time of process
            self.current_time += execution_time;
            // Updates current time of MLFQ with execution time of process
            if process.remaining_time > 0 {
                // Nested loop to determine if priority increase happens
                process.priority += 1;
                // Priority is increased if time quantum was completed but process has not
                self.queues[process.priority].push(process); 
                // Process is pushed into next queue until function is called again  
            }    
        }  
    } 

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0
        let mut boost_vector = Vec::new();
        // Initialize empty vector for adding processes in lower priority queues
        for index in 0..self.num_levels {
            // Iterate through queues using indexes
            if index > 0 {
                boost_vector.append(&mut self.queues[index]);
                // Append processes in lower priority queues to the empty vector
                // Processes in highest priority not boosted to avoid further overhead
            }
        }
        for process in &mut boost_vector {
            process.priority = 0;
            // Iterate through all processes and boost to highest priority
        }
        for queue in &mut self.queues {
            queue.clear();
            // Iterate through queues in MLFQ and clear the contents
        }
        self.queues[0].append(&mut boost_vector);
        // Add contents of boost vector to the highest priority queue
        boost_vector.clear();
        // Empty boost vector after boost has completed
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
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

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
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    } 
}
