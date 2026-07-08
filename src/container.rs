use bevy::{ecs::entity::EntityHashSet, prelude::*};

use crate::observer::VisibleObserver;

/// Which [`VisibleContainer`] this entity is visible in.
/// An entity can only be visible inside one container.
/// It is the user's responsibility to add it to entities.
#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = VisibleContainer)]
pub struct VisibleIn(pub Entity);

/// Entities can be added and removed from this container using the [`VisibleIn`] relationship.
#[derive(Component, Reflect, Default, Debug)]
#[relationship_target(relationship = VisibleIn)]
pub struct VisibleContainer {
    #[relationship]
    contained: EntityHashSet,
}

pub(super) fn update_visible_containers(
    changed_containers: Query<Entity, (With<VisibleContainer>, Changed<VisibleContainer>)>,
    mut removed_containers: RemovedComponents<VisibleContainer>,
    containers: Query<&VisibleContainer>,
    mut observers: Query<Mut<VisibleObserver>>,
) {
    let total_containers = changed_containers.count() + removed_containers.len();
    let mut changed_containers_set = EntityHashSet::with_capacity(total_containers);
    changed_containers_set.extend(changed_containers.iter());
    changed_containers_set.extend(removed_containers.read());

    for mut observer in observers.iter_mut() {
        let observer_changed = observer.is_changed();
        let needs_update =
            observer_changed || !observer.containers.is_disjoint(&changed_containers_set);

        if !needs_update {
            continue;
        }

        let mut new_visible = EntityHashSet::new();
        for &container_entity in &observer.containers {
            let Ok(container) = containers.get(container_entity) else {
                continue;
            };

            new_visible.extend(container.contained.iter());
        }

        if observer.visible_entities != new_visible {
            observer.visible_entities = new_visible;
        }
    }
}
