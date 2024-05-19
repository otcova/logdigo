mod block;

use crate::graphics::renderers::BlockAtlasRenderer;
use crate::graphics::WgpuContext;
use std::hash::Hash;
use std::marker::PhantomData;

pub use block::*;

pub struct Handle<T: UIObject> {
    id: u64,
    _marker: PhantomData<T>,
}
impl<T: UIObject> Eq for Handle<T> {}
impl<T: UIObject> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: UIObject> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl<T: UIObject> Handle<T> {
    const fn from_id(id: u64) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

pub(crate) struct Objects {
    shared_renderers: SharedRenderers,
    collections: ObjectsCollections,
}

struct ObjectsCollections {
    blocks: BlockCollection,
}

struct SharedRenderers {
    block_atlas: BlockAtlasRenderer,
}

impl Objects {
    pub fn new(context: &WgpuContext) -> Self {
        Self {
            collections: ObjectsCollections {
                blocks: BlockCollection::new(context),
            },
            shared_renderers: SharedRenderers {
                block_atlas: BlockAtlasRenderer::new(context),
            },
        }
    }

    pub fn add<T: UIObject>(&mut self, object: T) -> Handle<T> {
        let object_renderer = T::mut_collection(&mut self.collections);
        object_renderer.add(object, &mut self.shared_renderers)
    }
}

pub(crate) trait UIObject: Sized {
    type Collection: UIObjectCollection<Object = Self>;
    fn mut_collection(objects: &mut ObjectsCollections) -> &mut Self::Collection;
}

pub(crate) trait UIObjectCollection {
    type Object: UIObject;
    fn new(context: &WgpuContext) -> Self;
    fn add(
        &mut self,
        object: Self::Object,
        shader_renderers: &mut SharedRenderers,
    ) -> Handle<Self::Object>;
}

macro_rules! impl_object {
    ($Object:ident, $Collection:ident, $collection_attrib:ident) => {
        impl UIObject for $Object {
            type Collection = $Collection;
            fn mut_collection(objects: &mut ObjectsCollections) -> &mut $Collection {
                &mut objects.$collection_attrib
            }
        }

        // impl Drop for Handle<$Object> { }
    };
}

use impl_object;
