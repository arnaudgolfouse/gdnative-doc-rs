//! Implementation of [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm) in Rust.
//!
//! Examples describe how to use the code in gdscript.

#![allow(dead_code, unused_variables)]

use gdnative::prelude::*;

/// Integer representing success in gdscript
const GODOT_SUCCESS: i64 = 0;
/// Integer representing failure in gdscript
const GODOT_ERROR: i64 = 1;

/// Interface exported to Godot
///
/// The map must first be filled by using e.g. `add_point`, `connect_points`,
/// `add_square_grid`...
///
/// And then you must call `recalculate` on it.
#[derive(NativeClass)]
#[inherit(Reference)]
pub struct Interface {
    /// Dummy property for demonstration purposes
    #[property]
    property: String,
}

/// Change a Rust's [`Result`] to an integer (which is how errors are reported
/// to Godot).
///
/// [`Ok`] becomes `0`, and [`Err`] becomes `1`.
fn result_to_int(res: Result<(), ()>) -> i64 {
    match res {
        Ok(()) => GODOT_SUCCESS,
        Err(()) => GODOT_ERROR,
    }
}

/// Try to convert the given [`Variant`] into a rectangle of `usize`.
///
/// Only works if `bounds` is a [`Rect2D`].
///
/// # Return
///
/// `(x_offset, y_offset, width, height)`
fn variant_to_width_and_height(bounds: Variant) -> Option<(usize, usize, usize, usize)> {
    bounds.try_to_rect2().map(|rect| {
        (
            rect.origin.x as usize,
            rect.origin.y as usize,
            rect.size.width as usize,
            rect.size.height as usize,
        )
    })
}

#[methods]
impl Interface {
    /// Create a new empty `DijkstraMap`.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// ```
    pub fn new(_owner: &Reference) -> Self {
        Interface {
            property: String::new(),
        }
    }

    #[export]
    /// Clear the underlying `DijkstraMap`.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.clear()
    /// ```
    pub fn clear(&mut self, _owner: &Reference) {}

    #[export]
    /// If `source_instance` is a `dijkstra map`, it is cloned into
    /// `self`.
    ///
    /// # Errors
    ///
    /// This function returns [`FAILED`] if `source_instance` is not a DijkstraMap.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// # fill dijkstra_map
    /// var dijkstra_map_copy = DijkstraMap.new()
    /// dijkstra_map_copy.duplicate_graph_from(dijkstra_map)
    /// ```
    pub fn duplicate_graph_from(&mut self, _owner: &Reference, source_instance: Variant) -> i64 {
        GODOT_SUCCESS
    }

    #[export]
    /// Returns the first positive available id.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// assert(dijkstra_map.get_available_point_id() == 2)
    /// ```
    pub fn get_available_point_id(&mut self, _owner: &Reference) -> i32 {
        0
    }

    #[export]
    /// Add a new point with the given `terrain_type`.
    ///
    /// If `terrain_type` not specified, `-1` is used.
    ///
    /// # Errors
    ///
    /// If a point with the given id already exists, the map is unchanged and
    /// [`FAILED`] is returned.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0) # terrain_type is -1
    /// dijkstra_map.add_point(1, 0) # terrain_type is 0
    /// ```
    pub fn add_point(
        &mut self,
        _owner: &Reference,
        point_id: i32,
        #[opt] terrain_type: Option<i32>,
    ) -> i64 {
        result_to_int(Ok(()))
    }

    #[export]
    /// Set the terrain type for `point_id`.
    ///
    /// If `terrain_id` is not specified, `-1` is used.
    ///
    /// # Errors
    ///
    /// If the given id does not exists in the map, [`FAILED`] is returned.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0, 2)
    /// dijkstra_map.set_terrain_for_point(0, 1)
    /// assert(dijkstra_map.get_terrain_for_point(0) == 1)
    /// dijkstra_map.set_terrain_for_point(0)
    /// assert(dijkstra_map.get_terrain_for_point(0) == -1)
    /// ```
    pub fn set_terrain_for_point(
        &mut self,
        _owner: &Reference,
        point_id: i32,
        terrain_id: Option<i32>,
    ) -> i64 {
        result_to_int(Err(()))
    }

    #[export]
    /// Get the terrain type for the given point.
    ///
    /// This function returns `-1` if no point with the given id exists in the
    /// map.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0, 1)
    /// dijkstra_map.add_point(1, -1)
    /// assert(dijkstra_map.get_terrain_for_point(0) == 1)
    /// assert(dijkstra_map.get_terrain_for_point(1) == -1)
    /// # `2` is not in the map, so this returns `-1`
    /// assert(dijkstra_map.get_terrain_for_point(2) == -1)
    /// ```
    pub fn get_terrain_for_point(&mut self, _owner: &Reference, point_id: i32) -> i32 {
        0
    }

    #[export]
    /// Removes a point from the map.
    ///
    /// # Errors
    ///
    /// Returns [`FAILED`] if the point does not exists in the map.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// assert(dijkstra_map.remove_point(0) == 0)
    /// assert(dijkstra_map.remove_point(0) == 1)
    /// ```
    pub fn remove_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        result_to_int(Ok(()))
    }

    #[export]
    /// Returns [`true`] if the map contains the given point.
    pub fn has_point(&mut self, _owner: &Reference, point_id: i32) -> bool {
        false
    }

    #[export]
    /// Disable the given point for pathfinding.
    ///
    /// # Errors
    ///
    /// Returns [`FAILED`] if the point does not exists in the map.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// assert(dijkstra_map.disable_point(0) == 0)
    /// assert(dijkstra_map.disable_point(1) == 1)
    /// ```
    pub fn disable_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        result_to_int(Ok(()))
    }

    #[export]
    /// Enable the given point for pathfinding.
    ///
    /// # Errors
    ///
    /// Returns [`FAILED`] if the point does not exists in the map.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// assert(dijkstra_map.enable_point(0) == 0)
    /// assert(dijkstra_map.enable_point(1) == 1)
    /// ```
    pub fn enable_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        result_to_int(Ok(()))
    }

    #[export]
    /// Returns [`true`] if the point exists and is disabled, otherwise returns
    /// [`false`].
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.disable_point(0)
    /// assert(dijkstra_map.is_point_disabled(0))
    /// assert(!dijkstra_map.is_point_disabled(1))
    /// assert(!dijkstra_map.is_point_disabled(2))
    /// ```
    pub fn is_point_disabled(&mut self, _owner: &Reference, point_id: i32) -> bool {
        false
    }

    #[export]
    /// Connects the two given points.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `weight` : weight of the connection. Defaults to `1.0`.
    /// - `bidirectional` : wether or not the reciprocal connection should be
    /// made. Defaults to [`true`].
    ///
    /// # Errors
    ///
    /// Return [`FAILED`] if one of the points does not exists in the map.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1, 2.0)
    /// dijkstra_map.connect_points(1, 2, 1.0, false)
    /// # produces the graph :
    /// # 0 <---> 1 ----> 2
    /// #    2.0     1.0
    /// assert(dijkstra_map.connect_points(1, 3) == 1) # 3 does not exists in the map
    /// ```
    pub fn connect_points(
        &mut self,
        _owner: &Reference,
        source: i32,
        target: i32,
        #[opt] weight: Option<f32>,
        #[opt] bidirectional: Option<bool>,
    ) -> i64 {
        result_to_int(Ok(()))
    }

    #[export]
    /// Remove a connection between the two given points.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `bidirectional` (default : [`true`]) : if [`true`], also removes
    /// connection from target to source.
    ///
    /// # Errors
    ///
    /// Returns [`FAILED`] if one of the points does not exist.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.remove_connection(0, 1)
    /// assert(dijkstra_map.remove_connection(0, 2) == 1) # 2 does not exists in the map
    /// dijkstra_map.connect_points(0, 1)
    /// # only removes connection from 0 to 1
    /// dijkstra_map.remove_connection(0, 1, false)
    /// assert(dijkstra_map.has_connection(1, 0))
    /// ```
    pub fn remove_connection(
        &mut self,
        _owner: &Reference,
        source: i32,
        target: i32,
        #[opt] bidirectional: Option<bool>,
    ) -> i64 {
        result_to_int(Ok(()))
    }

    #[export]
    /// Returns [`true`] if there is a connection from `source` to `target`
    /// (and they both exist).
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.connect_points(0, 1, 1.0, false)
    /// assert(dijkstra_map.has_connection(0, 1))
    /// assert(!dijkstra_map.has_connection(1, 0))
    /// assert(!dijkstra_map.has_connection(0, 2))
    /// ```
    pub fn has_connection(&mut self, _owner: &Reference, source: i32, target: i32) -> bool {
        false
    }

    #[export]
    /// Given a point, returns the id of the next point along the shortest path
    /// toward the target.
    ///
    /// # Errors
    ///
    /// This function return `-1` if there is no path from the point to the target.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// assert(dijkstra_map.get_direction_at_point(0) == 0)
    /// assert(dijkstra_map.get_direction_at_point(1) == 0)
    /// assert(dijkstra_map.get_direction_at_point(2) == -1)
    /// ```
    pub fn get_direction_at_point(&mut self, _owner: &Reference, point_id: i32) -> i32 {
        -1
    }

    #[export]
    /// Returns the cost of the shortest path from this point to the target.
    ///
    /// If there is no path, the cost is [`INF`].
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// assert(dijkstra_map.get_cost_at_point(0) == 0.0)
    /// assert(dijkstra_map.get_cost_at_point(1) == 1.0)
    /// assert(dijkstra_map.get_cost_at_point(2) == INF)
    /// ```
    pub fn get_cost_at_point(&mut self, _owner: &Reference, point_id: i32) -> f32 {
        1.0
    }

    #[export]
    /// Recalculates cost map and direction map information for each point,
    /// overriding previous results.
    ///
    /// This is the central function of the library, the one that actually uses
    /// Dijkstra's algorithm.
    ///
    /// # Parameters
    ///
    /// - `origin` : ID of the origin point, or array of IDs (preferably
    /// [`Int32Array`]).
    /// - `optional_params: `[`Dictionary`] : Specifies optional arguments. \
    /// Valid arguments are :
    ///   - `"input_is_destination": `[`bool`] (default : [`true`]) : \
    ///     Wether or not the `origin` points are seen as destination.
    ///   - `"maximum_cost": `[`float`]
    ///         (default : [`INF`]) : \
    ///     Specifies maximum cost. Once all shortest paths no longer than
    ///     maximum cost are found, algorithm terminates. All points with cost
    ///     bigger than this are treated as inaccessible.
    ///   - `"initial_costs": `[`float`] [`Array`] (default : empty) : \
    ///     Specifies initial costs for given origins. Values are paired with
    ///     corresponding indices in the origin argument. Every unspecified
    ///     cost is defaulted to `0.0`. \
    ///     Can be used to weigh the origins with a preference.
    ///   - `"terrain_weights": `[`Dictionary`] (default : empty) : \
    ///     Specifies weights of terrain types. Keys are terrain type IDs and
    ///     values are floats. Unspecified terrains will have
    ///     [infinite](INF) weight. \
    ///     Note that `-1` correspond to the default terrain (which have a
    ///     weight of `1.0`), and will thus be ignored if it appears in the
    ///     keys.
    ///   - `"termination_points": `[`int`] OR [`int`] [`Array`] (default : empty) : \
    ///     A set of points that stop the computation if they are reached by
    ///     the algorithm.
    ///
    ///   Note that keys of incorrect types are ignored with a warning.
    ///
    /// # Errors
    ///
    /// [`FAILED`] is returned if :
    /// - One of the keys in `optional_params` is invalid.
    /// - `origin` is neither an [`int`], a [`PoolIntArray`] or a [`Array`].
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0, 0)
    /// dijkstra_map.add_point(1, 1)
    /// dijkstra_map.add_point(2, 0)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.connect_points(1, 2, 10.0)
    /// var optional_params = {
    ///     "terrain_weights": { 0: 1.0, 1: 2.0 },
    ///     "termination_points": null,
    ///     "input_is_destination": true,
    ///     "maximum_cost": 2.0,
    ///     "initial_costs": null,
    /// }
    /// dijkstra_map.recalculate(0, optional_params)
    /// assert(dijkstra_map.get_direction_at_point(0) == 0)
    /// assert(dijkstra_map.get_direction_at_point(1) == 0)
    /// # 2 is too far from 0, so because we set "maximum_cost" to 2.0, it is innaccessible.
    /// assert(dijkstra_map.get_direction_at_point(2) == -1)
    /// ```
    pub fn recalculate(
        &mut self,
        _owner: &Reference,
        origin: gdnative::core_types::Variant,
        #[opt] optional_params: Option<Dictionary>,
    ) -> i64 {
        GODOT_SUCCESS
    }

    #[export]
    /// For each point in the given array, returns the id of the next point
    /// along the shortest path toward the target.
    ///
    /// If a point does not exists, or there is no path from it to the target,
    /// the corresponding point will be `-1`.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// assert(Array(dijkstra_map.get_direction_at_points(PoolIntArray([0, 1, 2]))) == [0, 0, -1])
    /// ```
    pub fn get_direction_at_points(
        &mut self,
        _owner: &Reference,
        points: Int32Array,
    ) -> Int32Array {
        Int32Array::default()
    }

    #[export]
    /// For each point in the given array, returns the cost of the shortest
    /// path from this point to the target.
    ///
    /// If there is no path from a point to the target, the cost is
    /// [`INF`].
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// assert(Array(dijkstra_map.get_cost_at_points(PoolIntArray([0, 1, 2]))) == [0.0, 1.0, INF])
    /// ```
    pub fn get_cost_at_points(
        &mut self,
        _owner: &Reference,
        points: gdnative::core_types::Int32Array,
    ) -> gdnative::core_types::Float32Array {
        Float32Array::default()
    }

    #[export]
    /// Returns the entire Dijktra map of costs in form of a Dictionary.
    ///
    /// Keys are points' IDs, and values are costs. Inaccessible points are not
    /// present in the dictionary.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// var cost_map = { 0: 0.0, 1: 1.0 }
    /// var computed_cost_map = dijkstra_map.get_cost_map()
    /// for id in computed_cost_map.keys():
    ///     assert(computed_cost_map[id] == cost_map[id])
    /// ```
    pub fn get_cost_map(&mut self, _owner: &Reference) -> Dictionary {
        Dictionary::new().into_shared()
    }

    #[export]
    /// Returns the entire Dijkstra map of directions in form of a
    /// [`Dictionary`].
    ///
    /// Keys are points' IDs, and values are the next point along the shortest
    /// path.
    ///
    /// ## Note
    ///
    /// Unreacheable points are not present in the map.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// var direction_map = { 0: 0, 1: 0 }
    /// var computed_direction_map = dijkstra_map.get_direction_map()
    /// for id in computed_direction_map.keys():
    ///     assert(computed_direction_map[id] == direction_map[id])
    /// ```
    pub fn get_direction_map(&mut self, _owner: &Reference) -> Dictionary {
        Dictionary::new().into_shared()
    }

    #[export]
    /// Returns an array of all the points whose cost is between `min_cost` and
    /// `max_cost`.
    ///
    /// The array will be sorted by cost.
    ///
    /// # Example
    /// ```gdscript
    /// var dijkstra_map = DijkstraMap.new()
    /// dijkstra_map.add_point(0)
    /// dijkstra_map.add_point(1)
    /// dijkstra_map.add_point(2)
    /// dijkstra_map.connect_points(0, 1)
    /// dijkstra_map.recalculate(0)
    /// assert(Array(dijkstra_map.get_all_points_with_cost_between(0.5, 1.5)) == [1])
    /// ```
    pub fn get_all_points_with_cost_between(
        &mut self,
        _owner: &Reference,
        min_cost: f32,
        max_cost: f32,
    ) -> gdnative::core_types::Int32Array {
        Int32Array::default()
    }

    #[export]
    /// Returns an [array] of points describing the shortest path from a
    /// starting point.
    ///
    /// If the starting point is a target or is inaccessible, the [array] will
    /// be empty.
    ///
    /// ## Note
    ///
    /// The starting point itself is not included.
    ///
    /// [array]: gdnative::core_types::Int32Array
    pub fn get_shortest_path_from_point(
        &mut self,
        _owner: &Reference,
        point_id: i32,
    ) -> gdnative::core_types::Int32Array {
        Int32Array::default()
    }

    #[export]
    /// Adds a square grid of connected points.
    ///
    /// # Parameters
    ///
    /// - `bounds` : Dimensions of the grid. At the moment, only [`Rect2`] is
    ///   supported.
    /// - `terrain_type` (default : `-1`) : Terrain to use for all points of
    ///   the grid.
    /// - `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal
    ///   connections (up, down, right and left). \
    ///   If `orthogonal_cost` is [`INF`] or [`NAN`], orthogonal
    ///   connections are disabled.
    /// - `diagonal_cost` (default : [`INF`]) : specifies cost of diagonal
    ///   connections. \
    ///   If `diagonal_cost` is [`INF`] or [`NAN`], diagonal connections
    ///   are disabled.
    ///
    /// # Returns
    ///
    /// This function returns a Dictionary where keys are coordinates of points
    /// ([`Vector2`]) and values are their corresponding point IDs.
    pub fn add_square_grid(
        &mut self,
        _owner: &Reference,
        bounds: Variant,
        #[opt] terrain_type: Option<i32>,
        #[opt] orthogonal_cost: Option<f32>,
        #[opt] diagonal_cost: Option<f32>,
    ) -> Dictionary {
        Dictionary::new().into_shared()
    }

    #[export]
    /// Adds a hexagonal grid of connected points.
    ///
    /// # Parameters
    ///
    /// - `bounds` : Dimensions of the grid.
    /// - `terrain_type` (default : `-1`) : specifies terrain to be used.
    /// - `weight` (default : `1.0`) : specifies cost of connections.
    ///
    /// # Returns
    ///
    /// This function returns a [`Dictionary`] where keys are coordinates of
    /// points ([`Vector2`]) and values are their corresponding point IDs.
    ///
    /// # Note
    ///
    /// Hexgrid is in the "pointy" orentation by default (see example below).
    ///
    /// To switch to "flat" orientation, swap `width` and `height`, and switch
    /// `x` and `y` coordinates of the keys in the return [`Dictionary`].
    /// ([`Transform2D`] may be convenient there)
    ///
    /// # Example
    ///
    /// This is what `add_hexagonal_grid(Rect2(1, 4, 2, 3), ...)` would produce:
    ///
    ///```text
    ///    / \     / \
    ///  /     \ /     \
    /// |  1,4  |  2,4  |
    ///  \     / \     / \
    ///    \ /     \ /     \
    ///     |  1,5  |  2,5  |
    ///    / \     / \     /
    ///  /     \ /     \ /
    /// |  1,6  |  2,6  |
    ///  \     / \     /
    ///    \ /     \ /
    ///```
    pub fn add_hexagonal_grid(
        &mut self,
        _owner: &Reference,
        bounds: Variant,
        #[opt] terrain_type: Option<i32>,
        #[opt] weight: Option<f32>,
    ) -> Dictionary {
        Dictionary::new().into_shared()
    }
}

fn init(handle: gdnative::prelude::InitHandle) {
    handle.add_class::<Interface>();
}
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
