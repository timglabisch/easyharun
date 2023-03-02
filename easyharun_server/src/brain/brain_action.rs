pub enum BrainAction {
    ContainersStart(Vec<ContainerStart>),
    ContainersStop(Vec<ContainerStop>),
    NoOp,
}

pub struct ContainerStop {

}

pub struct ContainerStart {

}