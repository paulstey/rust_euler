#!/bin/bash

#
# In order to submit this job to Slurm, run the command below from the terminal.
# `sbatch submit.sh`

#SBATCH --time=6:00:00
#SBATCH --partition=batch
#SBATCH --nodes=1
#SBATCH --constraint=192core
#SBATCH --cpus-per-task=192
#SBATCH --mem=94G
#SBATCH --job-name prob70 
#SBATCH --output prob72-%j.out
#SBATCH --error prob72-%j.err

#-------- End of Slurm commands -------------------------

cargo run --release
