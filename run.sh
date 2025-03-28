#!/bin/bash

#SBATCH --job-name=uniprot_nearest_neighbor
#SBATCH --output=log/uniprot_nearest_neighbor.out%j
#SBATCH --error=log/uniprot_nearest_neighbor.err%j
#SBATCH --time=12:00:00
#SBATCH --qos=huge-long
#SBATCH --nodes=1
#SBATCH --ntasks=16
#SBATCH --nodelist=cbcb16
#SBATCH --mem=256gb
#SBATCH --partition=cbcb
#SBATCH --account=cbcb

cargo run -- uniprot_arg.fasta 16