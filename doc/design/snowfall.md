# Design Doc

## Coordinate spaces

-   Z-axis is up
-   world-space (32-bit float space)
-   voxel-space (signed 32-bit integer space)

The voxel at 0,0,0 fills the world space from 0,0,0 to 1,1,1.

A standard character height is 8 voxels high.

Voxels roughly equate to 0.25 meters in length. This equates to 4x the resolution of Minecraft, or 64x the number of voxels in a given volume (which has significant performance and scability implications).

The engine should eventually support variable size voxels (e.g. finer resolution blocks) for special blocks and objects. This is not yet supported.

## Tools

-   rust
-   deno
-   just
-   mprocs
-   markdown
-   marp
