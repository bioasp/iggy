This folder contains the input data used for planning the experiments presented in
"Designing optimal experiments to discriminate interaction graph models".

gold_full_BN.sif  - the gold standard Boolean Network (BN) in SIF format
gold_full_BN.pdf  - image of the gold standard BN
gold_comp_BN.sif  - the compressed gold standard BN in SIF format
gold_comp_BN.bn   - the compressed gold standard BN in BN format (used for simulation via ODEfy)
gold_comp_IG.sif  - the compressed IG of the gold standard in SIF format
gold_comp_IG.pdf  - image of the compressed IG of the gold standard
v1_comp_BN.sif    - distorted version of the compressed gold standard BN
v1_comp_IG.sif    - distorted version of IG of the compressed gold standard BN


To create the model candidates used in the 'in silico study' call:

opt_graph.py v1_comp_BN.sif prior_data --depmat --repair_mode=2 --show_repairs=0
