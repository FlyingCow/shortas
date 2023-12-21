pub trait DbMapper<Entity, DbModel> {
    fn to_db(entity: Entity) -> Option<DbModel>;
    fn to_entity(model: DbModel) -> Option<Entity>;
}

pub trait ToEntityMapper<DbModel, Entity> {
    fn to_entity(model: DbModel) -> Option<Entity>;
}

pub trait ToDbModelMapper<Entity, DbModel> {
    fn to_db(entity: Entity) -> Option<DbModel>;
}
