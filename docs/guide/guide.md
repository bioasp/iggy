# Iggy - User Guide (version 2.2.1-dev)

Sven Thiele

## What are `iggy` and `optgraph`

`iggy` and `optgraph` are tools for consistency based analysis of influence graphs and
 observed systems behavior (signed changes between two measured states).
For many (biological) systems are knowledge bases available that describe the interaction
 of its components in terms of causal networks, boolean networks and influence graphs
  where edges indicate either positive or negative effect of one node upon another.

`iggy` implements methods to check the consistency of large-scale data sets and provides explanations
for inconsistencies.
In practice, this is used to identify unreliable data or to indicate missing reactions.
 Further, `iggy` addresses the problem of repairing networks and corresponding yet often discrepant
  measurements in order to re-establish their mutual consistency and predict unobserved variations
  even under inconsistency.

`optgraph` confronts interaction graph models with observed systems behavior from multiple experiments.
`optgraph` computes networks fitting the observation data by removing (or adding) a minimal number of edges in the given network.

## Downloads

- You can find the precompiled binaries for 64bit linux and macOS on the [release page](https://github.com/bioasp/iggy/releases/latest).

- Sample data [demo_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)

## Compile yourself

Clone the git repository:

```sh
git clone https://github.com/bioasp/iggy.git
cargo build --release
```

The executables can be found under `./target/release/`

## Input Model + Data

`iggy` and `optgraph` work with two kinds of data.
The first is representing an interaction graph model.
The second is the experimental data, representing experimental condition and observed behavior.

### Model

The model is represented as file in complex interaction format `CIF` as shown below.
Lines in the `CIF` file specify a interaction between (multiple) source nodes and one target node.

```txt
shp2                        ->  grb2_sos
!mtor_inhibitor             ->  mtor
?jak2_p                     ->  stat5ab_py
!ras_gap & grb2_sos         ->  pi3k
akt & erk & mtor & pi3k     ->  mtorc1
gab1_bras_py                ->  ras_gap
gab1_ps & jak2_p & pi3k     ->  gab1_bras_py
```

In our influence graph models, we have simple interactions like:
in Line 1 for `shp2` *increases* `grb2_sos`
and in Line 2 the `!` indicates that `mtor_inhibitor` tends to *decrease* `mtor`.
In Line 3 the `?` indicates an unknown influence of `jak2_p` on `stat5ab_py`.
Complex interactions can be composed with the `&` operator to model a combined influence of multiple sources on a target.
In Line 4 a decrease in `ras_gap` with an increase in `grb2_sos` tend to increase `pi3k`.

### Experimental data

The experimental data is given in the file format shown below.
Nodes which are perturbed in the experimental condition are denoted as `input`.
The first line of the example below states that `depor` has been perturbed in the experiment.
This means `depor` has been under the control of the experimentalist and its behavior must therefore not be explained.
The behavior of a node can be either `+`, `-`, `0`, `NotPlus`, `NotMinus`.
Line 2 states that an *increase* (`+`) was observed in `depor`,
 as it is declared an `input` this behavior has been caused by the experimentalist.
Line 3 states that `stat5ab_py` has *decreased* (`-`) and
line 4 states that `ras` has *not changed* (`0`).
Line 5 states that an *uncertain decrease* (`NotPlus`) has been observed in `plcg` and
line 6 states that an *uncertain increase* (`NotMinus`) has been observed in `mtorc1`.
Line 7 states that `akt` is initially on the minimum level, this means it cannot further decrease, and
line 8 states that `grb2_sos` is initially on the maximum level, this means it cannot further increase.

```txt
depor         = input
depor         = +
stat5ab_py    = -
ras_gap       = 0
jak2_p        = NotPlus
mtorc1        = NotMinus
akt           = MIN
pi3k          = MAX
```

### Consistency notions

The Iggy tools implement different constraints that inform the consistency notion under which the analysis are perform. In other words, what is considered a consistent behavior of a system. The defaults are:

- All observed changes must be explained by a predecessor.

    This basic constraint is always turned on.

- All observed changes must be explained by an input.

    You can turn this off with the flag `--founded-constraints-off`.

- 0-change must be explained.

   You can turn this constraint off with the flag `--fwd-propagation-off`.

Additional you can turn on the following constraint:

- An elementary path from an input must exist to explain changes.
   You can turn this constraint on with the flag `--elempath`.

With the flag `--depmat` you can turn on a consistency notion that is used for the *dependency matrix*. This notion includes the *elementary path* constraint.

For more information on the consistency notion see:

- [Extended notions of sign consistency to relate experimental data to signaling and regulatory network topologies](http://dx.doi.org/10.1186/s12859-015-0733-7),
Sven Thiele, Luca Cerone, Julio Saez-Rodriguez, Anne Siegel, Carito Guziołowski, and Steffen Klamt, *BMC Bioinformatics*, 16(345), 2015.

- [Sign consistency methods to reason over the transitional behavior of dynamic systems](https://sthiele.github.io/scm/)

For more information on the dependency matrix see:

- [A methodology for the structural and functional analysis of signaling and regulatory networks](doi:http://dx.doi.org/10.1186/1471-2105-7-56), Klamt S, Saez-Rodriguez J, Lindquist J, Simeoni L, Gilles E., BMC Bioinforma. 2006; 7(1):56.

## Iggy

`iggy` performs consistency checks for an interaction model and a data profile.
It computes explanations (minimal inconsistent cores *mics*) for inconsistencies
 and suggests repairs for model and data.

The *mics* are connected parts of the model and indicate unreliable data or missing reactions.
The repairs re-establish the mutual consistency between model and data, and enable predictions of unobserved behavior even under inconsistency.

The typical usage of `iggy` is:

```sh
iggy -n network.cif -o observation.obs -l 10 -p
```

For more options, you can ask for help as follows:

```txt
> iggy -h
iggy 2.2.1-dev
Sven Thiele <sthiele78@gmail.com>
Iggy confronts interaction graph models with observations of (signed) changes between two measured
states (including uncertain observations). Iggy discovers inconsistencies in networks or data,
applies minimal repairs, and predicts the behavior for the unmeasured species. It distinguishes
strong predictions (e.g. increase in a node) and weak predictions (e.g., the value of a node
increases or remains unchanged)

USAGE:
    iggy [OPTIONS] --network <FILE>

OPTIONS:
    -a, --auto-inputs                Declare nodes with indegree 0 as inputs
        --depmat                     Combine multiple states, a change must be explained by an
                                     elementary path from an input
        --elempath                   Every change must be explained by an elementary path from an
                                     input
        --founded-constraints-off    Disable foundedness constraints
        --fwd-propagation-off        Disable forward propagation constraints
    -h, --help                       Print help information
        --json                       Print JSON output
    -l, --show-labelings <N>         Show N labelings, default is OFF, 0=all
        --mics                       Compute minimal inconsistent cores
    -n, --network <FILE>             Influence graph in CIF format
    -o, --observations <FILE>        Observations in bioquali format
    -p, --show-predictions           Show predictions
        --scenfit                    Compute scenfit of the data, default is mcos
    -V, --version                    Print version information
```

### Compute minimal correction sets (mcos) or *scenfit* and predictions under inconsistency

`iggy` implements two measures for inconsistency *minimal correction sets (mcos)* and *scenfit*. While *mcos* measures the minimal number of observations that cannot be explained, *scenfit* measures a minimal number of changes to the model to re-establish the mutual consistency between model and data.
The default in iggy is *mcos* but *scenfit* can be used with the option `--scenfit`.

With the option `--show-labelings, -l N` `iggy` computes at most `N` such labelings and repairs that are consistent.

With the flag `--show-predictions, -p` `iggy` computes predictions under inconsistencies.
More precisely the behaviors of the system that are invariant also under the minimal repairs.

`iggy` presents the results of its analysis as text output.
The output of `iggy` can be redirected into a file using the `>` operator.
For example to write the results shown below into the file `myfile.txt` type:

```sh
iggy -n network.cif -o observations.obs -l 10 -p > myfile.txt
```

In the following we will dissect the output generated by `iggy`.
The first lines of the output state the constraints that have been used to analyze network and data.
For our example, it is the default setting with the following constraints.
For a deeper understanding of these constraints see [sthiele15].

```txt
## Settings

- All observed changes must be explained by a predecessor.
- 0-change must be explained.
- All observed changes must be explained by an input.
```

Next, follow some statistics on the input data.
The *network statistics* tells us that the influence graph model given as `network.cif`
consists of `18` species nodes and `4` complex nodes,
with `19` edges with activating influence
and `6` edges with inhibiting influence
and `1` edge with `Unknown`  influence.

```txt
Network file: network.cif

## Network statistics

- OR nodes (species): 18
- AND nodes (complex regulation): 4
- Activations: 19
- Inhibitions: 6
- Unknowns:    1

```

The following *observations statistics* tells us that the experimental data given as `observation.obs` consist of `14` observations from which all are nodes of the model. This leaves `4` nodes of the model unobserved. Further there are `0` observations of species that are not in the model.
The experimental conditions has `2` perturbations marked as `input` nodes,
and `1` node were observed with a *minimum level* `MIN` (resp. *maximum level* `MAX`).
From the 14 observations `4` nodes were observed as increased `+`,
`1` node *decreased* (`-`),
`7` nodes did *not change* (`0`),
`1` node were observed with an *uncertain decrease* (`NotPlus`),
`1` node were observed with an *uncertain increase* (`NotMinus`).

```txt
Observation file: observations.obs

## Observations statistics

- Observed model nodes:   14
- Unobserved model nodes: 4
- Observed not in model:  0
- Inputs:                 2
- MIN:                    1
- MAX:                    1
- Observations:           14
  - +:                    4
  - -:                    1
  - 0:                    7
  - notPlus:              1
  - notMinus:             1

```

Then follow the results of the consistency analysis.
Network and data are inconsistent
and the size of a *minimal correction set* (`mcos`) is `2`.
This means that at least `2` influences need to be added to restore consistency.
For a deeper understanding of mcos see [samaga13].
Further, the output contains at most `10` consistent labeling including correction set.
This is because we choose to set the flag `--show_labelings 10`.
In our example we have `2` possible labelings.
Each labeling represents a consistent behavior of the model (given mcos the corrections).
`Labeling 1`,
tells it is possible that
`mek1` *increases* (`+`),
`shp2_ph` and `mtorc` do *not change* (`0`) and that
`stat5ab_py` *decrease* (`-`).
The `Repairs` section tells us that this is a consistent behavior if `mtor` would receive a increasing influence and `socs1` would receive a decreasing influence,
which is currently not included in the model.
`Labeling 2`, represents an alternative behavior,
 here `mtorc1` does *increases* (`+`).
Please note that in this example both labelings are consistent under the same correction set.
In another example more than one minimal correction set could exists.

```txt
## Consistency results

scenfit: 2

- Labeling 1:
  mtorc1 = 0
  ras_gap = 0
  shp2 = 0
  gab1_bras_py = 0
  jak2_p = 0
  mek1 = +
  erk = +
  brb2 = 0
  akt = 0
  stat5ab_py = -
  brb = -
  gab1_ps = +
  grb2_sos = 0
  socs1 = 0
  pi3k = 0
  mtor = 0
  mtor_inhibitor = 0
  depor = +

  Repair set:
  - new increasing influence on mtor
  - new decreasing influence on socs1

- Labeling 2:
  mtorc1 = +
  ras_gap = 0
  shp2 = 0
  gab1_bras_py = 0
  jak2_p = 0
  mek1 = +
  erk = +
  brb2 = -
  akt = 0
  stat5ab_py = -
  brb = -
  gab1_ps = +
  grb2_sos = 0
  socs1 = 0
  pi3k = 0
  mtor = 0
  mtor_inhibitor = 0
  depor = +
  
  Repair set:
  - new increasing influence on mtor
  - new decreasing influence on socs1
```

Finally, the prediction results are listed.
A prediction is a statement that hold under all labeling under all minimal repairs.
For a formal definition of predictions see [sthiele15].
Here the predictions are that
`gab1_ps` *always increases* (`+`),
`stat5ab_py` *always decreases* (`-`),
`shp2` always stays *unchanged* (`0`),
`mtorc1` *never decreases* (`NotMinus`), and
`brb2` always stays *never increases* (`NotPlus`),

```txt
## Predictions

mek1 = +
erk = +
gab1_ps = +
depor = +
stat5ab_py = -
ras_gap = 0
shp2 = 0
gab1_bras_py = 0
jak2_p = 0
akt = 0
grb2_sos = 0
socs1 = 0
pi3k = 0
mtor = 0
mtor_inhibitor = 0
brb2 = notPlus
mtorc1 = notMinus
brb = CHANGE

## Prediction statistics

- predicted +        : 4
- predicted -        : 1
- predicted 0        : 10
- predicted notPlus  : 1
- predicted notMinus : 1
- predicted CHANGE   : 1
```

For more information on minimal correction sets *mcos* see:

- [Detecting and Removing  Inconsistencies between Experimental Data and Signaling Network Topologies Using Integer Linear Programming on Interaction Graphs.](doi:http://dx.doi.org/10.1371/journal.pcbi.1003204)
Melas IN, Samaga R, Alexopoulos LG, Klamt S. , PLoS Comput Biol. 2013; 9(9):1003204.

### Compute minimal inconsistent cores `--mics`

Iggy computes minimal inconsistent cores *mics* for inconsistent model and data.
The *mics* are connected parts of the model and indicate unreliable data or missing reactions.
To compute the minimal inconsistent cores use the flag `--mics` as follows:

```sh
iggy -n data/Yeast/yeast_guelzim.cif  -o data/Yeast/yeast_snf2.obs --mics
```

```txt
# Iggy Report

## Settings

- All observed changes must be explained by a predecessor.
- 0-change must be explained.
- All observed changes must be explained by an input.

Network file: data/Yeast/yeast_guelzim.cif

## Network statistics

- OR nodes (species): 477
- AND nodes (complex regulation): 0
- Activations: 665
- Inhibitions: 270
- Unknowns:    0

Observation file: data/Yeast/yeast_snf2.obs

## Observations statistics

- Observed model nodes:   89
- Unobserved model nodes: 388
- Observed not in model:  485
- Inputs:                 0
- MIN:                    0
- MAX:                    0
- Observations:           574
  - +:                    376
  - -:                    198
  - 0:                    0
  - notPlus:              0
  - notMinus:             0

## Consistency results

mcos: 530

- mic 1:
  YGR108W YJL159W 
- mic 2:
  YMR307W YIL013C 
- mic 3:
  YNL241C YLR109W 
- mic 4:
  YAL063C YER065C 
- mic 5:
  YNL009W YBR159W 
- mic 6:
  YMR186W YOL006C 
- mic 7:
  YDR224C YGR108W YAL040C 
- mic 8:
  YGR108W YPR119W 
- mic 9:
  YDR224C YKL185W YLR131C YPL256C YJL194W YLR079W YNL210W YOR159C YAL040C YDR522C YJL106W YDL127W YDR523C YHR053C YGR044C YIL072W YLR286C YHL022C YMR133W YJR094C YHR055C YDL179W YNL327W YHR014W 
- mic 10:
  YKL185W YLR131C YPL256C YJL194W YLR079W YNL210W YOR159C YDR522C YJL159W YJL106W YDL127W YDR523C YHR053C YGR044C YIL072W YLR286C YHL022C YMR133W YJR094C YHR055C YDL179W YNL327W YHR014W 
- mic 11:
  YKL185W YLR131C YPL256C YJL194W YLR079W YNL210W YOR159C YDR522C YJL106W YPR119W YDL127W YDR523C YHR053C YGR044C YIL072W YLR286C YHL022C YMR133W YJR094C YHR055C YDL179W YNL327W YHR014W 
- mic 12:
  YDR007W YPL187W YLR452C YAL038W YPL256C YHR084W YNL210W YOR159C YCL030C YMR199W STA3 YDR522C YCR018C YHR174W YNL216W YJL106W STA1 YCL067C YBR083W YGL089C YJL157C STA2 YIR019C YCR012W YLR403W YCL066W YDR523C YOR212W YOL086C YIL099W YKL209C YGR044C YIL072W YHL022C YMR133W YDR103W YKL178C YJR094C YGL008C YCL027W YIL015W YDR461W YOR077W YGR254W YFL026W YNL145W YHR014W YOL006C YJR004C YLR113W 
- mic 13:
  YMR021C YGL043W YNL314W YCR093W YML010W YIR023W YBR112C YDL106C YOR290C YOL067C YGR288W YHL027W YCR065W YER040W YCR097W YOL051W YFL031W YLR451W YLR014C YGL166W YHR119W YBR049C YMR070W YKR206W YMR042W YBL021C YDR216W YDL170W YBL093C YPL082C YKR099W YDR034C YDR176W YBR297W YGL237C YGL073W YBR279W YJR060W YCR084C YMR043W YDR123C YOL108C YBR289W YER169W YPL075W YOR363C YDR421W YIL101C YJL176C YDR392W YGL013C YDR043C YNR052C YML007W YOL116W YGL209W YOR344C YLR098C YER161C YMR037C YKL038W YEL009C YGL254W YKL015W YML099C YOR140W YHL025W YFR034C YDR448W YHR152W YGL255W YDR423C YDL056W YGL025C YKL062W YOR358W YKL032C YOR230W YER108C 
```

For more information on minimal inconsistent cores see:

- [Detecting Inconsistencies in Large Biological Networks with Answer Set Programming](http://dx.doi.org/10.1017/S1471068410000554),
Martin Gebser, Torsten Schaub, Sven Thiele, and Philippe Veber,
*Theory and Practice of Logic Programming*, 11(2-3), pages 323-360, 2011.

## Optgraph

`optgraph` confronts interaction graph models with observed systems behavior from multiple experiments.
`optgraph` computes networks fitting the observation data by removing (or adding) a minimal number of edges in the given network.

Typical usage is:

```sh
optgraph -n network.cif -o observations_dir/ --show-repairs 10
```

For more options, you can ask for help as follows:

```txt
> optgraph -h
optgraph 2.2.1-dev
Sven Thiele <sthiele78@gmail.com>
Optgraph confronts interaction graph models with observations of (signed) changes between two
measured states. Opt-graph computes networks fitting the observation data by removing (or adding) a
minimal number of edges in the given network

USAGE:
    optgraph [OPTIONS] --network <FILE> --observations <DIR>

OPTIONS:
    -a, --auto-inputs                  Declare nodes with indegree 0 as inputs
        --depmat                       Combine multiple states, a change must be explained by an
                                       elementary path from an input
        --elempath                     Every change must be explained by an elementary path from an
                                       input
        --founded-constraints-off      Disable foundedness constraints
        --fwd-propagation-off          Disable forward propagation constraints
    -h, --help                         Print help information
        --json                         Print JSON output
    -m, --repair-mode <REPAIR_MODE>    REPAIR_MODE: remove = remove edges (default), optgraph = add
                                       + remove edges, flip = flip direction of edges
    -n, --network <FILE>               Influence graph in CIF format
    -o, --observations <DIR>           Directory of observations in bioquali format
    -r, --show-repairs <N>             Show N repairs, default is OFF, 0=all
    -V, --version                      Print version information
```

### Example

```sh
optgraph -n in_silico_HEK293/v1_comp_BN.cif -o in_silico_HEK293/prior_data --depmat  -r 0 -m optgraph
```

```txt
# Optgraph Report

## Settings

- Dependency matrix combines multiple states.
- An elementary path from an input must exist to explain changes.

Network file: data/in_silico_HEK293/v1_comp_BN.cif

## Network statistics

- OR nodes (species): 23
- AND nodes (complex regulation): 12
- Activations: 47
- Inhibitions: 12
- Unknowns:    0

Observation files:
- data/in_silico_HEK293/prior_data/first_response_mek1_up.txt
- data/in_silico_HEK293/prior_data/first_response_pi3k_mek1_down.txt
- data/in_silico_HEK293/prior_data/first_response_pi3k_down.txt

## Consistency results


The network and data can reach a scenfit of 0.

- Repair set 1: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: !gab1_bras_py -> gab1_ps 

- Repair set 2: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: !ras_gap -> gab1_ps 

- Repair set 3: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: mtorc1 -> gab1_ps 

- Repair set 4: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: akt -> gab1_ps 

- Repair set 5: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: !grb2_sos -> gab1_ps 

- Repair set 6: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: erk -> gab1_ps 

- Repair set 7: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: mtorc2 -> gab1_ps 

- Repair set 8: 
  - remove edge: mek1 -> stat5ab_py 
  - remove edge: !mek1 -> shp2 
  - add edge: mek1 -> gab1_ps 

```

For more information on *OptGraph* see:

- [Designing optimal experiments to discriminate interaction graph models](https://doi.org/10.1109/TCBB.2018.2812184), Sven Thiele, Sandra Heise, Wiebke Hessenkemper, Hannes Bongartz, Melissa Fensky, Fred Schaper, Steffen Klamt, *IEEE/ACM Trans. Comput. Biol. Bioinform*, 16(3), 2019.

[samaga13]: https://dx.doi.org/10.1371/journal.pcbi.1003204
[sthiele15]: https://dx.doi.org/10.1186%2Fs12859-015-0733-7
