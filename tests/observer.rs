use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_replicon::{
    prelude::*,
    test_app::{ServerTestAppExt, TestClientEntity},
};
use bevy_replicon_visible_sets::prelude::*;
use serde::{Deserialize, Serialize};

#[test]
fn add_container_to_observer() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_a = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();
    let container_b = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_a].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_b)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);

    server_app
        .world_mut()
        .entity_mut(observer_entity)
        .get_mut::<VisibleObserver>()
        .unwrap()
        .containers
        .insert(container_b);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn switch_to_empty_container() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_a = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();
    let container_b = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_a].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_a)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(observer_entity)
        .get_mut::<VisibleObserver>()
        .unwrap()
        .containers
        .clear();

    server_app
        .world_mut()
        .entity_mut(observer_entity)
        .get_mut::<VisibleObserver>()
        .unwrap()
        .containers
        .insert(container_b);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn remove_container() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_a = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_a].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_a)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(observer_entity)
        .get_mut::<VisibleObserver>()
        .unwrap()
        .containers
        .remove(&container_a);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn despawn_observer() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_a = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_a].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_a)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app.world_mut().entity_mut(observer_entity).despawn();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

fn create_test_apps() -> (App, App) {
    let mut server_app = App::new();
    let mut client_app = App::new();
    for app in [&mut server_app, &mut client_app] {
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            RepliconPlugins.set(ServerPlugin::new(PostUpdate)),
            VisibilitySetPlugin,
        ))
        .replicate::<TestComponent>();
        app.finish();
    }

    (server_app, client_app)
}

fn count_entities<T: Component>(app: &mut App) -> usize {
    app.world_mut().query::<&T>().iter(app.world()).len()
}

#[derive(Component, Serialize, Deserialize)]
struct TestComponent;
