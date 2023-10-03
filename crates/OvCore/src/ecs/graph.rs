use super::game_object::GameObject;
use std::ops::{Deref, DerefMut, Index as IndexOps, IndexMut};
use serde::{Serialize, Deserialize};
use thunderdome::{Arena, Index};

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pool: Arena<GameObject>,
    root: Index,
}

impl Deref for Graph {
    type Target = Arena<GameObject>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

impl IndexOps<Index> for Graph {
    type Output = GameObject;

    fn index(&self, index: Index) -> &Self::Output {
        &self.pool[index]
    }
}

impl IndexMut<Index> for Graph {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.pool[index]
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            pool: Arena::new(),
            root: Index::DANGLING,
        }
    }
}

impl Graph {
    fn addGameObject(&mut self, mut obj: GameObject) {
        self.pool.insert(obj);
    }

    fn getComponentByName(&self, name: &str) -> Option<&GameObject> {
        self.pool
            .iter()
            .find(|handle| handle.1.getName() == name)
            .map(|handle| handle.1)
    }
}

#[cfg(test)]
mod test {
    use crate::ecs::game_object::GameObject;

    use super::Graph;

    #[test]
    pub fn addObj() {
        let mut graph = Graph::default();
        let obj = GameObject::default();
        graph.addGameObject(obj);
        let obj = graph.getComponentByName("GameObject");
        println!("{:?}", graph);
        println!("{:?}", obj);
    }
}
