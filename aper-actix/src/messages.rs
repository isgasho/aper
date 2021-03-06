use actix::{Addr, Message};
use aper::{StateMachine, StateUpdateMessage};

use crate::player_actor::PlayerActor;

/// A [StateUpdateMessage], wrapped in a new struct so that we can implement
/// actix's [Message] trait on it.
#[derive(Message)]
#[rtype(result = "()")]
pub struct WrappedStateUpdateMessage<State: StateMachine>(pub StateUpdateMessage<State>);

/// A message received by a [crate::ChannelActor].
#[derive(Message)]
#[rtype(result = "()")]
pub enum ChannelMessage<State: StateMachine> {
    /// A new player has joined this channel.
    Connect(Addr<PlayerActor<State>>, String),

    /// A transition has been received from a player. Includes the address of the sending
    /// [PlayerActor].
    Event(Addr<PlayerActor<State>>, State::Transition),

    /// A transition is occurring because a suspended transition was triggered.
    Tick(State::Transition),
}
