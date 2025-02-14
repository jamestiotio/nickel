# test = 'pass'

# Example array functions. This code is illustrative: prefer using the array
# stdlib functions `std.array.map` and `std.array.fold_right` instead.
let my_array_lib = {
  map : forall a b. (a -> b) -> Array a -> Array b = fun f arr =>
      if arr == [] then
        []
      else
        let head = std.array.first arr in
        let tail = std.array.drop_first arr in
        [f head] @ map f tail,

  fold : forall a b. (a -> b -> b) -> Array a -> b -> b = fun f arr first =>
      if arr == [] then
        first
      else
        let head = std.array.first arr in
        let tail = std.array.drop_first arr in
        f head (fold f tail first),
}
in
# Compute `7!`
let l = my_array_lib.map (fun x => x + 1) [1, 2, 3, 4, 5, 6] in
my_array_lib.fold (fun x acc => x * acc) l 1
