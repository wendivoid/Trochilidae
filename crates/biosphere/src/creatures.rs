use crate::{
    environment::{EnvCondition, EnvOperator, EnvValue, EnvVariable},
    phenotype::{Anatomy, Habitability, Phenotype},
};

pub fn monopodial() -> Phenotype {
    let spawn = vec![EnvCondition {
        var: EnvVariable::Elevation,
        op: EnvOperator::Greater,
        val: EnvValue::Var(EnvVariable::Water),
    }];
    Phenotype {
        lifespan: 25.0,
        anatomy: Anatomy::Vascular(plants::vascular::monopodial()),
        habitability: Habitability {
            spawn,
            daily: Default::default(),
            yearly: Default::default(),
        },
    }
}

pub fn sympodial() -> Phenotype {
    let spawn = vec![EnvCondition {
        var: EnvVariable::Elevation,
        op: EnvOperator::Greater,
        val: EnvValue::Var(EnvVariable::Water),
    }];
    Phenotype {
        lifespan: 15.0,
        anatomy: Anatomy::Vascular(plants::vascular::sympodial()),
        habitability: Habitability {
            spawn,
            daily: Default::default(),
            yearly: Default::default(),
        },
    }
}
