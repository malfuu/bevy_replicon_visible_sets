use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_replicon_visible_sets::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_replicon::prelude::*;
use bevy_replicon::test_app::{ServerTestAppExt, TestClientEntity};

#[test]
fn no_entities_visible() {
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

    server_app.connect_client(&mut client_app);

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    let remote_count = count_entities::<Remote>(&mut client_app);
    assert_eq!(remote_count, 0);
}

#[test]
fn entity_visible() {
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

    server_app.connect_client(&mut client_app);

    let client_entity = **client_app.world().resource::<TestClientEntity>();

    let container = server_app
        .world_mut()
        .spawn(VisibleContainer::default())
        .id();

    let mut observer_set = EntityHashSet::default();
    observer_set.insert(container);
    let observer = server_app
        .world_mut()
        .spawn(VisibleObserver(observer_set))
        .id();

    let mut client_visible_set = EntityHashSet::default();
    client_visible_set.insert(observer);
    server_app
        .world_mut()
        .entity_mut(client_entity)
        .insert(ClientVisible(client_visible_set));

    server_app
        .world_mut()
        .spawn((Replicated, TestComponent, VisibleIn(container)));

    server_app.update();
    server_app.exchange_with_client(&mut client_app);
    client_app.update();

    let remote_count = count_entities::<TestComponent>(&mut client_app);

    assert_eq!(remote_count, 1);
}

#[derive(Component, Serialize, Deserialize)]
struct TestComponent;

fn count_entities<T: Component>(app: &mut App) -> usize {
    app.world_mut().query::<&T>().iter(app.world()).len()
}
