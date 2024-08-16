# Roguelike details
- Should we focus roguelike or roguelite? What would people unlock?
- Roguelike implies that the best progression is knowledge. What would people be learning if a big focus is randomness?

# Invariants

## Classes
- Classes have a pool of perks associated with that class. ~= ClassPerkPool
- Players are expected - but not forced - to pick an average of 2 different classes (up to 6 max) to cover ~85% of roles.

## Perks
Classes max out at level 6-8? So we need about ~9-12 perks per class to keep variety.
Players can expect to get ~2 elite skills by max level in a class.

## Weapons

## Ammo
There are two important data types to distinguish: *Ammo Type* and *Ammo Grade*. 
Type is short, medium, long, micromissile, shotgun, etc.
Grade is Armor-Piercing, Hollow-Point, Poison, etc.

# Random Per-Run

## Classes
Each run, each player's "Level Tree" is built for every class. This is the list of what perks they will get at each level.
The level tree should have about 1/3-2/3 of all the perks for that class, so each run is different.
The perks at levels ? and ? are Milestone Perks, with doubled/tripled(?) effect.

## Weapons
### Weapon Stats / Upgrading
Each weapon has a list of stats that can be possibly be upgraded (damage, mag size, bonus damage vs small enemies, etc)
Each run, each weapon selects 1/3 of its stats that the player is allowed to upgrade, and 1/4 of its stats to be upgraded by 1-2 levels by default. This means guns can fill slightly different roles between runs.

### Store
Weapons start hidden. At the start of the game, unhide 2 weapons associated with each class.
Every ? waves, unlock ? weapons associated with which each class.
Once all weapons for a class are unlocked, much less frequently unlock "super" weapons. These will either be specially-made weapons, or regular weapons with a large number of base upgrades and also a higher cap on upgrades.

## Enemies
Each run, for each enemy, an "evolution tree" is built out of the modifiers. This tree represents the only champion variants of that enemy that are allowed to spawn. The tree has 4 tiers: 
0th tier is just the basic enemy with 1 minor modifier
1st tier has 2 branches with a major and minor modifier each
2nd tier has only 1 branch and adds 1 major modifier onto the branch before it
3rd tier is an island with 3 major modifiers, prioritizing modifiers that aren't in the rest of the tree.

The higher tiers become more common as the game progresses.

# Misc
Information is something that benefits the whole team, so info-gathering abilities and potential shouldn't be given to classes, lest they fall behind other classes in terms of personal power. Info is maybe something that should be gathered by doing optional map objectives?
