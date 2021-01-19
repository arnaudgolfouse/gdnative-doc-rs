# Interface
Interface exported to Godot

All public method of this struct are usable in gdscript.

## func new() -> [Self](https://docs.godotengine.org/en/stable/classes/class_self.html)
________
Create a new empty [`DijkstraMap`](https://docs.godotengine.org/en/stable/classes/class_dijkstramap.html).

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
```
## func clear() -> [void](https://docs.godotengine.org/en/stable/classes/class_void.html)
________
Clear the underlying [`DijkstraMap`](https://docs.godotengine.org/en/stable/classes/class_dijkstramap.html).

### Example
```gdscript
dijkstra_map.clear()
```
## func duplicate_graph_from(source_instance: [Variant](https://docs.godotengine.org/en/stable/classes/class_variant.html)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
If `source_instance` is a [dijkstra map](https://docs.godotengine.org/en/stable/classes/class_interface.html), it is cloned into
`self`.

### Errors
This function returns `1` if `source_instance` is not a DijkstraMap.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
# fill dijkstra_map
var dijkstra_map_copy = DijkstraMap.new()
dijkstra_map_copy.duplicate_graph_from(dijkstra_map)
```
## func get_available_point_id() -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Returns the first positive available id.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
assert(dijkstra_map.get_available_point_id() == 2)
```
## func add_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), terrain_type: [int](https://docs.godotengine.org/en/stable/classes/class_int.html) (opt)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Add a new point with the given `terrain_type`.

If `terrain_type` is [`None`](https://docs.godotengine.org/en/stable/classes/class_none.html), `-1` is used.

### Errors
If a point with the given id already exists, the map is unchanged and
`1` is returned.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0) # terrain_type is -1
dijkstra_map.add_point(1, 0) # terrain_type is 0
```
## func set_terrain_for_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), terrain_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html) (opt)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Set the terrain type for `point_id`.

If `terrain_id` is [`None`](https://docs.godotengine.org/en/stable/classes/class_none.html), `-1` is used.

### Errors
If the given id does not exists in the map, `1` is returned.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 2)
dijkstra_map.set_terrain_for_point(0, 1)
assert(dijkstra_map.get_terrain_for_point(0) == 1)
dijkstra_map.set_terrain_for_point(0)
assert(dijkstra_map.get_terrain_for_point(0) == -1)
```
## func get_terrain_for_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Get the terrain type for the given point.

This function returns `-1` if no point with the given id exists in the
map.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 1)
dijkstra_map.add_point(1, -1)
assert(dijkstra_map.get_terrain_for_point(0) == 1)
assert(dijkstra_map.get_terrain_for_point(1) == -1)
# `2` is not in the map, so this returns `-1`
assert(dijkstra_map.get_terrain_for_point(2) == -1)
```
## func remove_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Removes a point from the map.

### Errors
Returns `1` if the point does not exists in the map.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert(dijkstra_map.remove_point(0) == 0)
assert(dijkstra_map.remove_point(0) == 1)
```
## func has_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [bool](https://docs.godotengine.org/en/stable/classes/class_bool.html)
________
Returns [`true`] if the map contains the given point.

## func disable_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Disable the given point for pathfinding.

### Errors
Returns `1` if the point does not exists in the map.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert(dijkstra_map.disable_point(0) == 0)
assert(dijkstra_map.disable_point(1) == 1)
```
## func enable_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Enable the given point for pathfinding.

### Errors
Returns `1` if the point does not exists in the map.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert(dijkstra_map.enable_point(0) == 0)
assert(dijkstra_map.enable_point(1) == 1)
```
## func is_point_disabled(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [bool](https://docs.godotengine.org/en/stable/classes/class_bool.html)
________
Returns [`true`] if the point exists and is disabled, otherwise returns
[`false`].

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.disable(0)
assert(dijkstra_map.is_point_disabled(0))
assert(!dijkstra_map.is_point_disabled(1))
assert(!dijkstra_map.is_point_disabled(2))
```
## func connect_points(source: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), target: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), weight: [float](https://docs.godotengine.org/en/stable/classes/class_float.html) (opt), bidirectional: [bool](https://docs.godotengine.org/en/stable/classes/class_bool.html) (opt)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Connects the two given points.

### Parameters
- `source` : source point of the connection.
- `target` : target point of the connection.
- `weight` : weight of the connection. Defaults to `1.0`.
- `bidirectional` : wether or not the reciprocal connection should be
made. Defaults to [`true`].
### Errors
Return `1` if one of the points does not exists in the map.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1, 2.0)
dijkstra_map.connect_points(1, 2, 1.0, false)
# produces the graph :
# 0 <---> 1 ----> 2
#    2.0     1.0
assert(dijkstra_map.connect_points(1, 3) == 1) # 3 does not exists in the map
```
## func remove_connection(source: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), target: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), bidirectional: [bool](https://docs.godotengine.org/en/stable/classes/class_bool.html) (opt)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Remove a connection between the two given points.

### Parameters
- `source` : source point of the connection.
- `target` : target point of the connection.
- `bidirectional` (default : [`true`]) : if [`true`], also removes
connection from target to source.
### Errors
Returns `1` if one of the points does not exist.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.connect_points(0, 1)
dijkstra_map.remove_connection(0, 1)
assert(dijkstra_map.remove_connection(0, 2) == 1) # 2 does not exists in the map
dijkstra_map.connect_points(0, 1)
# only removes connection from 0 to 1
dijkstra_map.remove_connection(0, 1, false)
assert(dijkstra_map.has_connection(1, 0))
```
## func has_connection(source: [int](https://docs.godotengine.org/en/stable/classes/class_int.html), target: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [bool](https://docs.godotengine.org/en/stable/classes/class_bool.html)
________
Returns [`true`] if there is a connection from `source` to `target`
(and they both exist).

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.connect_points(0, 1, 1.0, false)
assert(dijkstra_map.has_connection(0, 1))
assert(!dijkstra_map.has_connection(1, 0))
assert(!dijkstra_map.has_connection(0, 2))
```
## func get_direction_at_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Given a point, returns the id of the next point along the shortest path
toward the target.

### Errors
This function return `-1` if there is no path from the point to the target.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert(dijkstra_map.get_direction_at_point(0) == 0)
assert(dijkstra_map.get_direction_at_point(1) == 0)
assert(dijkstra_map.get_direction_at_point(2) == -1)
```
## func get_cost_at_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [float](https://docs.godotengine.org/en/stable/classes/class_float.html)
________
Returns the cost of the shortest path from this point to the target.

If there is no path, the cost is [`INFINITY`](https://docs.godotengine.org/en/stable/classes/class_infinity.html).

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert(dijkstra_map.get_cost_at_point(0) == 0.0)
assert(dijkstra_map.get_cost_at_point(1) == 1.0)
assert(dijkstra_map.get_cost_at_point(2) == INF)
```
## func recalculate(origin: [Variant](https://docs.godotengine.org/en/stable/classes/class_variant.html), optional_params: [Dictionary](https://docs.godotengine.org/en/stable/classes/class_dictionary.html) (opt)) -> [int](https://docs.godotengine.org/en/stable/classes/class_int.html)
________
Recalculates cost map and direction map information for each point,
overriding previous results.

This is the central function of the library, the one that actually uses
Dijkstra's algorithm.

### Parameters
-   `origin` : ID of the origin point, or array of IDs (preferably
[`Int32Array`](https://docs.godotengine.org/en/stable/classes/class_int32array.html)).


-   `optional_params: `[`Dictionary`](https://docs.godotengine.org/en/stable/classes/class_dictionary.html) : Specifies optional arguments. 

Valid arguments are :

  - `"input_is_destination" -> bool` (default : [`true`]) : 

Wether or not the `origin` points are seen as destination.
  - `"maximum_cost" -> float`
(default : [`INFINITY`](https://docs.godotengine.org/en/stable/classes/class_infinity.html)) : 

Specifies maximum cost. Once all shortest paths no longer than
maximum cost are found, algorithm terminates. All points with cost
bigger than this are treated as inaccessible.
  - `"initial_costs" -> float Array` (default : empty) : 

Specifies initial costs for given origins. Values are paired with
corresponding indices in the origin argument. Every unspecified
cost is defaulted to `0.0`. 

Can be used to weigh the origins with a preference.
  - `"terrain_weights" -> Dictionary` (default : empty) : 

Specifies weights of terrain types. Keys are terrain type IDs and
values are floats. Unspecified terrains will have
[`INFINITE`](https://docs.godotengine.org/en/stable/classes/class_infinity.html) weight. 

Note that `-1` correspond to the default terrain (which have a
weight of `1.0`), and will thus be ignored if it appears in the
keys.
  - `"termination_points" -> int OR int Array` (default : empty) : 

A set of points that stop the computation if they are reached by
the algorithm.
  Note that keys of incorrect types are ignored with a warning.


### Errors
`1` is returned if :

- One of the keys in `optional_params` is invalid.
- `origin` is neither an [`I64`](https://docs.godotengine.org/en/stable/classes/class_i64.html), a [`Int32Array`](https://docs.godotengine.org/en/stable/classes/class_int32array.html) or a [`VariantArray`](https://docs.godotengine.org/en/stable/classes/class_variantarray.html).
### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 0)
dijkstra_map.add_point(1, 1)
dijkstra_map.add_point(2, 0)
dijkstra_map.connect_points(0, 1)
dijkstra_map.connect_points(1, 2, 10.0)
var optional_params = {
    "terrain_weights": { 0: 1.0, 1: 2.0 },
    "termination_points": null,
    "input_is_destination": true,
    "maximum_cost": 2.0,
    "initial_costs": null,
}
dijkstra_map.recalculate(0, optional_params)
assert(dijkstra_map.get_direction_at_point(0) == 0)
assert(dijkstra_map.get_direction_at_point(1) == 0)
# 2 is too far from 0, so because we set "maximum_cost" to 2.0, it is innaccessible.
assert(dijkstra_map.get_direction_at_point(2) == -1)
```
## func get_direction_at_points(points: [PoolIntArray](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html)) -> [PoolIntArray](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html)
________
For each point in the given array, returns the id of the next point
along the shortest path toward the target.

If a point does not exists, or there is no path from it to the target,
the corresponding point will be `-1`.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert(Array(dijkstra_map.get_direction_at_points(PoolIntArray([0, 1, 2]))) == [0, 0, -1])
```
## func get_cost_at_points(points: [PoolIntArray](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html)) -> [PoolRealArray](https://docs.godotengine.org/en/stable/classes/class_poolrealarray.html)
________
For each point in the given array, returns the cost of the shortest
path from this point to the target.

If there is no path from a point to the target, the cost is
[`INFINITY`](https://docs.godotengine.org/en/stable/classes/class_infinity.html).

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert(Array(dijkstra_map.get_cost_at_points(PoolIntArray([0, 1, 2]))) == [0.0, 1.0, INF])
```
## func get_cost_map() -> [Dictionary](https://docs.godotengine.org/en/stable/classes/class_dictionary.html)
________
Returns the entire Dijktra map of costs in form of a Dictionary.

Keys are points' IDs, and values are costs. Inaccessible points are not
present in the dictionary.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
var cost_map = { 0: 0.0, 1: 1.0 }
var computed_cost_map = dijkstra_map.get_cost_map()
for id in computed_cost_map.keys():
    assert(computed_cost_map[id] == cost_map[id])
```
## func get_direction_map() -> [Dictionary](https://docs.godotengine.org/en/stable/classes/class_dictionary.html)
________
Returns the entire Dijkstra map of directions in form of a
[`Dictionary`](https://docs.godotengine.org/en/stable/classes/class_dictionary.html).

Keys are points' IDs, and values are the next point along the shortest
path.

#### Note
Unreacheable points are not present in the map.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
var direction_map = { 0: 0, 1: 0 }
var computed_direction_map = dijkstra_map.get_direction_map()
for id in computed_direction_map.keys():
    assert(computed_direction_map[id] == direction_map[id])
```
## func get_all_points_with_cost_between(min_cost: [float](https://docs.godotengine.org/en/stable/classes/class_float.html), max_cost: [float](https://docs.godotengine.org/en/stable/classes/class_float.html)) -> [PoolIntArray](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html)
________
Returns an array of all the points whose cost is between `min_cost` and
`max_cost`.

The array will be sorted by cost.

### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert(Array(dijkstra_map.get_all_points_with_cost_between(0.5, 1.5)) == [1])
```
## func get_shortest_path_from_point(point_id: [int](https://docs.godotengine.org/en/stable/classes/class_int.html)) -> [PoolIntArray](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html)
________
Returns an [array](https://docs.godotengine.org/en/stable/classes/class_int32array.html) of points describing the shortest path from a
starting point.

If the starting point is a target or is inaccessible, the [array](https://docs.godotengine.org/en/stable/classes/class_int32array.html) will
be empty.

#### Note
The starting point itself is not included.

## func add_square_grid(bounds: [Variant](https://docs.godotengine.org/en/stable/classes/class_variant.html), terrain_type: [int](https://docs.godotengine.org/en/stable/classes/class_int.html) (opt), orthogonal_cost: [float](https://docs.godotengine.org/en/stable/classes/class_float.html) (opt), diagonal_cost: [float](https://docs.godotengine.org/en/stable/classes/class_float.html) (opt)) -> [Dictionary](https://docs.godotengine.org/en/stable/classes/class_dictionary.html)
________
Adds a square grid of connected points.

### Parameters
- `bounds` : Dimensions of the grid. At the moment, only [`Rect2`](https://docs.godotengine.org/en/stable/classes/class_rect2.html) is
supported.
- `terrain_type` (default : `-1`) : Terrain to use for all points of
the grid.
- `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal
connections (up, down, right and left). 

If `orthogonal_cost` is [`INFINITY`](https://docs.godotengine.org/en/stable/classes/class_infinity.html) or [`Nan`](https://docs.godotengine.org/en/stable/classes/class_nan.html), orthogonal
connections are disabled.
- `diagonal_cost` (default : [`INFINITY`](https://docs.godotengine.org/en/stable/classes/class_infinity.html)) : specifies cost of diagonal
connections. 

If `diagonal_cost` is [`INFINITY`](https://docs.godotengine.org/en/stable/classes/class_infinity.html) or [`Nan`](https://docs.godotengine.org/en/stable/classes/class_nan.html), diagonal connections
are disabled.
### Returns
This function returns a Dictionary where keys are coordinates of points
([`Vector2`](https://docs.godotengine.org/en/stable/classes/class_vector2.html)) and values are their corresponding point IDs.

## func add_hexagonal_grid(bounds: [Variant](https://docs.godotengine.org/en/stable/classes/class_variant.html), terrain_type: [int](https://docs.godotengine.org/en/stable/classes/class_int.html) (opt), weight: [float](https://docs.godotengine.org/en/stable/classes/class_float.html) (opt)) -> [Dictionary](https://docs.godotengine.org/en/stable/classes/class_dictionary.html)
________
Adds a hexagonal grid of connected points.

### Parameters
- `bounds` : Dimensions of the grid.
- `terrain_type` (default : `-1`) : specifies terrain to be used.
- `weight` (default : `1.0`) : specifies cost of connections.
### Returns
This function returns a [`Dictionary`](https://docs.godotengine.org/en/stable/classes/class_dictionary.html) where keys are coordinates of
points ([`Vector2`](https://docs.godotengine.org/en/stable/classes/class_vector2.html)) and values are their corresponding point IDs.

### Note
Hexgrid is in the "pointy" orentation by default (see example below).

To switch to "flat" orientation, swap `width` and `height`, and switch
`x` and `y` coordinates of the keys in the return [`Dictionary`](https://docs.godotengine.org/en/stable/classes/class_dictionary.html).
([`Transform2D`](https://docs.godotengine.org/en/stable/classes/class_transform2d.html) may be convenient there)

### Example
This is what `add_hexagonal_grid(Rect2(1, 4, 2, 3), ...)` would produce:

```text
    / \     / \
  /     \ /     \
 |  1,4  |  2,4  |
  \     / \     / \
    \ /     \ /     \
     |  1,5  |  2,5  |
    / \     / \     /
  /     \ /     \ /
 |  1,6  |  2,6  |
  \     / \     /
    \ /     \ /
```
