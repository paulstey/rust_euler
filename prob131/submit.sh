#!/bin/bash

#
# In order to submit this job to Slurm, run the command below from the terminal.
# `sbatch submit.sh`

#SBATCH --time=1:00:00
#SBATCH --partition=batch
#SBATCH --nodes=1
#SBATCH --constraint=48core
#SBATCH --cpus-per-task=48
#SBATCH --mem=36G
#SBATCH --job-name prob131
#SBATCH --output prob131-%j.out
#SBATCH --error prob131-%j.err

#-------- End of Slurm commands -------------------------

cargo run --release


date
