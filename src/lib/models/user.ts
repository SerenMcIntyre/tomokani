import * as v from "valibot";

export const UserSchema = v.object({
  username: v.string(),
  level: v.number(),
  subscription: v.object({
    active: v.boolean(),
    max_level_granted: v.number(),
  }),
});
