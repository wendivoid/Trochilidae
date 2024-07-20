use crate::environment::{EnvCondition, EnvOperator, EnvValue, EnvVariable, Environment};

pub struct Habitability {
    pub spawn: Vec<EnvCondition>,
    pub daily: Vec<EnvCondition>,
    pub yearly: Vec<EnvCondition>,
}

impl Habitability {
    pub fn should_spawn(&self, environment: Environment) -> bool {
        let mut to_spawn = true;
        for env_cond in self.spawn.iter() {
            match env_cond.op {
                EnvOperator::Equal => match env_cond.var {
                    EnvVariable::Elevation => match &env_cond.val {
                        &EnvValue::Val(x) => to_spawn = environment.elevation == x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.elevation == environment.elevation,
                            EnvVariable::Water => to_spawn = environment.elevation == environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.elevation == environment.moisture,
                        },
                    },
                    EnvVariable::Water => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.water_table == *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.water_table == environment.elevation,
                            EnvVariable::Water => to_spawn = environment.water_table == environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.water_table == environment.moisture,
                        },
                    },
                    EnvVariable::Moisture => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.moisture < *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.moisture == environment.elevation,
                            EnvVariable::Water => to_spawn = environment.moisture == environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.moisture == environment.moisture,
                        },
                    },
                },
                EnvOperator::Greater => match env_cond.var {
                    EnvVariable::Elevation => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.elevation > *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.elevation > environment.elevation,
                            EnvVariable::Water => to_spawn = environment.elevation > environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.elevation > environment.moisture,
                        },
                    },
                    EnvVariable::Water => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.water_table > *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.water_table > environment.elevation,
                            EnvVariable::Water => to_spawn = environment.water_table > environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.water_table > environment.moisture,
                        },
                    },
                    EnvVariable::Moisture => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.moisture > *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.moisture > environment.elevation,
                            EnvVariable::Water => to_spawn = environment.moisture > environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.moisture > environment.moisture,
                        },
                    },
                },
                EnvOperator::Less => match env_cond.var {
                    EnvVariable::Elevation => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.elevation < *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.elevation < environment.elevation,
                            EnvVariable::Water => to_spawn = environment.elevation < environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.elevation < environment.moisture,
                        },
                    },
                    EnvVariable::Water => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.water_table < *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.water_table < environment.elevation,
                            EnvVariable::Water => to_spawn = environment.water_table < environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.water_table < environment.moisture,
                        },
                    },
                    EnvVariable::Moisture => match &env_cond.val {
                        EnvValue::Val(x) => to_spawn = environment.moisture < *x,
                        EnvValue::Var(var) => match var {
                            EnvVariable::Elevation => to_spawn = environment.moisture < environment.elevation,
                            EnvVariable::Water => to_spawn = environment.moisture < environment.water_table,
                            EnvVariable::Moisture => to_spawn = environment.moisture < environment.moisture,
                        },
                    },
                },
            }
        }
        to_spawn
    }
}
