/*!
Entity visibility with the usage of Potentially Visible Sets and Layers.

# Quick Start

there is no quick start.

<div class="warning">
//! > Using Replicon's high level [bevy_replicon::server::visibility::AppVisibilityExt] might cause
//! > conflicts with this crate. Do not use it.
</div>

# Clients

All Client entities have [`ClientVisible`] added to them, as a registered require component.

# Entity Visibility

I should also add some details here.

# Component Visibility

Component Visibility is not implemented yet.
*/
#![deny(missing_docs)]

/// Potentially Visible Set containers.
pub mod container;
/// Observers that see inside containers.
pub mod observer;
/// Visibility Masks.
pub mod visibility_mask;

#[doc(hidden)]
pub mod prelude;

#[doc(hidden)]
use bevy::{ecs::entity::EntityHashSet, prelude::*};
use bevy_replicon::{
    prelude::*,
    server::visibility::{
        client_visibility::ClientVisibility, filters_mask::FilterBit, registry::FilterRegistry,
    },
    shared::replication::registry::ReplicationRegistry,
};

use crate::{container::update_visible_containers, observer::update_visible_observers};

/// Plugin that enables visible set management.
pub struct VisibilitySetPlugin;

impl Plugin for VisibilitySetPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<ServerPlugin>(),
            "Replicon ServerPlugin is required!"
        );

        app.init_resource::<VisibleBit>()
            .configure_sets(
                PostUpdate,
                (
                    VisibilitySystems::First,
                    VisibilitySystems::Blacklisting,
                    VisibilitySystems::Update,
                    VisibilitySystems::Last,
                )
                    .chain()
                    .before(ServerSystems::Send),
            )
            .add_systems(
                PostUpdate,
                (
                    new_clients,
                    new_replicateds,
                    removed_replicateds,
                    always_visible_added,
                    always_visible_removed,
                )
                    .in_set(VisibilitySystems::Blacklisting),
            )
            .add_systems(
                PostUpdate,
                (
                    update_visible_containers,
                    update_visible_observers,
                    update_client_visible,
                    sync_client_visible,
                )
                    .chain()
                    .in_set(VisibilitySystems::Update),
            )
            .register_required_components::<ClientVisibility, ClientVisible>();
    }
}

/// System set for updating client visibility.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VisibilitySystems {
    /// First step in visibility updates. Empty by default.
    First,
    /// Step for blacklisting visibility all entities,
    /// effectively making it whitelist only.
    /// Required due to replicon's blacklist nature.
    Blacklisting,
    /// General visibility updates.
    Update,
    /// Final step in visibility updates. Empty by default.
    Last,
}

/// Contains what [`VisibleObserver`]s a client can see through.
/// Crate users must manually populate `observers` with entities containing [`VisibleObserver`].
#[derive(Component, Default, Debug)]
pub struct ClientVisible {
    /// Entities containing [`VisibleObserver`] that this client sees through.
    pub observers: EntityHashSet,
    /// Cached union of all entities visible to this client.
    visible_entities: EntityHashSet,
    /// Cached union of all entities visible to this client in the previous update, used to diff visibility changes.
    previous_visible_entities: EntityHashSet,
}

impl ClientVisible {
    /// Creates a new `ClientVisible` with the given observers.
    pub fn new(observers: EntityHashSet) -> Self {
        Self {
            observers,
            ..default()
        }
    }
}

/// Marks an entity as always visible to all clients.
#[derive(Component, Reflect, Default, Debug)]
pub struct AlwaysVisible;

#[derive(Resource, Deref, Debug)]
struct VisibleBit(FilterBit);

impl FromWorld for VisibleBit {
    fn from_world(world: &mut World) -> Self {
        let bit = world.resource_scope(|world, mut filter_registry: Mut<FilterRegistry>| {
            world.resource_scope(|world, mut registry: Mut<ReplicationRegistry>| {
                filter_registry.register_scope::<Entity>(world, &mut registry)
            })
        });
        Self(bit)
    }
}

fn new_clients(
    mut new_clients: Query<&mut ClientVisibility, Added<AuthorizedClient>>,
    entities: Query<(Entity, Has<AlwaysVisible>), With<Replicated>>,
    bit: Res<VisibleBit>,
) {
    for mut visibility in &mut new_clients {
        for (entity, always_visible) in &entities {
            visibility.set(entity, **bit, always_visible);
        }
    }
}

fn new_replicateds(
    entities: Query<(Entity, Has<AlwaysVisible>), Added<Replicated>>,
    mut clients: Query<(&mut ClientVisibility, &ClientVisible)>,
    bit: Res<VisibleBit>,
) {
    for (mut visibility, client_visible) in &mut clients {
        for (entity, always_visible) in &entities {
            let is_visible = always_visible || client_visible.visible_entities.contains(&entity);
            visibility.set(entity, **bit, is_visible);
        }
    }
}

fn removed_replicateds(
    mut entities: RemovedComponents<Replicated>,
    mut clients: Query<&mut ClientVisibility>,
    bit: Res<VisibleBit>,
) {
    let removed: Vec<Entity> = entities.read().collect();

    for mut visibility in &mut clients {
        for &entity in &removed {
            visibility.set(entity, **bit, true);
        }
    }
}

fn always_visible_added(
    entities: Query<Entity, (Added<AlwaysVisible>, With<Replicated>)>,
    mut clients: Query<&mut ClientVisibility>,
    bit: Res<VisibleBit>,
) {
    for mut visibility in &mut clients {
        for entity in &entities {
            visibility.set(entity, **bit, true);
        }
    }
}

fn always_visible_removed(
    mut removed: RemovedComponents<AlwaysVisible>,
    replicated_entities: Query<Entity, With<Replicated>>,
    mut clients: Query<(&mut ClientVisibility, &ClientVisible)>,
    bit: Res<VisibleBit>,
) {
    if removed.is_empty() {
        return;
    }

    let removed_entities: EntityHashSet = removed.read().collect();

    for (mut visibility, client_visible) in &mut clients {
        for &entity in &removed_entities {
            if !replicated_entities.contains(entity) {
                continue;
            }

            let is_visible = client_visible.visible_entities.contains(&entity);
            visibility.set(entity, **bit, is_visible);
        }
    }
}

fn update_client_visible(
    mut clients: Query<(&mut ClientVisibility, &ClientVisible), Changed<ClientVisible>>,
    bit: Res<VisibleBit>,
) {
    for (mut visibility, client_visible) in clients.iter_mut() {
        let current = &client_visible.visible_entities;
        let previous = &client_visible.previous_visible_entities;

        let appearing_entities = current.difference(previous);
        let disappearing_entities = previous.difference(current);

        for &entity in appearing_entities {
            visibility.set(entity, **bit, true);
        }
        for &entity in disappearing_entities {
            visibility.set(entity, **bit, false);
        }
    }
}

fn sync_client_visible(mut clients: Query<Mut<ClientVisible>, Changed<ClientVisible>>) {
    for mut client_visible in clients.iter_mut() {
        let client_visible = client_visible.bypass_change_detection();
        client_visible
            .previous_visible_entities
            .clone_from(&client_visible.visible_entities);
    }
}
