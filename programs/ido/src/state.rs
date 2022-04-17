use crate::{AnchorDeserialize, AnchorSerialize, Space};

#[repr(u8)]
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum StateMachineWrapper {
    NotStarted(StateMachine<NotStarted>) = 0u8,
    SellRound(StateMachine<SellRound>) = 1u8,
    TradeRound(StateMachine<TradeRound>) = 2u8,
    Ended(StateMachine<Ended>) = 3u8,
}

impl Default for StateMachineWrapper {
    fn default() -> Self {
        Self::NotStarted(StateMachine::new())
    }
}

impl Space for StateMachineWrapper {
    fn space() -> usize {
        1
    }
}

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct StateMachine<S> {
    _state: S,
}
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct NotStarted {}
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct SellRound {}
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct TradeRound {}
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Ended {}

impl StateMachine<NotStarted> {
    pub fn new() -> Self {
        StateMachine {
            _state: NotStarted {},
        }
    }
}
impl From<StateMachine<NotStarted>> for StateMachine<SellRound> {
    fn from(_: StateMachine<NotStarted>) -> Self {
        StateMachine {
            _state: SellRound {},
        }
    }
}
impl From<StateMachine<SellRound>> for StateMachine<TradeRound> {
    fn from(_: StateMachine<SellRound>) -> Self {
        StateMachine {
            _state: TradeRound {},
        }
    }
}
impl From<StateMachine<TradeRound>> for StateMachine<SellRound> {
    fn from(_: StateMachine<TradeRound>) -> Self {
        StateMachine {
            _state: SellRound {},
        }
    }
}

impl From<u8> for StateMachineWrapper {
    fn from(n: u8) -> Self {
        match n {
            0 => StateMachineWrapper::NotStarted(StateMachine {
                _state: NotStarted {},
            }),
            1 => StateMachineWrapper::SellRound(StateMachine {
                _state: SellRound {},
            }),
            2 => StateMachineWrapper::TradeRound(StateMachine {
                _state: TradeRound {},
            }),
            _ => StateMachineWrapper::Ended(StateMachine { _state: Ended {} }),
        }
    }
}

impl StateMachineWrapper {
    pub fn next(self) -> Self {
        match self {
            StateMachineWrapper::NotStarted(val) => StateMachineWrapper::SellRound(val.into()),
            StateMachineWrapper::SellRound(val) => StateMachineWrapper::TradeRound(val.into()),
            StateMachineWrapper::TradeRound(val) => StateMachineWrapper::SellRound(val.into()),
            StateMachineWrapper::Ended(v) => StateMachineWrapper::Ended(v),
        }
    }
    pub fn end(self) -> Self {
        StateMachineWrapper::Ended(StateMachine { _state: Ended {} })
    }
}
