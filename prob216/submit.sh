#!/bin/bash

#
# In order to submit this job to Slurm, run the command below from the terminal.
#   sbatch submit.sh

#SBATCH --time=2:00:00
#SBATCH --partition=batch
#SBATCH --nodes=1
#SBATCH --constraint=192core
#SBATCH --cpus-per-task=192
#SBATCH --mem=24G
#SBATCH --job-name prob216 
#SBATCH --output prob216-%j.out
#SBATCH --error prob216-%j.err

#-------- End of Slurm commands -------------------------

module load rust 

cargo run --release
