use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_replicon::{prelude::*, test_app::ServerTestAppExt};
use bevy_replicon_visible_sets::prelude::*;
use serde::{Deserialize, Serialize};

#[test]
fn always_visible() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, AlwaysVisible));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn always_visible_added_later() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

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
        .insert(AlwaysVisible);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn always_visible_removed() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let entity = server_app
        .world_mut()
        .spawn((Replicated, TestComponent, AlwaysVisible))
        .id();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);

    server_app
        .world_mut()
        .entity_mut(entity)
        .remove::<AlwaysVisible>();

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn always_visible_ignore_container() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    let container_entity = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    server_app.world_mut().spawn((
        Replicated,
        TestComponent,
        VisibleIn(container_entity),
        AlwaysVisible,
    ));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
}

#[test]
fn always_visible_without_replicated() {
    let (mut server_app, mut client_app) = create_test_apps();
    server_app.connect_client(&mut client_app);

    server_app.world_mut().spawn((TestComponent, AlwaysVisible));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 0);
}

#[test]
fn always_visible_present_on_late_join() {
    let (mut server_app, mut client_app) = create_test_apps();

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, AlwaysVisible));

    server_app.update();

    server_app.connect_client(&mut client_app);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    assert_eq!(count_entities::<TestComponent>(&mut client_app), 1);
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
