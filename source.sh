# Find important directories
sim_directory="$(dirname $BASH_SOURCE)/sim"
parent_dir="$(dirname $BASH_SOURCE)"

# Create venv
if [ ! -d "$(pwd)/$parent_dir/sim-env" ]; then
  echo "Generating Python venv..."
  python -m venv $(pwd)/$parent_dir/sim-env && source sim-env/bin/activate && pip install -r $(pwd)/$parent_dir/pyRequirements.txt
  echo "Done!"
fi

# Add to venv PYTHONPATH
echo "$(pwd)/$sim_directory/tools" > "$parent_dir/sim-env/lib/python3.8/site-packages/sim_tools.pth"
echo "$(pwd)/$sim_directory/src" > "$parent_dir/sim-env/lib/python3.8/site-packages/sim_src.pth"
# echo "$(pwd)/$sim_directory/src/simfrastructure" > "$parent_dir/sim-env/lib/python3.8/site-packages/sim_infrastructure.pth"

# Source python venv
source $parent_dir/sim-env/bin/activate
echo "Sourced python venv"

# Add tools to path
export PATH="$sim_directory/tools:$PATH"
echo "Added tools folder to path. Ready to use 'sim'"

# Fix output dirs
export PYTHONPYCACHEPREFIX="$parent_dir/output/python"
export CARGO_TARGET_DIR="output"
echo "Configured output directories"