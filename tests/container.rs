use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_replicon::{
    prelude::*,
    test_app::{ServerTestAppExt, TestClientEntity},
};
use bevy_replicon_visible_sets::prelude::*;

use serde::{Deserialize, Serialize};

#[test]
fn visible_in_is_visible() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
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
        .spawn((Replicated, TestComponent, VisibleIn(container_entity)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn move_to_unobserved_container() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let observed_container = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();
    let unobserved_container = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([observed_container].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    let entity = server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(observed_container)))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(entity)
        .insert(VisibleIn(unobserved_container));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn visible_in_without_replicated() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
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
        .spawn((TestComponent, VisibleIn(container_entity)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn replicated_without_visible_in() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    server_app.world_mut().spawn((Replicated, TestComponent));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn visible_in_added_later() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    let entity = server_app
        .world_mut()
        .spawn((Replicated, TestComponent))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);

    server_app
        .world_mut()
        .entity_mut(entity)
        .insert(VisibleIn(container_entity));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn replicated_added_later() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    let entity = server_app
        .world_mut()
        .spawn((TestComponent, VisibleIn(container_entity)))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);

    server_app.world_mut().entity_mut(entity).insert(Replicated);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn remove_replicated_component() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    let entity = server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_entity)))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(entity)
        .remove::<Replicated>();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn remove_visible_in_component() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    let entity = server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_entity)))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(entity)
        .remove::<VisibleIn>();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn remove_container_component() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
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
        .spawn((Replicated, TestComponent, VisibleIn(container_entity)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(container_entity)
        .remove::<VisibleContainer>();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn despawn_visible_in() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
        .id();

    server_app
        .world_mut()
        .entity_mut(client_entity)
        .get_mut::<ClientVisible>()
        .unwrap()
        .observers
        .insert(observer_entity);

    let entity = server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container_entity)))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app.world_mut().entity_mut(entity).despawn();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn despawn_container() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let observer_entity = server_app
        .world_mut()
        .spawn(VisibleObserver::new([container_entity].into()))
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
        .spawn((Replicated, TestComponent, VisibleIn(container_entity)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(container_entity)
        .despawn();

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
