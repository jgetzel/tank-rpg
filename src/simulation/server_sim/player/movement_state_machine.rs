use bevy::prelude::Component;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::Velocity;
use crate::simulation::server_sim::player::{calc_player_next_velocity, Player, PlayerInput};
use crate::utils::math::state_machine::{StateBehavior, StateMachine};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MovementState {
    Idle,
    Moving,
}

type MovementStateParams<'a> = (&'a Player, &'a PlayerInput, &'a mut Velocity, f32);

type MovementStateBehavior = Box<dyn StateBehavior<MovementStateMachine>>;

#[derive(Component)]
pub struct MovementStateMachine {
    state: MovementState,
    behaviors: HashMap<MovementState, MovementStateBehavior>,
}

impl Default for MovementStateMachine {
    fn default() -> Self {
        let mut behaviors: HashMap<MovementState, MovementStateBehavior> = HashMap::new();
        behaviors.insert(MovementState::Idle, Box::new(IdleBehavior));
        behaviors.insert(MovementState::Moving, Box::new(MovingBehavior));

        Self {
            state: MovementState::Idle,
            behaviors,
        }
    }
}


impl StateMachine for MovementStateMachine {
    type State = MovementState;
    type Params<'a> = MovementStateParams<'a>;

    fn update(&mut self, params: MovementStateParams) {
        if let Some(behavior) = self.behaviors.get(&self.state) {
            if let Some(new_state) = behavior.update(params) {
                self.state = new_state;
            };
        }
    }
}

struct IdleBehavior;

struct MovingBehavior;

impl StateBehavior<MovementStateMachine> for IdleBehavior {
    fn update(&self, params: MovementStateParams) -> Option<MovementState> {
        if params.1.movement.length_squared() > 0.0 {
            Some(MovementState::Moving)
        } else {
            None
        }
    }
}

impl StateBehavior<MovementStateMachine> for MovingBehavior {
    fn update(&self, params: MovementStateParams) -> Option<MovementState> {
        let (player, input, vel, delta) = params;
        vel.linvel = calc_player_next_velocity(vel.linvel, player, input, delta);
        if input.movement.length_squared() == 0.0 {
            Some(MovementState::Idle)
        } else {
            None
        }
    }

}
