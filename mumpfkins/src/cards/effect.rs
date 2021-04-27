use std::sync::Arc;

pub struct Effect {
    name: String,
    effect: Arc<dyn Fn()>
}

pub struct FightEffect {

}