//! Types related to task management

use super::TaskContext;

use crate::{
    config::MAX_SYSCALL_NUM,
    //task::{add_syscall_count},
    //timer::get_time_us,
};

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The firsttime
    pub firsttime:usize,//tjy
    /// count syscall_times
    pub syscall_times: [u32; MAX_SYSCALL_NUM],//tjy20241030
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
