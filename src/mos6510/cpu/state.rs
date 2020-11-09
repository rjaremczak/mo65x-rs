#[derive(PartialEq, Debug)]
pub struct CpuState {
    exec_level: ExecLevel,
    exec_state: ExecState,
}

#[derive(PartialEq, Debug)]
pub enum ExecLevel {
    Normal = 0,
    PendingIrq = 1,
    PendingNmi = 2,
    PendingReset = 3,
}

#[derive(PartialEq, Debug)]
pub enum ExecState {
    Idle = 0,
    Stopped = 1,
    Halted = 2,
    Running = 3,
    Stopping = 4,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            exec_level: ExecLevel::Normal,
            exec_state: ExecState::Idle,
        }
    }

    pub fn try_set_idle(&mut self) {
        match self.exec_state {
            ExecState::Stopped | ExecState::Halted => self.exec_state = ExecState::Idle,
            _ => {}
        }
    }

    pub fn try_stopping(&mut self) {
        match self.exec_state {
            ExecState::Running => self.exec_state = ExecState::Stopping,
            _ => {}
        }
    }

    pub fn start_running(&mut self) {
        self.exec_state = ExecState::Running;
    }

    pub fn try_stop_running(&mut self) {
        match self.exec_state {
            ExecState::Running => self.exec_state = ExecState::Idle,
            ExecState::Stopping => self.exec_state = ExecState::Stopped,
            _ => {}
        }
    }

    pub fn is_running(&self) -> bool {
        self.exec_state == ExecState::Running
    }
}
