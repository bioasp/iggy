# Changelog
All notable changes to this project will be documented in this file.

## Unreleased

## v1.4.3 - 2019, Mar 1
use pyasp 1.4.4

## v1.4.2 - 2018, Aug 16
### Fixed
- Fix in opt_graph: auto generated inputs are now used for all experiments
- Fix the description of repair mode 1 (removing edges)

### Changed
- iggy and opt_graph now display the auto generated inputs when using --autoinputs

## v1.4.1 - 2017, Jul 20
### Fixed
- Fix missing pyasp import.

## v1.4 - 2017, Mar 29
### Changed
- Use pyasp-1.4.3
  Pyasp 1.4.2 has been broken so we had make an emergency release an update to the new version 1.4.3.
  Older version of iggy wont install properly anymore.

## v1.2 - 2015, Jun 26
### Added
- Added level bound constraints.
  Now one can add information about the initial state of a node to the experiment profile.
  For example, the line
    node1=MIN
  states that node1 is initially at the minimum level (mostly that means absent).
  The level bound constraint prohibits any solutions that predict a further decrease in node1.
  Conversely, the line
    node2=MAX
  states that node2 is initially at the maximum level.
  The level bound constraint prohibits then any solution that predict a further increase in node2.
 
### Changed
- Use pyasp-1.4.1
- Port to python 3

## v0.5 - 2015, Feb 04
### Added
- This CHANGELOG file

### Fixed
- Fixed problem with wrongly computed numbers of unlabeled nodes and measured nodes which are not in the model
- Fixed problem with doubled soultions when enumerating colorings resp. repairs

## Undocumented versions
- 0.4
- 0.3
- 0.2
- 0.1dev

