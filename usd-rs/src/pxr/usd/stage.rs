//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! This contains everything you need for working with a usd stage, the main
//! point of entry into the usd library.

use cpp::cpp;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/stage.h"
    #pragma GCC diagnostic pop

}}

//------------------------------------------------------------------------------
type TfRefBase = core::ffi::c_void;

// This is really just a typedef.
struct TfRefPtr {
    ref_base: *mut TfRefBase,
}

impl std::clone::Clone for TfRefPtr {
    fn clone(&self) -> Self {
        let new_ref_base = unsafe { self.ref_base.clone() };
        Self {
            ref_base: new_ref_base,
        }
    }
}

//------------------------------------------------------------------------------
#[repr(C)]
/// Specifies the initial set of prims to load when opening a UsdStage
pub enum InitialLoadSet {
    /// Load all loadable prims (default).
    LoadAll = 0,
    /// Load no loadable prims.
    LoadNone = 1,
}

impl Default for InitialLoadSet {
    fn default() -> Self {
        InitialLoadSet::LoadAll
    }
}

//------------------------------------------------------------------------------
/// The outermost container for scene description, which owns and presents
/// composed prims as a scenegraph, following the composition recipe recursively
/// described in its associated "root layer".
///
/// USD derives its persistent-storage scalability by combining and reusing
/// simple compositions into richer aggregates using referencing and layering
/// with sparse overrides. Ultimately, every composition (i.e. "scene") is
/// identifiable by its root layer, i.e. the .usd file, and a scene is
/// instantiated in an application on a UsdStage that presents a composed view
/// of the scene's root layer. Each simple composition referenced into a larger
/// composition could be presented on its own UsdStage, at the same (or not)
/// time that it is participating in the larger composition on its own UsdStage;
/// all of the underlying layers will be shared by the two stages, while each
/// maintains its own scenegraph of composed prims.
///
/// A UsdStage has sole ownership over the UsdPrim 's with which it is
/// populated, and retains shared ownership (with other stages and direct
/// clients of SdfLayer's, via the Sdf_LayerRegistry that underlies all SdfLayer
/// creation methods) of layers. It provides roughly five categories of API that
/// address different aspects of scene management:

/// - Stage lifetime management methods for constructing and initially populating
/// a UsdStage from an existing layer file, or one that will be created as a
/// result, in memory or on the filesystem.
/// - Load/unload working set management methods that allow you to specify which
/// payloads should be included and excluded from the stage's composition.
/// - Variant management methods to manage policy for which variant to use when
/// composing prims that provide a named variant set, but do not specify a
/// selection.
/// - Prim access, creation, and mutation methods that allow you to find, create,
/// or remove a prim identified by a path on the stage. This group also provides
/// methods for efficiently traversing the prims on the stage.
/// - Layers and EditTargets methods provide access to the layers in the stage's
/// root LayerStack (i.e. the root layer and all of its recursive sublayers),
/// and the ability to set a UsdEditTarget into which all subsequent mutations
/// to objects associated with the stage (e.g. prims, properties, etc) will go.
/// - Serialization methods for "flattening" a composition (to varying degrees),
/// and exporting a completely flattened view of the stage to a string or file.
/// These methods can be very useful for targetted asset optimization and
/// debugging, though care should be exercized with large scenes, as flattening
/// defeats some of the benefits of referenced scene description, and may
/// produce very large results, especially in file formats that do not support
/// data de-duplication, like the usda ASCII format!
pub struct Stage {
    this: TfRefPtr,
}

type void = core::ffi::c_void;

impl Stage {
    pub fn create_in_memory(load: InitialLoadSet) -> Self {
        let this = unsafe {
            cpp!([load as "pxr::UsdStage::InitialLoadSet"] -> TfRefPtr as "pxr::UsdStageRefPtr" {
                return pxr::UsdStage::CreateInMemory(load);
            })
        };

        Self { this }
    }

    pub fn export(&self) {
        let data = self.this.clone();
        unsafe {
            cpp!([data as "pxr::UsdStageRefPtr"] {
                data->Export("test_out.usda");
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let stage = Stage::create_in_memory(InitialLoadSet::default());
        stage.export();
    }
}
