This directory contains the files used in the 'in vivo' study presented in the paper
[*Designing optimal experiments to discriminate interaction graph models*](https://doi.org/10.1109/TCBB.2018.2812184).

`gold_full_BN.cif`  - the gold standard Boolean Network (BN) in CIF format

`gold_comp_BN.cif`  - the compressed gold standard BN in CIF format

`gold_comp_IG.sif`  - the compressed IG of the gold standard in SIF format

To create the model candidates used in the 'in vivo study' call:

    opt_graph -n gold_comp_BN.cif -o Wildtype_data --depmat --repair-mode optgraph -r 0

old version call:

    opt_graph.py gold_comp_BN.sif Wildtype_data --depmat --repair_mode=2 --show_repairs=0

