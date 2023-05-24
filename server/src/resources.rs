use bevy::{prelude::*, utils::HashMap};
use naia_bevy_server::{RoomKey, UserKey};

#[derive(Resource)]
pub struct MainRoomKey(pub RoomKey);

#[derive(Resource)]
pub struct UserAvatarMapping {
    user_to_avatar: HashMap<UserKey, Entity>,
    avatar_to_user: HashMap<Entity, UserKey>,
}

impl UserAvatarMapping {
    pub fn new() -> Self {
        Self {
            user_to_avatar: HashMap::new(),
            avatar_to_user: HashMap::new(),
        }
    }

    pub fn insert(&mut self, user: UserKey, avatar_entity: Entity) {
        self.user_to_avatar.insert(user, avatar_entity);
        self.avatar_to_user.insert(avatar_entity, user);
    }

    pub fn get_by_user(&self, user: &UserKey) -> Option<&Entity> {
        self.user_to_avatar.get(user)
    }

    pub fn get_by_entity(&self, avatar: &Entity) -> Option<&UserKey> {
        self.avatar_to_user.get(avatar)
    }

    pub fn remove_by_user(&mut self, user: &UserKey) {
        if let Some(entity) = self.user_to_avatar.remove(user) {
            self.avatar_to_user.remove(&entity);
        }
    }

    pub fn remove_by_entity(&mut self, avatar: &Entity) {
        if let Some(key) = self.avatar_to_user.remove(avatar) {
            self.user_to_avatar.remove(&key);
        }
    }
}

#[derive(Resource)]
pub struct UserNameMapping {
    user_to_name: HashMap<UserKey, String>,
    name_to_user: HashMap<String, UserKey>,
}

impl UserNameMapping {
    pub fn new() -> Self {
        Self {
            user_to_name: HashMap::new(),
            name_to_user: HashMap::new(),
        }
    }

    pub fn insert(&mut self, user: UserKey, name: String) {
        self.user_to_name.insert(user, name.clone());
        self.name_to_user.insert(name, user);
    }

    pub fn get_by_user(&self, user: &UserKey) -> Option<&String> {
        self.user_to_name.get(user)
    }

    pub fn get_by_name(&self, name: &String) -> Option<&UserKey> {
        self.name_to_user.get(name)
    }

    pub fn remove_by_user(&mut self, user: &UserKey) {
        if let Some(name) = self.user_to_name.remove(user) {
            self.name_to_user.remove(&name);
        }
    }

    pub fn remove_by_name(&mut self, name: &String) {
        if let Some(key) = self.name_to_user.remove(name) {
            self.user_to_name.remove(&key);
        }
    }
}
