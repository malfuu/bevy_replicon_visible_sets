//! Entity visibility with the usage of Potentially Visible Sets and Layers.
//!
//! # Quick Start
//!
//! there is no quick start.
//!
//!
//! > [!CAUTION]
//! > Using Replicon's high level [bevy_replicon::server::visibility::AppVisibilityExt] might cause
//! > conflicts with this crate. Do not use it.
//!
//! # Entity Visibility
//!
//! Entities
//!
//! # Component Visibility
//!
//! Component Visibility is not implemented yet.
#![deny(missing_docs)]

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
                (new_clients, new_replicateds, removed_replicateds)
                    .in_set(VisibilitySystems::Blacklisting),
            )
            .add_systems(
                PostUpdate,
                (
                    update_visible_containers,
                    update_visible_observers,
                    update_client_visible,
                )
                    .in_set(VisibilitySystems::Update),
            );
    }
}

/// System set for updating client visibility.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VisibilitySystems {
    /// First step in visibility updates. Empty by default.
    First,
    /// Step for blacklisting visibility all entities,
    /// effectively turning visibility whitelist only.
    /// Required due to replicon's blacklist nature.
    Blacklisting,
    /// General visibility updates.
    Update,
    /// Final step in visibility updates. Empty by default.
    Last,
}

/// Contains what [`VisibleObserver`]s a client can see through.
/// Crate users must manually populate this hashset with entities containing [`VisibleObserver`].
#[derive(Component)]
pub struct ClientVisible(pub EntityHashSet);

/// An entity that can see entities inside of containers.
/// This can be given to a a player's character or a long distanced camera.
/// Crate users must manually populate this hashset with entities containing [`VisibleContainer`].
#[derive(Component)]
pub struct VisibleObserver(pub EntityHashSet);

/// Which [`VisibleContainer`] this entity is visible in.
#[derive(Component)]
#[relationship(relationship_target = VisibleContainer)]
pub struct VisibleIn(pub Entity);

/// Entities can be added and removed from this container using the [`VisibleIn`] relationship.
#[derive(Component, Default, Debug)]
#[relationship_target(relationship = VisibleIn)]
pub struct VisibleContainer {
    #[relationship]
    contained: EntityHashSet,
}

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
    entities: Query<Entity, With<Replicated>>,
    bit: Res<VisibleBit>,
) {
    for entity in &entities {
        for mut visibility in &mut new_clients {
            visibility.set(entity, **bit, false);
        }
    }
}

fn new_replicateds(
    entities: Query<Entity, Added<Replicated>>,
    mut clients: Query<&mut ClientVisibility>,
    bit: Res<VisibleBit>,
) {
    for entity in &entities {
        for mut visibility in &mut clients {
            visibility.set(entity, **bit, false);
        }
    }
}

fn removed_replicateds(
    mut entities: RemovedComponents<Replicated>,
    mut clients: Query<&mut ClientVisibility>,
    bit: Res<VisibleBit>,
) {
    for entity in entities.read() {
        for mut visibility in &mut clients {
            visibility.set(entity, **bit, true);
        }
    }
}

fn update_visible_containers(
    _containers: Query<Ref<VisibleContainer>>,
    mut _observers: Query<Mut<VisibleObserver>>,
) {
    todo!()
}

fn update_visible_observers(
    mut _clients: Query<&mut ClientVisibility>,
    _observers: Query<&VisibleObserver>,
) {
    todo!()
}

fn update_client_visible(
    mut _clients: Query<(&mut ClientVisibility, &ClientVisible)>,
    _observers: Query<&VisibleObserver>,
) {
    todo!()
}
