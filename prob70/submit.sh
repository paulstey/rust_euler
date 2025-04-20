#!/bin/bash

#
# In order to submit this job to Slurm, run the command below from the terminal.
# `sbatch submit.sh`

#SBATCH --time=4:00:00
#SBATCH --partition=batch
#SBATCH --nodes=1
#SBATCH --constraint=192core
#SBATCH --cpus-per-task=102
#SBATCH --mem=64G
#SBATCH --job-name prob70 
#SBATCH --output prob70-%j.out
#SBATCH --error prob70-%j.err

#-------- End of Slurm commands -------------------------

cargo run --release
