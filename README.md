## example of progenitor cli code

I couldnt see how to use progenitor's cli feature. I am using this as a working repo to figure it out.

from the repo:

cli-gen.json.txt

> cargo +nightly build

# notes
- turned off build.rs because i dont understand the json file is required if using params.
- commands need names to become sub commands
- httpmock is not used yet
- removed required from the cli-gen.json example.. not sure what that param is for.
- added a second endpoint to better understand generation