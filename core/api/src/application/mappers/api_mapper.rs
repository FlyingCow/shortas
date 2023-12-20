pub trait ApiMapper<Entity, Presenter, Payload>{
    fn to_entity(payload:Payload) -> Entity;
    fn to_api(entity: Entity) -> Presenter;
}