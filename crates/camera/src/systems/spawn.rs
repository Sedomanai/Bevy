pub fn spawn_camera(
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<(&Camera, &Projection)>,
        Query<(&mut Camera, &mut Projection)>,
        Query<(&Camera, &mut Projection)>,
    )>,
) {
    for (cam, proj) in queries.p0().iter() {}
}
