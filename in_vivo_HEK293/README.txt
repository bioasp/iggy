This directory contains the files used in the 'in vivo' study presented in 
"Designing optimal experiments to discriminate interaction graph models"

gold_full_BN.sif  - the gold standard Boolean Network (BN) in SIF format
gold_comp_BN.sif  - the compressed gold standard BN in SIF format
gold_comp_IG.sif  - the compressed IG of the gold standard in SIF format

To create the model candidates used in the 'in vivo study' call:

opt_graph.py gold_comp_BN.sif Wildtype_data --depmat --repair_mode=2 --show_repairs=0

