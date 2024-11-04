#!/bin/bash

#
# In order to submit this job to Slurm, run the command below from the terminal.
#   sbatch submit.sh

#SBATCH --time=1:00:00
#SBATCH --partition=batch
#SBATCH --nodes=1
#SBATCH --cpus-per-task=48
#SBATCH --mem=120G
#SBATCH --constraint=48core
#SBATCH --job-name prob303 
#SBATCH --output prob303-%j.out
#SBATCH --error prob303-%j.err

#-------- End of Slurm commands -------------------------

module load rust 

cargo run --release 
