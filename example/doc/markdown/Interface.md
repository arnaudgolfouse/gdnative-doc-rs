# Interface
Interface exported to Godot

The map must first be filled by using e.g. `add_point`, `connect_points`,
`add_square_grid`...

And then you must call `recalculate` on it.

## func new() -> Self
________
Create a new empty `DijkstraMap`.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
```
## func clear() -> void
________
Clear the underlying `DijkstraMap`.

### Example

```gdscript
dijkstra_map.clear()
```
## func duplicate_graph_from(source_instance: [Variant]) -> [int]
________
If `source_instance` is a `dijkstra map`, it is cloned into
`self`.

### Errors
This function returns [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) if `source_instance` is not a DijkstraMap.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
# fill dijkstra_map
var dijkstra_map_copy = DijkstraMap.new()
dijkstra_map_copy.duplicate_graph_from(dijkstra_map)
```
## func get_available_point_id() -> [int]
________
Returns the first positive available id.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
assert(dijkstra_map.get_available_point_id() == 2)
```
## func add_point(point_id: [int], terrain_type: [int] (opt)) -> [int]
________
Add a new point with the given `terrain_type`.

If `terrain_type` not specified, `-1` is used.

### Errors
If a point with the given id already exists, the map is unchanged and
[`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) is returned.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0) # terrain_type is -1
dijkstra_map.add_point(1, 0) # terrain_type is 0
```
## func set_terrain_for_point(point_id: [int], terrain_id: [int] (opt)) -> [int]
________
Set the terrain type for `point_id`.

If `terrain_id` is not specified, `-1` is used.

### Errors
If the given id does not exists in the map, [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) is returned.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 2)
dijkstra_map.set_terrain_for_point(0, 1)
assert(dijkstra_map.get_terrain_for_point(0) == 1)
dijkstra_map.set_terrain_for_point(0)
assert(dijkstra_map.get_terrain_for_point(0) == -1)
```
## func get_terrain_for_point(point_id: [int]) -> [int]
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
## func remove_point(point_id: [int]) -> [int]
________
Removes a point from the map.

### Errors
Returns [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) if the point does not exists in the map.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert(dijkstra_map.remove_point(0) == 0)
assert(dijkstra_map.remove_point(0) == 1)
```
## func has_point(point_id: [int]) -> [bool]
________
Returns [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html) if the map contains the given point.

## func disable_point(point_id: [int]) -> [int]
________
Disable the given point for pathfinding.

### Errors
Returns [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) if the point does not exists in the map.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert(dijkstra_map.disable_point(0) == 0)
assert(dijkstra_map.disable_point(1) == 1)
```
## func enable_point(point_id: [int]) -> [int]
________
Enable the given point for pathfinding.

### Errors
Returns [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) if the point does not exists in the map.

### Example

```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert(dijkstra_map.enable_point(0) == 0)
assert(dijkstra_map.enable_point(1) == 1)
```
## func is_point_disabled(point_id: [int]) -> [bool]
________
Returns [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html) if the point exists and is disabled, otherwise returns
[`false`](https://docs.godotengine.org/en/stable/classes/class_bool.html).

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
## func connect_points(source: [int], target: [int], weight: [float] (opt), bidirectional: [bool] (opt)) -> [int]
________
Connects the two given points.

### Parameters
- `source` : source point of the connection.
- `target` : target point of the connection.
- `weight` : weight of the connection. Defaults to `1.0`.
- `bidirectional` : wether or not the reciprocal connection should be
made. Defaults to [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html).
### Errors
Return [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) if one of the points does not exists in the map.

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
## func remove_connection(source: [int], target: [int], bidirectional: [bool] (opt)) -> [int]
________
Remove a connection between the two given points.

### Parameters
- `source` : source point of the connection.
- `target` : target point of the connection.
- `bidirectional` (default : [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html)) : if [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html), also removes
connection from target to source.
### Errors
Returns [`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) if one of the points does not exist.

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
## func has_connection(source: [int], target: [int]) -> [bool]
________
Returns [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html) if there is a connection from `source` to `target`
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
## func get_direction_at_point(point_id: [int]) -> [int]
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
## func get_cost_at_point(point_id: [int]) -> [float]
________
Returns the cost of the shortest path from this point to the target.

If there is no path, the cost is [`INF`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants).

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
## func recalculate(origin: [Variant], optional_params: [Dictionary] (opt)) -> [int]
________
Recalculates cost map and direction map information for each point,
overriding previous results.

This is the central function of the library, the one that actually uses
Dijkstra's algorithm.

### Parameters
- `origin` : ID of the origin point, or array of IDs (preferably
[`Int32Array`](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html)).
- `optional_params: `[`Dictionary`](https://docs.godotengine.org/en/stable/classes/class_dictionary.html) : Specifies optional arguments. 

  Valid arguments are :

  - `"input_is_destination": `[`bool`](https://docs.godotengine.org/en/stable/classes/class_bool.html) (default : [`true`](https://docs.godotengine.org/en/stable/classes/class_bool.html)) : 

    Wether or not the `origin` points are seen as destination.
  - `"maximum_cost": `[`float`](https://docs.godotengine.org/en/stable/classes/class_float.html)
(default : [`INF`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants)) : 

    Specifies maximum cost. Once all shortest paths no longer than
maximum cost are found, algorithm terminates. All points with cost
bigger than this are treated as inaccessible.
  - `"initial_costs": `[`float`](https://docs.godotengine.org/en/stable/classes/class_float.html) [`Array`](https://docs.godotengine.org/en/stable/classes/class_array.html) (default : empty) : 

    Specifies initial costs for given origins. Values are paired with
corresponding indices in the origin argument. Every unspecified
cost is defaulted to `0.0`. 

    Can be used to weigh the origins with a preference.
  - `"terrain_weights": `[`Dictionary`](https://docs.godotengine.org/en/stable/classes/class_dictionary.html) (default : empty) : 

    Specifies weights of terrain types. Keys are terrain type IDs and
values are floats. Unspecified terrains will have
[infinite](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants) weight. 

    Note that `-1` correspond to the default terrain (which have a
weight of `1.0`), and will thus be ignored if it appears in the
keys.
  - `"termination_points": `[`int`](https://docs.godotengine.org/en/stable/classes/class_int.html) OR [`int`](https://docs.godotengine.org/en/stable/classes/class_int.html) [`Array`](https://docs.godotengine.org/en/stable/classes/class_array.html) (default : empty) : 

    A set of points that stop the computation if they are reached by
the algorithm.
Note that keys of incorrect types are ignored with a warning.
### Errors
[`FAILED`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enum-globalscope-error) is returned if :

- One of the keys in `optional_params` is invalid.
- `origin` is neither an [`int`](https://docs.godotengine.org/en/stable/classes/class_int.html), a [`PoolIntArray`](https://docs.godotengine.org/en/stable/classes/class_poolintarray.html) or a [`Array`](https://docs.godotengine.org/en/stable/classes/class_array.html).
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
## func get_direction_at_points(points: [PoolIntArray]) -> [PoolIntArray]
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
## func get_cost_at_points(points: [PoolIntArray]) -> [PoolRealArray]
________
For each point in the given array, returns the cost of the shortest
path from this point to the target.

If there is no path from a point to the target, the cost is
[`INF`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants).

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
## func get_cost_map() -> [Dictionary]
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
## func get_direction_map() -> [Dictionary]
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
## func get_all_points_with_cost_between(min_cost: [float], max_cost: [float]) -> [PoolIntArray]
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
## func get_shortest_path_from_point(point_id: [int]) -> [PoolIntArray]
________
Returns an [array] of points describing the shortest path from a
starting point.

If the starting point is a target or is inaccessible, the [array] will
be empty.

#### Note
The starting point itself is not included.

## func add_square_grid(bounds: [Variant], terrain_type: [int] (opt), orthogonal_cost: [float] (opt), diagonal_cost: [float] (opt)) -> [Dictionary]
________
Adds a square grid of connected points.

### Parameters
- `bounds` : Dimensions of the grid. At the moment, only [`Rect2`](https://docs.godotengine.org/en/stable/classes/class_rect2.html) is
supported.
- `terrain_type` (default : `-1`) : Terrain to use for all points of
the grid.
- `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal
connections (up, down, right and left). 

  If `orthogonal_cost` is [`INF`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants) or [`NAN`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants), orthogonal
connections are disabled.
- `diagonal_cost` (default : [`INF`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants)) : specifies cost of diagonal
connections. 

  If `diagonal_cost` is [`INF`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants) or [`NAN`](https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants), diagonal connections
are disabled.
### Returns
This function returns a Dictionary where keys are coordinates of points
([`Vector2`](https://docs.godotengine.org/en/stable/classes/class_vector2.html)) and values are their corresponding point IDs.

## func add_hexagonal_grid(bounds: [Variant], terrain_type: [int] (opt), weight: [float] (opt)) -> [Dictionary]
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

[Dictionary]: https://docs.godotengine.org/en/stable/classes/class_dictionary.html
[PoolIntArray]: https://docs.godotengine.org/en/stable/classes/class_poolintarray.html
[PoolRealArray]: https://docs.godotengine.org/en/stable/classes/class_poolrealarray.html
[Variant]: https://docs.godotengine.org/en/stable/classes/class_variant.html
[array]: https://docs.godotengine.org/en/stable/classes/class_poolintarray.html
[bool]: https://docs.godotengine.org/en/stable/classes/class_bool.html
[float]: https://docs.godotengine.org/en/stable/classes/class_float.html
[int]: https://docs.godotengine.org/en/stable/classes/class_int.html