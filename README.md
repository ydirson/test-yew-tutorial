# Explorations towards extending yew tutorial

This tree is a bit more than this: the original idea is that of a real
app (a helper-in-your-pocket-phone app for figurine wargame players,
centered around [OnePageRules](https://onepagerules.com/) for now,
dubbed *General's Familiar*), starting with precisely what the
official yew tutorial showcases: displaying a list of items fetched as
JSON from another website.

## Branches

* `master` follows the tutorial (with a few exceptions that seemed to
  make sense, will submit them as tutorial updates
* `opr` builds on the tutorial to manipulate real-life data that's a
  bit more complex than in original tutorial.  Tries to introduce new
  ideas one at a time, with the idea to enhance the tutorial with
  them.
* `opr-*` experiment with some yew component libraries to
  eventually/hopefully get to a decent-looking app, which is an IMHO
  important aspect that the current tutorial does not currently
  address

Since one of the ideas of this project is to gradually enrich the
tutorial, commits in those branches regularly get rewritten to fix
issues, get better descriptions, etc.  It is expected to stabilize the
`master` contents at some point in the not-too-far future.

## Important issues to solve first

* `opr-material-yew` exhibits erratic behavior of selection callback
* size of WASM executable is unreasonable

## Functionnality Roadmap

* setup CI so the results can be experienced with just a web browser
* get the selected units from each army displayed, with something like
  Drawer components on either sides of the screen
* ensure important list-level info is displayed, and not hidden by
  the drawer
* get help on keywords
* quick switch of selection (for small lists) by e.g. swiping the
  drawer up/down
* quick switch of selection for larger lists, by using e.g. an iconbar
  to jump to given unit type (use eg. unit acronyms for a first step)
* assign an icon / get a pic to customize iconbar

### specific to the *General's Familiar*

* ordering units, grouping identical ones
* extract meaningful army-level info for permanent display (aura
  available from units, (optionally) spells, ...)
* add some stats to unit state (damage, mana, activated, exhausted...)
  and a way to change them
* keep history of stat changes (to help check about forgotten stuff,
  and help battle reports)
* proximity communication to share stats update
* WarFleets support
