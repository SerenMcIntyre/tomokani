import * as v from "valibot";

export const BaseObjectSchema = v.object({
    id: v.number(),
    object: v.string(),
});