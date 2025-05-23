//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A GKNoiseSource instance is a description of procedural noise in 3D space.  Noise sources generate values
    /// between -1.0 and 1.0, inclusive, for any position in continuous 3D space.
    /// Subclasses represent specific types of noise, each with their own parameters that affect the nature of the noise.
    /// Noise sources are the starting point for generating and using procedural noise.  The 3D noise values may be manipulated and
    /// combined with the GKNoise class.  Portions of this 3D noise can be extracted and utilized via the GKNoiseMap class.
    /// Extracted portions of noise are useful in both 2D and 3D games.  Applications include creating realistic textures,
    /// height maps for 2D and 3D game world terrain, tile maps for 2D games, and intentionally imperfect game object and
    /// camera movements in 2D and 3D games.
    /// This class is not intended to be instantiated.
    ///
    ///
    /// See: GKNoise
    ///
    /// See: GKNoiseMap
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gknoisesource?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKNoiseSource {}
);

impl GKNoiseSource {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl GKNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Coherent noise is smoothly-changing, semi-random noise.  A given input always produces the same output.
    /// A small change in input produces a small change in output.  A large change in input produces a random
    /// change in output. This class is not intended to be instantiated.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkcoherentnoisesource?language=objc)
    #[unsafe(super(GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKCoherentNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKCoherentNoiseSource {}
);

impl GKCoherentNoiseSource {
    extern_methods!(
        #[unsafe(method(frequency))]
        #[unsafe(method_family = none)]
        pub unsafe fn frequency(&self) -> c_double;

        /// Setter for [`frequency`][Self::frequency].
        #[unsafe(method(setFrequency:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrequency(&self, frequency: c_double);

        #[unsafe(method(octaveCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn octaveCount(&self) -> NSInteger;

        /// Setter for [`octaveCount`][Self::octaveCount].
        #[unsafe(method(setOctaveCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOctaveCount(&self, octave_count: NSInteger);

        #[unsafe(method(lacunarity))]
        #[unsafe(method_family = none)]
        pub unsafe fn lacunarity(&self) -> c_double;

        /// Setter for [`lacunarity`][Self::lacunarity].
        #[unsafe(method(setLacunarity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLacunarity(&self, lacunarity: c_double);

        #[unsafe(method(seed))]
        #[unsafe(method_family = none)]
        pub unsafe fn seed(&self) -> i32;

        /// Setter for [`seed`][Self::seed].
        #[unsafe(method(setSeed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSeed(&self, seed: i32);
    );
}

/// Methods declared on superclass `NSObject`.
impl GKCoherentNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Perlin noise is useful for creating natural-looking textures and realistic-looking terrain.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkperlinnoisesource?language=objc)
    #[unsafe(super(GKCoherentNoiseSource, GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKPerlinNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKPerlinNoiseSource {}
);

impl GKPerlinNoiseSource {
    extern_methods!(
        #[unsafe(method(persistence))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistence(&self) -> c_double;

        /// Setter for [`persistence`][Self::persistence].
        #[unsafe(method(setPersistence:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPersistence(&self, persistence: c_double);

        #[unsafe(method(perlinNoiseSourceWithFrequency:octaveCount:persistence:lacunarity:seed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn perlinNoiseSourceWithFrequency_octaveCount_persistence_lacunarity_seed(
            frequency: c_double,
            octave_count: NSInteger,
            persistence: c_double,
            lacunarity: c_double,
            seed: i32,
        ) -> Retained<Self>;

        #[unsafe(method(initWithFrequency:octaveCount:persistence:lacunarity:seed:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrequency_octaveCount_persistence_lacunarity_seed(
            this: Allocated<Self>,
            frequency: c_double,
            octave_count: NSInteger,
            persistence: c_double,
            lacunarity: c_double,
            seed: i32,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKPerlinNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Billow noise is similar to Perlin noise, with more rounded shapes and clearly-defined transitions beween values.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkbillownoisesource?language=objc)
    #[unsafe(super(GKCoherentNoiseSource, GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKBillowNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKBillowNoiseSource {}
);

impl GKBillowNoiseSource {
    extern_methods!(
        #[unsafe(method(persistence))]
        #[unsafe(method_family = none)]
        pub unsafe fn persistence(&self) -> c_double;

        /// Setter for [`persistence`][Self::persistence].
        #[unsafe(method(setPersistence:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPersistence(&self, persistence: c_double);

        #[unsafe(method(billowNoiseSourceWithFrequency:octaveCount:persistence:lacunarity:seed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn billowNoiseSourceWithFrequency_octaveCount_persistence_lacunarity_seed(
            frequency: c_double,
            octave_count: NSInteger,
            persistence: c_double,
            lacunarity: c_double,
            seed: i32,
        ) -> Retained<Self>;

        #[unsafe(method(initWithFrequency:octaveCount:persistence:lacunarity:seed:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrequency_octaveCount_persistence_lacunarity_seed(
            this: Allocated<Self>,
            frequency: c_double,
            octave_count: NSInteger,
            persistence: c_double,
            lacunarity: c_double,
            seed: i32,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKBillowNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Ridged noise is similar to Perlin noise, with sharply-defined, relatively thin peaks.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkridgednoisesource?language=objc)
    #[unsafe(super(GKCoherentNoiseSource, GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKRidgedNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKRidgedNoiseSource {}
);

impl GKRidgedNoiseSource {
    extern_methods!(
        #[unsafe(method(ridgedNoiseSourceWithFrequency:octaveCount:lacunarity:seed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn ridgedNoiseSourceWithFrequency_octaveCount_lacunarity_seed(
            frequency: c_double,
            octave_count: NSInteger,
            lacunarity: c_double,
            seed: i32,
        ) -> Retained<Self>;

        #[unsafe(method(initWithFrequency:octaveCount:lacunarity:seed:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrequency_octaveCount_lacunarity_seed(
            this: Allocated<Self>,
            frequency: c_double,
            octave_count: NSInteger,
            lacunarity: c_double,
            seed: i32,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKRidgedNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Voronoi noise partitions the space into angular, polygonal "cells", which are reminiscent
    /// of stained glass or crystal-like structures.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkvoronoinoisesource?language=objc)
    #[unsafe(super(GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKVoronoiNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKVoronoiNoiseSource {}
);

impl GKVoronoiNoiseSource {
    extern_methods!(
        #[unsafe(method(frequency))]
        #[unsafe(method_family = none)]
        pub unsafe fn frequency(&self) -> c_double;

        /// Setter for [`frequency`][Self::frequency].
        #[unsafe(method(setFrequency:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrequency(&self, frequency: c_double);

        #[unsafe(method(displacement))]
        #[unsafe(method_family = none)]
        pub unsafe fn displacement(&self) -> c_double;

        /// Setter for [`displacement`][Self::displacement].
        #[unsafe(method(setDisplacement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDisplacement(&self, displacement: c_double);

        #[unsafe(method(isDistanceEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDistanceEnabled(&self) -> bool;

        /// Setter for [`isDistanceEnabled`][Self::isDistanceEnabled].
        #[unsafe(method(setDistanceEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDistanceEnabled(&self, distance_enabled: bool);

        #[unsafe(method(seed))]
        #[unsafe(method_family = none)]
        pub unsafe fn seed(&self) -> i32;

        /// Setter for [`seed`][Self::seed].
        #[unsafe(method(setSeed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSeed(&self, seed: i32);

        #[unsafe(method(voronoiNoiseWithFrequency:displacement:distanceEnabled:seed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn voronoiNoiseWithFrequency_displacement_distanceEnabled_seed(
            frequency: c_double,
            displacement: c_double,
            distance_enabled: bool,
            seed: i32,
        ) -> Retained<Self>;

        #[unsafe(method(initWithFrequency:displacement:distanceEnabled:seed:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrequency_displacement_distanceEnabled_seed(
            this: Allocated<Self>,
            frequency: c_double,
            displacement: c_double,
            distance_enabled: bool,
            seed: i32,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKVoronoiNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Produces a single, constant value at all positions in the space.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkconstantnoisesource?language=objc)
    #[unsafe(super(GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKConstantNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKConstantNoiseSource {}
);

impl GKConstantNoiseSource {
    extern_methods!(
        #[unsafe(method(value))]
        #[unsafe(method_family = none)]
        pub unsafe fn value(&self) -> c_double;

        /// Setter for [`value`][Self::value].
        #[unsafe(method(setValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setValue(&self, value: c_double);

        #[unsafe(method(constantNoiseWithValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn constantNoiseWithValue(value: c_double) -> Retained<Self>;

        #[unsafe(method(initWithValue:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithValue(this: Allocated<Self>, value: c_double) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKConstantNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Produces 3D cylindrical noise with an infinite number of cylinders-within-cyliners of constantly-increasing radius.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkcylindersnoisesource?language=objc)
    #[unsafe(super(GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKCylindersNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKCylindersNoiseSource {}
);

impl GKCylindersNoiseSource {
    extern_methods!(
        #[unsafe(method(frequency))]
        #[unsafe(method_family = none)]
        pub unsafe fn frequency(&self) -> c_double;

        /// Setter for [`frequency`][Self::frequency].
        #[unsafe(method(setFrequency:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrequency(&self, frequency: c_double);

        #[unsafe(method(cylindersNoiseWithFrequency:))]
        #[unsafe(method_family = none)]
        pub unsafe fn cylindersNoiseWithFrequency(frequency: c_double) -> Retained<Self>;

        #[unsafe(method(initWithFrequency:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrequency(
            this: Allocated<Self>,
            frequency: c_double,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKCylindersNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Produces 3D spherical noise with an infinite number of spheres-within-spheres of constantly-increasing radius.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkspheresnoisesource?language=objc)
    #[unsafe(super(GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKSpheresNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKSpheresNoiseSource {}
);

impl GKSpheresNoiseSource {
    extern_methods!(
        #[unsafe(method(frequency))]
        #[unsafe(method_family = none)]
        pub unsafe fn frequency(&self) -> c_double;

        /// Setter for [`frequency`][Self::frequency].
        #[unsafe(method(setFrequency:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrequency(&self, frequency: c_double);

        #[unsafe(method(spheresNoiseWithFrequency:))]
        #[unsafe(method_family = none)]
        pub unsafe fn spheresNoiseWithFrequency(frequency: c_double) -> Retained<Self>;

        #[unsafe(method(initWithFrequency:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrequency(
            this: Allocated<Self>,
            frequency: c_double,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKSpheresNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Produces noise in a checkerboard pattern.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkcheckerboardnoisesource?language=objc)
    #[unsafe(super(GKNoiseSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKCheckerboardNoiseSource;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKCheckerboardNoiseSource {}
);

impl GKCheckerboardNoiseSource {
    extern_methods!(
        #[unsafe(method(squareSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn squareSize(&self) -> c_double;

        /// Setter for [`squareSize`][Self::squareSize].
        #[unsafe(method(setSquareSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSquareSize(&self, square_size: c_double);

        #[unsafe(method(checkerboardNoiseWithSquareSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn checkerboardNoiseWithSquareSize(square_size: c_double) -> Retained<Self>;

        #[unsafe(method(initWithSquareSize:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSquareSize(
            this: Allocated<Self>,
            square_size: c_double,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKCheckerboardNoiseSource {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
