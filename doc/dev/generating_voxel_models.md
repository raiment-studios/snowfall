# Generating Voxel Models

1. Clone the repo
2. Working on a voxel model...
    1. Open `model_generator` in the editor
    2. Choose the model you're working on or add a new one
    3. Run `just dev` to have the model_generator re-run on any edits, thus regenerating the models on file
3. Viewing the voxel model
    1. In a separate terminal window go the `voxel_viewer` directory
    2. Run `just run <name of model without the extension>`
    3. The viewer will automatically reload any changes to the model from step 2
