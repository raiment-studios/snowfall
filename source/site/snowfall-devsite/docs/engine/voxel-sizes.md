# Voxel Sizes

## Game element sizes

| Element    | Size                      |
| ---------- | ------------------------- |
| Voxel      | 0.1 meters                |
| Sector     | 1,000 voxels              |
| Town       | 10 x 10 sectors           |
| City-State | 100 x 100 sectors         |
| Region     | up to 2000 x 2000 sectors |

Note that some objects in the world may used non-standard, scaled voxel sizes. Voxels also may be textured to give the appearance of higher voxel resolution.

## Reference sizes

| Reference | Size             |
| --------- | ---------------- |
| person    | 8x8x17 voxels    |
| stadium   | 2000x1000 voxels |

## Total world size

Given the [[world]] description, Galthea itself is about 6 Regions high and 1 across: i.e. 12 million Sectors or 12 trillion voxels for just the ground plane. A _fully_ generated world is not expected in real scenarios, but the engine should aim to be able to fit an entirely generated world in 1 petabyte of storage. Assuming 75% compression, that's > 300 voxels in height on average.
