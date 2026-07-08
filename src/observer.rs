use bevy::{ecs::entity::EntityHashSet, prelude::*};

use crate::ClientVisible;

/// An entity that can see entities inside of containers.
/// This can be given to a a player's character or a long distanced camera.
/// Crate users must manually populate `containers` with entities containing [`VisibleContainer`].
#[derive(Component, Default, Debug)]
pub struct VisibleObserver {
    /// Entities containing [`VisibleContainer`] that this observer looks at.
    pub containers: EntityHashSet,
    /// Cached union of all entities visible through these containers.
    pub(crate) visible_entities: EntityHashSet,
}

impl VisibleObserver {
    /// Creates a new `VisibleObserver` with the given containers.
    pub fn new(containers: EntityHashSet) -> Self {
        Self {
            containers,
            ..default()
        }
    }
}

pub(super) fn update_visible_observers(
    changed_observers: Query<Entity, Changed<VisibleObserver>>,
    mut removed_observers: RemovedComponents<VisibleObserver>,
    observers: Query<&VisibleObserver>,
    mut clients: Query<Mut<ClientVisible>>,
) {
    let total_containers = changed_observers.count() + removed_observers.len();
    let mut changed_observers_set = EntityHashSet::with_capacity(total_containers);
    changed_observers_set.extend(changed_observers.iter());
    changed_observers_set.extend(removed_observers.read());

    for mut client in clients.iter_mut() {
        let client_changed = client.is_changed();
        let needs_update = client_changed || !client.observers.is_disjoint(&changed_observers_set);

        if !needs_update {
            continue;
        }

        let mut new_visible = EntityHashSet::new();
        for &observer_entity in &client.observers {
            let Ok(observer) = observers.get(observer_entity) else {
                continue;
            };
            new_visible.extend(observer.visible_entities.iter());
        }
        if client.visible_entities != new_visible {
            client.visible_entities = new_visible;
        }
    }
}
